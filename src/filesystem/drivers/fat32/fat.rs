use crate::{device::DeviceBox, error, locks::Mutex};
use alloc::{sync::Arc, vec::Vec};

pub type FATBox = Arc<Mutex<FAT>>;

pub struct FAT {
    drive: DeviceBox,
    sectors_per_cluster: u32,
    bytes_per_cluster: usize,
    _num_fats: u32,
    _fat_size: u32,
    first_fat_sector: usize,
    first_data_sector: u32,
    bytes_per_sector: usize,
}

impl FAT {
    pub fn new(
        drive: DeviceBox,
        sectors_per_cluster: u8,
        reserved_sector_count: u16,
        num_fats: u8,
        fat_size: u32,
        bytes_per_sector: u16,
    ) -> Self {
        FAT {
            drive: drive,
            sectors_per_cluster: sectors_per_cluster as u32,
            bytes_per_cluster: (sectors_per_cluster as usize) * (bytes_per_sector as usize),
            first_fat_sector: reserved_sector_count as usize,
            _num_fats: num_fats as u32,
            _fat_size: fat_size as u32,
            bytes_per_sector: bytes_per_sector as usize,
            first_data_sector: reserved_sector_count as u32 + ((num_fats as u32) * fat_size),
        }
    }

    pub fn get_cluster_chain(&self, first_cluster: u32) -> error::Result<Vec<u32>> {
        let mut cluster_chain = Vec::new();

        let mut buffer = Vec::with_capacity(self.bytes_per_sector);
        unsafe { buffer.set_len(self.bytes_per_sector) };

        let mut last_sector = 0xFFFFFFFF;
        let mut cluster = first_cluster;
        let drive = self.drive.lock();
        loop {
            let sector = self.first_fat_sector + ((cluster as usize * 4) / self.bytes_per_sector);
            let offset = (cluster as usize * 4) % self.bytes_per_sector;

            if last_sector != sector {
                drive.read(sector, buffer.as_mut_slice())?;
                last_sector = sector;
            }

            cluster_chain.push(cluster & 0x0FFFFFFF);
            cluster = ((buffer[offset] as u32)
                | ((buffer[offset + 1] as u32) << 8)
                | ((buffer[offset + 2] as u32) << 16)
                | ((buffer[offset + 3] as u32) << 24))
                & 0x0FFFFFFF;

            if cluster == 0 || (cluster & 0x0FFFFFFF) >= 0x0FFFFFF8 {
                break;
            }
        }

        Ok(cluster_chain)
    }

    pub fn grow_cluster_chain(&self, first_cluster: u32, num_clusters: usize) -> error::Result<()> {
        // Locate end of cluster chain
        let mut buffer = Vec::with_capacity(self.bytes_per_sector);
        unsafe { buffer.set_len(self.bytes_per_sector) };

        let mut cluster = first_cluster;
        let mut last_sector = 0xFFFFFFFF;
        let mut original_count = 0;
        let mut drive = self.drive.lock();
        let (mut prev_sector, mut prev_offset) = loop {
            let sector = self.first_fat_sector + ((cluster as usize * 4) / self.bytes_per_sector);
            let offset = (cluster as usize * 4) % self.bytes_per_sector;

            if last_sector != sector {
                drive.read(sector, buffer.as_mut_slice())?;
                last_sector = sector;
            }

            cluster = ((buffer[offset] as u32)
                | ((buffer[offset + 1] as u32) << 8)
                | ((buffer[offset + 2] as u32) << 16)
                | ((buffer[offset + 3] as u32) << 24))
                & 0x0FFFFFFF;
            original_count += 1;

            if cluster == 0 {
                return Err(error::Status::IOError);
            }

            if (cluster & 0x0FFFFFFF) >= 0x0FFFFFF8 {
                break (sector, offset);
            }
        };

        // Allocate new clusters
        let mut prev_buffer = buffer;
        let mut buffer = Vec::with_capacity(self.bytes_per_sector);
        unsafe { buffer.set_len(self.bytes_per_sector) };

        let mut cluster: u32 = 2;
        last_sector = 0xFFFFFFFF;
        for _ in original_count..num_clusters {
            let (sector, offset) = loop {
                let sector =
                    self.first_fat_sector + ((cluster as usize * 4) / self.bytes_per_sector);
                let offset = (cluster as usize * 4) % self.bytes_per_sector;

                if last_sector != sector {
                    drive.read(sector, buffer.as_mut_slice())?;
                    last_sector = sector;
                }

                let new_cluster = ((buffer[offset] as u32)
                    | ((buffer[offset + 1] as u32) << 8)
                    | ((buffer[offset + 2] as u32) << 16)
                    | ((buffer[offset + 3] as u32) << 24))
                    & 0x0FFFFFFF;

                if new_cluster == 0 {
                    break (sector, offset);
                }

                cluster += 1;
            };

            prev_buffer[prev_offset + 0] = (cluster.wrapping_shr(0) & 0xFF) as u8;
            prev_buffer[prev_offset + 1] = (cluster.wrapping_shr(8) & 0xFF) as u8;
            prev_buffer[prev_offset + 2] = (cluster.wrapping_shr(16) & 0xFF) as u8;
            prev_buffer[prev_offset + 3] = (cluster.wrapping_shr(24) & 0xFF) as u8;

            drive.write(prev_sector, prev_buffer.as_slice())?;
            prev_buffer = buffer;
            prev_sector = sector;
            prev_offset = offset;

            buffer = Vec::with_capacity(self.bytes_per_sector);
            for byte in &prev_buffer {
                buffer.push(*byte);
            }

            cluster += 1;
        }

        prev_buffer[prev_offset + 0] = 0xFF;
        prev_buffer[prev_offset + 1] = 0xFF;
        prev_buffer[prev_offset + 2] = 0xFF;
        prev_buffer[prev_offset + 3] = 0x0F;
        drive.write(prev_sector, prev_buffer.as_slice())?;

        Ok(())
    }

    pub fn shrink_cluster_chain(
        &self,
        first_cluster: u32,
        num_clusters: usize,
    ) -> error::Result<()> {
        // Find new end of cluster
        let mut buffer = Vec::with_capacity(self.bytes_per_sector);
        unsafe { buffer.set_len(self.bytes_per_sector) };

        let mut last_sector = 0xFFFFFFFF;
        let mut cluster = first_cluster;
        let mut drive = self.drive.lock();
        for _ in 0..num_clusters {
            let sector = self.first_fat_sector + ((cluster as usize * 4) / self.bytes_per_sector);
            let offset = (cluster as usize * 4) % self.bytes_per_sector;

            if last_sector != sector {
                drive.read(sector, buffer.as_mut_slice())?;
                last_sector = sector;
            }

            cluster = ((buffer[offset] as u32)
                | ((buffer[offset + 1] as u32) << 8)
                | ((buffer[offset + 2] as u32) << 16)
                | ((buffer[offset + 3] as u32) << 24))
                & 0x0FFFFFFF;

            if cluster == 0 || (cluster & 0x0FFFFFFF) >= 0x0FFFFFF8 {
                return Err(error::Status::IOError);
            }
        }

        // Free remaining clusters
        let mut next_cluster = cluster;
        {
            let offset = (cluster as usize * 4) % self.bytes_per_sector;

            buffer[offset] = 0xFF;
            buffer[offset + 1] = 0xFF;
            buffer[offset + 2] = 0xFF;
            buffer[offset + 3] = 0x0F;
        }

        loop {
            let sector = self.first_fat_sector + ((cluster as usize * 4) / self.bytes_per_sector);
            let offset = (cluster as usize * 4) % self.bytes_per_sector;

            if last_sector != sector {
                drive.write(last_sector, buffer.as_slice())?;
                drive.read(sector, buffer.as_mut_slice())?;
                last_sector = sector;
            }

            next_cluster = ((buffer[offset] as u32)
                | ((buffer[offset + 1] as u32) << 8)
                | ((buffer[offset + 2] as u32) << 16)
                | ((buffer[offset + 3] as u32) << 24))
                & 0x0FFFFFFF;

            buffer[offset] = 0;
            buffer[offset + 1] = 0;
            buffer[offset + 2] = 0;
            buffer[offset + 3] = 0;

            if next_cluster == 0 || (next_cluster & 0x0FFFFFFF) >= 0x0FFFFFF8 {
                break;
            }
        }

        drive.write(last_sector, buffer.as_slice())?;

        Ok(())
    }

    pub fn read_cluster(&self, cluster: u32, buffer: &mut [u8]) -> error::Result<()> {
        self.drive
            .lock()
            .read(self.cluster_to_sector(cluster), buffer)
    }

    pub fn write_cluster(&self, cluster: u32, buffer: &[u8]) -> error::Result<()> {
        self.drive
            .lock()
            .write(self.cluster_to_sector(cluster), buffer)
    }

    pub fn bytes_per_cluster(&self) -> usize {
        self.bytes_per_cluster
    }

    fn cluster_to_sector(&self, cluster: u32) -> usize {
        (((cluster - 2) * self.sectors_per_cluster) + self.first_data_sector) as usize
    }
}
