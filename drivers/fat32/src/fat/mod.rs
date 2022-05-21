use crate::{cluster_chain::Cluster, SECTOR_SIZE};
use alloc::{boxed::Box, vec::Vec};
use base::{error::FAT32_FS_DRIVER_MODULE_NUMBER, multi_owner::Owner};
use cache::Cache;
use device::Device;
use filesystem::VolumeTrait;
use process::{Mutex, ProcessTypes};

mod cache;

pub enum ClusterState {
    Free,
    Some(Cluster),
    End,
}

#[derive(Debug)]
enum FATError {
    Corrupt,
    OutOfSpace,
}

pub struct FAT<T: ProcessTypes + 'static> {
    cache: Cache<T>,

    first_sector: usize,

    sectors_per_cluster: u32,
    bytes_per_cluster: u32,

    first_data_sector: u32,

    next_free_cluster: u32,
}

impl<T: ProcessTypes + 'static> FAT<T> {
    pub fn new(
        drive: Owner<Box<dyn Device>, Mutex<Box<dyn Device>, T>>,
        bytes_per_sector: u16,
        sectors_per_cluster: u8,
        reserved_sector_count: u16,
        num_fats: u8,
        fat_size: u32,
        first_sector: usize,
    ) -> Owner<Self, Mutex<Self, T>> {
        Owner::new(FAT {
            cache: Cache::new(
                drive,
                bytes_per_sector,
                reserved_sector_count,
                num_fats,
                fat_size,
            ),
            first_sector,
            sectors_per_cluster: sectors_per_cluster as u32,
            bytes_per_cluster: (sectors_per_cluster as u32) * (bytes_per_sector as u32),
            first_data_sector: reserved_sector_count as u32 + ((num_fats as u32) * fat_size),
            next_free_cluster: 0xFFFFFFFF,
        })
    }

    pub fn bytes_per_cluster(&self) -> usize {
        self.bytes_per_cluster as usize
    }

    pub fn read_cluster(&self, cluster: u32, buffer: &mut [u8]) -> base::error::Result<()> {
        self.cache
            .drive()
            .lock(|drive| drive.read(self.cluster_to_sector(cluster), buffer))?;
        Ok(())
    }

    pub fn allocate_cluster(&mut self) -> base::error::Result<Cluster> {
        let cluster = self.find_next_free_cluster()?;
        self.cache.set_next_cluster(cluster, ClusterState::End)?;
        Ok(cluster)
    }

    pub fn get_cluster_chain(
        &mut self,
        first_cluster: Cluster,
    ) -> base::error::Result<Vec<Cluster>> {
        let mut cluster_chain = Vec::new();

        let mut cluster = first_cluster;
        loop {
            cluster_chain.push(cluster);
            cluster = match self.cache.get_next_cluster(cluster)? {
                ClusterState::Free => return Err(Box::new(FATError::Corrupt)),
                ClusterState::End => return Ok(cluster_chain),
                ClusterState::Some(next_cluster) => next_cluster,
            };
        }
    }

    pub fn set_next_cluster(
        &mut self,
        cluster: Cluster,
        next_cluster: ClusterState,
    ) -> base::error::Result<()> {
        self.cache.set_next_cluster(cluster, next_cluster)
    }

    fn find_next_free_cluster(&mut self) -> base::error::Result<Cluster> {
        if self.next_free_cluster == 0xFFFFFFFF {
            self.next_free_cluster = 2;
        }

        let top = ((self.cache.fat_size() * self.cache.bytes_per_sector()) / 4) as u32;

        while self.next_free_cluster < top {
            match self.cache.get_next_cluster(self.next_free_cluster)? {
                ClusterState::Free => return Ok(self.next_free_cluster),
                _ => self.next_free_cluster += 1,
            }
        }

        Err(Box::new(FATError::OutOfSpace))
    }

    fn cluster_to_sector(&self, cluster: u32) -> usize {
        (((cluster - 2) * self.sectors_per_cluster) + self.first_data_sector) as usize
    }
}

impl<T: ProcessTypes + 'static> VolumeTrait for FAT<T> {
    fn set_name(&mut self, new_name: alloc::string::String) -> base::error::Result<()> {
        self.cache.drive().lock(|drive| {
            // Load the BPB
            let mut bpb = Box::new([0u8; SECTOR_SIZE]);
            drive.read(self.first_sector, bpb.as_mut_slice())?;

            // Copy the new name
            let mut bytes = new_name.bytes();
            for i in 0x47..0x52 {
                match bytes.next() {
                    Some(byte) => bpb[i] = byte,
                    None => break,
                }
            }

            // Write the BPB
            drive.write(self.first_sector, bpb.as_mut_slice())?;

            Ok(())
        })
    }
}

impl base::error::Error for FATError {
    fn module_number(&self) -> i32 {
        FAT32_FS_DRIVER_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            FATError::Corrupt => base::error::Status::CorruptFilesystem,
            FATError::OutOfSpace => base::error::Status::OutOfResource,
        }
    }
}

impl core::fmt::Display for FATError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FATError::Corrupt => write!(f, "FAT is corrupted"),
            FATError::OutOfSpace => write!(f, "No more space in volume"),
        }
    }
}
