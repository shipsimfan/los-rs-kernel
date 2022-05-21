use crate::{cluster_chain::ClusterChain, fat::FAT};
use alloc::{boxed::Box, vec::Vec};
use base::{error::FAT32_FS_DRIVER_MODULE_NUMBER, multi_owner::Owner};
use filesystem::FileTrait;
use process::{Mutex, ProcessTypes};

#[derive(Debug)]
enum FileError {
    OutOfRange,
}

pub struct File<T: ProcessTypes + 'static> {
    cluster_chain: ClusterChain,
    fat: Owner<FAT<T>, Mutex<FAT<T>, T>>,
    file_size: usize,
}

impl<T: ProcessTypes + 'static> File<T> {
    pub fn new(
        first_cluster: u32,
        file_size: usize,
        fat: &mut FAT<T>,
        fat_lock: Owner<FAT<T>, Mutex<FAT<T>, T>>,
    ) -> base::error::Result<Self> {
        let cluster_chain = ClusterChain::new(first_cluster, fat)?;

        Ok(File {
            cluster_chain,
            file_size,
            fat: fat_lock,
        })
    }
}

impl<T: ProcessTypes + 'static> FileTrait for File<T> {
    fn write(&mut self, offset: usize, buffer: &[u8]) -> base::error::Result<isize> {
        if buffer.len() == 0 {
            return Ok(0);
        }

        if offset + buffer.len() > self.file_size {
            return Err(Box::new(FileError::OutOfRange));
        }

        self.fat.lock(|fat| {
            let starting_cluster_index = offset / fat.bytes_per_cluster();
            let ending_cluster_index = (offset + buffer.len() - 1) / fat.bytes_per_cluster();

            if starting_cluster_index == ending_cluster_index {
                let mut write_buffer = Vec::with_capacity(fat.bytes_per_cluster());
                unsafe { write_buffer.set_len(fat.bytes_per_cluster()) };

                fat.read_cluster(
                    self.cluster_chain.get(starting_cluster_index).unwrap(),
                    write_buffer.as_mut_slice(),
                )?;

                let buffer_offset = offset - (starting_cluster_index * fat.bytes_per_cluster());
                for i in 0..buffer.len() {
                    write_buffer[i + buffer_offset] = buffer[i];
                }

                fat.write_cluster(
                    self.cluster_chain.get(starting_cluster_index).unwrap(),
                    write_buffer.as_slice(),
                )?;
            } else {
                // Write first cluster
                let mut write_buffer = Vec::with_capacity(fat.bytes_per_cluster());
                unsafe { write_buffer.set_len(fat.bytes_per_cluster()) };

                fat.read_cluster(
                    self.cluster_chain.get(starting_cluster_index).unwrap(),
                    write_buffer.as_mut_slice(),
                )?;

                let buffer_offset = offset - (starting_cluster_index * fat.bytes_per_cluster());
                for i in buffer_offset..fat.bytes_per_cluster() {
                    write_buffer[i] = buffer[i - buffer_offset];
                }

                fat.write_cluster(
                    self.cluster_chain.get(starting_cluster_index).unwrap(),
                    write_buffer.as_slice(),
                )?;

                // Write middle clusters
                let mut buffer_offset = fat.bytes_per_cluster() - buffer_offset;
                for cluster_index in starting_cluster_index + 1..ending_cluster_index {
                    fat.write_cluster(
                        self.cluster_chain.get(cluster_index).unwrap(),
                        &buffer[buffer_offset..buffer_offset + fat.bytes_per_cluster()],
                    )?;
                    buffer_offset += fat.bytes_per_cluster();
                }

                // Write final cluster
                fat.read_cluster(
                    self.cluster_chain.get(ending_cluster_index).unwrap(),
                    write_buffer.as_mut_slice(),
                )?;

                let mut j = 0;
                for i in buffer_offset..buffer.len() {
                    write_buffer[j] = buffer[i];
                    j += 1;
                }

                fat.write_cluster(
                    self.cluster_chain.get(starting_cluster_index).unwrap(),
                    write_buffer.as_slice(),
                )?;
            }

            fat.flush_buffer()?;

            Ok(buffer.len() as isize)
        })
    }

    fn read(&mut self, offset: usize, buffer: &mut [u8]) -> base::error::Result<isize> {
        if offset >= self.file_size {
            return Ok(-1);
        }

        // Get info from fat
        let bytes_per_cluster = self.fat.lock(|fat| fat.bytes_per_cluster());

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
        let end_cluster = if end_cluster > self.cluster_chain.len() {
            self.cluster_chain.len()
        } else {
            end_cluster
        };

        let num_clusters = end_cluster - start_cluster;
        let int_buffer_size = num_clusters * bytes_per_cluster;

        // Read clusters
        let mut int_buffer = Vec::with_capacity(int_buffer_size);
        unsafe { int_buffer.set_len(int_buffer_size) };
        {
            self.fat.lock(|fat| -> base::error::Result<()> {
                let mut index = 0;
                let mut current_cluster = 0;
                for cluster in self.cluster_chain.as_slice() {
                    if current_cluster < start_cluster {
                        current_cluster += 1;
                        continue;
                    } else if current_cluster >= end_cluster {
                        break;
                    }

                    fat.read_cluster(*cluster, &mut int_buffer[index..index + bytes_per_cluster])?;
                    index += bytes_per_cluster;
                    current_cluster += 1;
                }

                Ok(())
            })?;
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

    fn set_length(&mut self, new_length: usize) -> base::error::Result<()> {
        self.fat.lock(|fat| {
            let bytes_per_cluster = fat.bytes_per_cluster();

            let current_cluster_count =
                (self.file_size + bytes_per_cluster - 1) / bytes_per_cluster;
            let new_cluster_count = (new_length + bytes_per_cluster - 1) / bytes_per_cluster;

            if current_cluster_count > new_cluster_count {
                self.cluster_chain.shrink(new_cluster_count, fat)?;
            } else if current_cluster_count < new_cluster_count {
                self.cluster_chain.grow(new_cluster_count, fat)?;
            }

            self.file_size = new_length;

            Ok(())
        })
    }

    fn get_length(&self) -> usize {
        self.file_size
    }
}

impl base::error::Error for FileError {
    fn module_number(&self) -> i32 {
        FAT32_FS_DRIVER_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            FileError::OutOfRange => base::error::Status::OutOfRange,
        }
    }
}

impl core::fmt::Display for FileError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FileError::OutOfRange => write!(f, "Beyond end of file"),
        }
    }
}
