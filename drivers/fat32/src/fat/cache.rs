use super::ClusterState;
use crate::cluster_chain::Cluster;
use alloc::{boxed::Box, vec};
use base::multi_owner::Owner;
use device::Device;
use process::{Mutex, ProcessTypes};

enum BufferLocation {
    Replace,
    Found,
}

pub struct Cache<T: ProcessTypes + 'static> {
    drive: Owner<Box<dyn Device>, Mutex<Box<dyn Device>, T>>,

    bytes_per_sector: usize,

    first_fat_sector: usize,

    num_fats: usize,
    fat_size: usize,

    sectors: Box<[Option<FATSector>]>,
    next_access_count: usize,
}

#[derive(Clone)]
struct FATSector {
    index: usize,
    contents: Box<[u8]>,
    access_count: usize,
    modified: bool,
}

const CACHE_SIZE: usize = 16;

impl<T: ProcessTypes + 'static> Cache<T> {
    pub fn new(
        drive: Owner<Box<dyn Device>, Mutex<Box<dyn Device>, T>>,
        bytes_per_sector: u16,
        reserved_sector_count: u16,
        num_fats: u8,
        fat_size: u32,
    ) -> Self {
        Cache {
            drive,
            bytes_per_sector: bytes_per_sector as usize,
            first_fat_sector: reserved_sector_count as usize,
            num_fats: num_fats as usize,
            fat_size: fat_size as usize,
            sectors: vec![None; CACHE_SIZE].into_boxed_slice(),
            next_access_count: 0,
        }
    }

    pub fn drive(&self) -> &Owner<Box<dyn Device>, Mutex<Box<dyn Device>, T>> {
        &self.drive
    }

    pub fn fat_size(&self) -> usize {
        self.fat_size
    }

    pub fn bytes_per_sector(&self) -> usize {
        self.bytes_per_sector
    }

    pub fn flush_buffer(&mut self) -> base::error::Result<()> {
        for sector in self.sectors.iter_mut() {
            let sector = match sector {
                Some(sector) => sector,
                None => continue,
            };

            if !sector.modified {
                continue;
            }

            // Sector is modified, so write it
            for i in 0..self.num_fats {
                self.drive.lock(|drive| {
                    drive.write(
                        sector.index + self.first_fat_sector + self.fat_size * i,
                        sector.contents.as_ref(),
                    )
                })?;
            }

            sector.modified = false;
        }

        Ok(())
    }

    pub fn get_next_cluster(&mut self, cluster: Cluster) -> base::error::Result<ClusterState> {
        let sector_offset = (cluster as usize * 4) / self.bytes_per_sector;
        let offset = (cluster as usize * 4) % self.bytes_per_sector;

        let buffer = self.get_buffer(sector_offset)?;

        let next_cluster = ((buffer[offset] as u32)
            | ((buffer[offset + 1] as u32) << 8)
            | ((buffer[offset + 2] as u32) << 16)
            | ((buffer[offset + 3] as u32) << 24))
            & 0x0FFFFFFF;

        Ok(if next_cluster >= 0x0FFFFFF8 {
            ClusterState::End
        } else if next_cluster == 0 {
            ClusterState::Free
        } else {
            ClusterState::Some(next_cluster)
        })
    }

    pub fn set_next_cluster(
        &mut self,
        cluster: Cluster,
        next_cluster: ClusterState,
    ) -> base::error::Result<()> {
        let next_cluster = match next_cluster {
            ClusterState::Free => 0,
            ClusterState::Some(next_cluster) => next_cluster,
            ClusterState::End => 0x0FFFFFFF,
        };

        let sector_offset = (cluster as usize * 4) / self.bytes_per_sector;
        let offset = (cluster as usize * 4) % self.bytes_per_sector;

        let buffer = self.get_buffer_mut(sector_offset)?;

        buffer[offset + 0] = (next_cluster.wrapping_shr(0) & 0xFF) as u8;
        buffer[offset + 1] = (next_cluster.wrapping_shr(8) & 0xFF) as u8;
        buffer[offset + 2] = (next_cluster.wrapping_shr(16) & 0xFF) as u8;
        buffer[offset + 3] = (next_cluster.wrapping_shr(24) & 0xFF) as u8;

        Ok(())
    }

    fn get_buffer(&mut self, sector_offset: usize) -> base::error::Result<&[u8]> {
        self.get_buffer_common(sector_offset)
            .map(|val| &*val.contents)
    }

    fn get_buffer_mut(&mut self, sector_offset: usize) -> base::error::Result<&mut [u8]> {
        let sector = self.get_buffer_common(sector_offset)?;
        sector.modified = true;
        Ok(&mut sector.contents)
    }

    fn locate_buffer(&mut self, sector_offset: usize) -> (BufferLocation, usize) {
        // Search to see if the sector is cached
        let mut set_location = 0;
        let mut lowest_access_value = usize::MAX;

        let mut iter = self.sectors.iter_mut();
        for i in 0..CACHE_SIZE {
            let sector = match iter.next().unwrap() {
                Some(sector) => sector,
                None => {
                    set_location = i;
                    lowest_access_value = 0;
                    continue;
                }
            };

            if sector.index == sector_offset {
                sector.access_count = self.next_access_count;
                self.next_access_count += 1;
                return (BufferLocation::Found, i);
            }

            if sector.access_count < lowest_access_value {
                lowest_access_value = sector.access_count;
                set_location = i;
            }
        }

        (BufferLocation::Replace, set_location)
    }

    fn get_buffer_common<'a>(
        &'a mut self,
        sector_offset: usize,
    ) -> base::error::Result<&'a mut FATSector> {
        let (buffer_location, set_location) = self.locate_buffer(sector_offset);

        match buffer_location {
            BufferLocation::Found => return Ok(self.sectors[set_location].as_mut().unwrap()),
            BufferLocation::Replace => {}
        }

        // If not found, we must load it and cache it
        let mut buffer = vec![0; self.bytes_per_sector].into_boxed_slice();
        self.drive
            .lock(|drive| drive.read(sector_offset + self.first_fat_sector, &mut buffer))?;

        self.sectors[set_location] = Some(FATSector {
            index: sector_offset,
            contents: buffer,
            access_count: self.next_access_count,
            modified: false,
        });

        self.next_access_count += 1;

        Ok(self.sectors[set_location].as_mut().unwrap())
    }
}
