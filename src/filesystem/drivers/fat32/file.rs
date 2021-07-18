use alloc::vec::Vec;

use super::fat::FATBox;

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
    fn write(&mut self, _: usize, _: &[u8]) -> crate::error::Result {
        Err(crate::error::Status::NotSupported)
    }

    fn read(&mut self, offset: usize, buffer: &mut [u8]) -> Result<usize, crate::error::Status> {
        // Get info from fat
        let (bytes_per_cluster, cluster_chain) = {
            let fat = self.fat.lock();
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
            let ret = buffer.len();
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

            Ok(self.file_size - offset)
        }
    }

    fn set_length(&mut self, _: usize) -> crate::error::Result {
        Err(crate::error::Status::NotSupported)
    }

    fn get_length(&self) -> usize {
        self.file_size
    }
}
