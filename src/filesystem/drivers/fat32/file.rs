use super::fat::FATBox;
use crate::error;
use alloc::vec::Vec;

pub struct File {
    first_cluster: u32,
    file_size: usize,
    fat: FATBox,
}

impl File {
    pub fn new(first_cluster: u32, file_size: usize, fat: FATBox) -> Self {
        File {
            first_cluster: first_cluster,
            file_size: file_size,
            fat: fat,
        }
    }
}

impl crate::filesystem::File for File {
    fn write(&mut self, offset: usize, buffer: &[u8]) -> error::Result<isize> {
        if buffer.len() == 0 {
            return Ok(0);
        }

        if offset + buffer.len() > self.file_size {
            return Err(error::Status::OutOfRange);
        }

        let mut fat = self.fat.lock();

        let starting_cluster_index = offset / fat.bytes_per_cluster();
        let ending_cluster_index = (offset + buffer.len() - 1) / fat.bytes_per_cluster();

        let cluster_chain = fat.get_cluster_chain(self.first_cluster)?;

        if starting_cluster_index == ending_cluster_index {
            let mut write_buffer = Vec::with_capacity(fat.bytes_per_cluster());
            unsafe { write_buffer.set_len(fat.bytes_per_cluster()) };

            fat.read_cluster(
                cluster_chain[starting_cluster_index],
                write_buffer.as_mut_slice(),
            )?;

            let buffer_offset = offset - (starting_cluster_index * fat.bytes_per_cluster());
            for i in 0..buffer.len() {
                write_buffer[i + buffer_offset] = buffer[i];
            }

            fat.write_cluster(
                cluster_chain[starting_cluster_index],
                write_buffer.as_slice(),
            )?;
        } else {
            // Write first cluster
            let mut write_buffer = Vec::with_capacity(fat.bytes_per_cluster());
            unsafe { write_buffer.set_len(fat.bytes_per_cluster()) };

            fat.read_cluster(
                cluster_chain[starting_cluster_index],
                write_buffer.as_mut_slice(),
            )?;

            let buffer_offset = offset - (starting_cluster_index * fat.bytes_per_cluster());
            for i in buffer_offset..fat.bytes_per_cluster() {
                write_buffer[i] = buffer[i - buffer_offset];
            }

            fat.write_cluster(
                cluster_chain[starting_cluster_index],
                write_buffer.as_slice(),
            )?;

            // Write middle clusters
            let mut buffer_offset = fat.bytes_per_cluster() - buffer_offset;
            for cluster_index in starting_cluster_index + 1..ending_cluster_index {
                fat.write_cluster(
                    cluster_chain[cluster_index],
                    &buffer[buffer_offset..buffer_offset + fat.bytes_per_cluster()],
                )?;
                buffer_offset += fat.bytes_per_cluster();
            }

            // Write final cluster
            fat.read_cluster(
                cluster_chain[ending_cluster_index],
                write_buffer.as_mut_slice(),
            )?;

            let mut j = 0;
            for i in buffer_offset..buffer.len() {
                write_buffer[j] = buffer[i];
                j += 1;
            }

            fat.write_cluster(cluster_chain[ending_cluster_index], write_buffer.as_slice())?;
        }

        Ok(buffer.len() as isize)
    }

    fn read(&mut self, offset: usize, buffer: &mut [u8]) -> error::Result<isize> {
        if offset >= self.file_size {
            return Ok(-1);
        }

        // Get info from fat
        let (bytes_per_cluster, cluster_chain) = {
            let mut fat = self.fat.lock();
            (
                fat.bytes_per_cluster(),
                fat.get_cluster_chain(self.first_cluster)?,
            )
        };

        // Calculate true start and end
        let true_start_diff = offset % bytes_per_cluster;
        let true_start = offset - true_start_diff;

        let end = offset + buffer.len();
        let true_end_diff = end % bytes_per_cluster;
        let true_end = if true_end_diff == 0 {
            end
        } else {
            end - true_end_diff + bytes_per_cluster
        };

        let start_cluster = true_start / bytes_per_cluster;
        let end_cluster = true_end / bytes_per_cluster;
        let end_cluster = if end_cluster > cluster_chain.len() {
            cluster_chain.len()
        } else {
            end_cluster
        };

        let num_clusters = end_cluster - start_cluster;
        let int_buffer_size = num_clusters * bytes_per_cluster;

        // Read clusters
        let mut int_buffer = Vec::with_capacity(int_buffer_size);
        unsafe { int_buffer.set_len(int_buffer_size) };
        {
            let fat = self.fat.lock();
            let mut index = 0;
            let mut current_cluster = 0;
            for cluster in cluster_chain {
                if current_cluster < start_cluster {
                    current_cluster += 1;
                    continue;
                } else if current_cluster >= end_cluster {
                    break;
                }

                fat.read_cluster(cluster, &mut int_buffer[index..index + bytes_per_cluster])?;
                index += bytes_per_cluster;
                current_cluster += 1;
            }
        }

        // Copy buffer
        if end <= self.file_size {
            let mut i_buf_idx = true_start_diff;
            let ret = buffer.len() as isize;
            for byte in buffer {
                *byte = int_buffer[i_buf_idx];
                i_buf_idx += 1;
            }
            Ok(ret)
        } else {
            let mut idx = 0;
            let mut i_buf_idx = true_start_diff;
            for _ in offset..self.file_size {
                buffer[idx] = int_buffer[i_buf_idx];
                idx += 1;
                i_buf_idx += 1;
            }

            for i in idx..buffer.len() {
                buffer[i] = 0;
            }

            Ok((self.file_size - offset) as isize)
        }
    }

    fn set_length(&mut self, new_length: usize) -> error::Result<()> {
        let mut fat = self.fat.lock();
        let bytes_per_cluster = fat.bytes_per_cluster();

        let current_cluster_count = (self.file_size + bytes_per_cluster - 1) / bytes_per_cluster;
        let new_cluster_count = (new_length + bytes_per_cluster - 1) / bytes_per_cluster;

        if current_cluster_count > new_cluster_count {
            fat.shrink_cluster_chain(self.first_cluster, new_cluster_count)?;
        } else if current_cluster_count < new_cluster_count {
            fat.grow_cluster_chain(self.first_cluster, new_cluster_count)?;
        }

        self.file_size = new_length;

        Ok(())
    }

    fn get_length(&self) -> usize {
        self.file_size
    }
}
