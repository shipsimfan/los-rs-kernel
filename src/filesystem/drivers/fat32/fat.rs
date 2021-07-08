use crate::{device::DeviceBox, error, locks::Mutex};
use alloc::{sync::Arc, vec::Vec};

pub type FATBox = Arc<Mutex<FAT>>;

pub struct FAT {
    drive: DeviceBox,
    sectors_per_cluster: u32,
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
            first_fat_sector: reserved_sector_count as usize,
            _num_fats: num_fats as u32,
            _fat_size: fat_size as u32,
            bytes_per_sector: bytes_per_sector as usize,
            first_data_sector: reserved_sector_count as u32 + ((num_fats as u32) * fat_size),
        }
    }

    pub fn get_cluster_chain(&self, first_cluster: u32) -> Result<Vec<u32>, error::Status> {
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

    pub fn read_cluster(&self, cluster: u32, buffer: &mut [u8]) -> error::Result {
        self.drive
            .lock()
            .read(self.cluster_to_sector(cluster), buffer)
    }

    fn cluster_to_sector(&self, cluster: u32) -> usize {
        (((cluster - 2) * self.sectors_per_cluster) + self.first_data_sector) as usize
    }
}
