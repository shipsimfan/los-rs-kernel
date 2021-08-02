use crate::{device::DeviceBox, error, locks::Mutex};
use alloc::{sync::Arc, vec::Vec};

pub type FATBox = Arc<Mutex<FAT>>;
pub type Cluster = u32;

enum ClusterState {
    Free,
    Some(Cluster),
    End,
}

pub struct FAT {
    drive: DeviceBox,
    sectors_per_cluster: u32,
    bytes_per_cluster: usize,
    num_fats: usize,
    fat_size: usize,
    first_fat_sector: usize,
    first_data_sector: u32,
    bytes_per_sector: usize,
    buffer: Vec<u8>,
    buffer_modified: bool,
    buffer_sector_offset: usize,
    next_free_cluster: u32,
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
        let mut buffer = Vec::with_capacity(bytes_per_sector as usize);
        for _ in 0..buffer.capacity() {
            buffer.push(0);
        }

        FAT {
            drive,
            sectors_per_cluster: sectors_per_cluster as u32,
            bytes_per_cluster: (sectors_per_cluster as usize) * (bytes_per_sector as usize),
            first_fat_sector: reserved_sector_count as usize,
            num_fats: num_fats as usize,
            fat_size: fat_size as usize,
            bytes_per_sector: bytes_per_sector as usize,
            first_data_sector: reserved_sector_count as u32 + ((num_fats as u32) * fat_size),
            buffer,
            buffer_modified: false,
            buffer_sector_offset: 0xFFFFFFFF,
            next_free_cluster: 0xFFFFFFFF,
        }
    }

    fn flush_buffer(&mut self) -> error::Result<()> {
        if self.buffer_modified {
            let mut drive = self.drive.lock();
            let mut sector = self.buffer_sector_offset + self.first_fat_sector;
            for _ in 0..self.num_fats {
                drive.write(sector, self.buffer.as_slice())?;
                sector += self.fat_size;
            }
            self.buffer_modified = false;
        }

        Ok(())
    }

    fn set_buffer_sector(&mut self, new_sector_offset: usize) -> error::Result<()> {
        if self.buffer_sector_offset == new_sector_offset {
            return Ok(());
        }

        self.flush_buffer()?;

        self.buffer_sector_offset = new_sector_offset;
        self.drive.lock().read(
            new_sector_offset + self.first_fat_sector,
            self.buffer.as_mut_slice(),
        )
    }

    fn get_next_cluster(&mut self, cluster: Cluster) -> error::Result<ClusterState> {
        let sector_offset = (cluster as usize * 4) / self.bytes_per_sector;
        let offset = (cluster as usize * 4) % self.bytes_per_sector;

        self.set_buffer_sector(sector_offset)?;

        let next_cluster = ((self.buffer[offset] as u32)
            | ((self.buffer[offset + 1] as u32) << 8)
            | ((self.buffer[offset + 2] as u32) << 16)
            | ((self.buffer[offset + 3] as u32) << 24))
            & 0x0FFFFFFF;

        Ok(if next_cluster >= 0x0FFFFFF8 {
            ClusterState::End
        } else if next_cluster == 0 {
            ClusterState::Free
        } else {
            ClusterState::Some(next_cluster)
        })
    }

    fn set_next_cluster(
        &mut self,
        cluster: Cluster,
        next_cluster: ClusterState,
    ) -> error::Result<()> {
        let next_cluster = match next_cluster {
            ClusterState::Free => 0,
            ClusterState::Some(next_cluster) => next_cluster,
            ClusterState::End => 0x0FFFFFFF,
        };

        let sector_offset = (cluster as usize * 4) / self.bytes_per_sector;
        let offset = (cluster as usize * 4) % self.bytes_per_sector;

        self.set_buffer_sector(sector_offset)?;

        self.buffer[offset + 0] = (next_cluster.wrapping_shr(0) & 0xFF) as u8;
        self.buffer[offset + 1] = (next_cluster.wrapping_shr(8) & 0xFF) as u8;
        self.buffer[offset + 2] = (next_cluster.wrapping_shr(16) & 0xFF) as u8;
        self.buffer[offset + 3] = (next_cluster.wrapping_shr(24) & 0xFF) as u8;

        self.buffer_modified = true;

        Ok(())
    }

    fn find_next_free_cluster(&mut self) -> error::Result<Cluster> {
        if self.next_free_cluster == 0xFFFFFFFF {
            self.next_free_cluster = 2;
        }

        let top = ((self.fat_size * self.bytes_per_sector) / 4) as u32;

        while self.next_free_cluster < top {
            match self.get_next_cluster(self.next_free_cluster)? {
                ClusterState::Free => return Ok(self.next_free_cluster),
                _ => self.next_free_cluster += 1,
            }
        }

        Err(error::Status::OutOfResource)
    }

    fn allocate_cluster(&mut self) -> error::Result<u32> {
        let cluster = self.find_next_free_cluster()?;
        self.set_next_cluster(cluster, ClusterState::End)?;
        Ok(cluster)
    }

    fn free_cluster(&mut self, cluster: u32) -> error::Result<()> {
        if self.next_free_cluster == 0xFFFFFFFF {
            self.find_next_free_cluster()?;
        }

        self.set_next_cluster(cluster, ClusterState::Free)?;
        if cluster < self.next_free_cluster {
            self.next_free_cluster = cluster;
        }
        Ok(())
    }

    pub fn get_cluster_chain(&mut self, first_cluster: Cluster) -> error::Result<Vec<Cluster>> {
        let mut cluster_chain = Vec::new();

        let mut cluster = first_cluster;
        loop {
            cluster_chain.push(cluster);
            cluster = match self.get_next_cluster(cluster)? {
                ClusterState::Free => return Err(error::Status::CorruptFilesystem),
                ClusterState::End => return Ok(cluster_chain),
                ClusterState::Some(next_cluster) => next_cluster,
            };
        }
    }

    pub fn grow_cluster_chain(
        &mut self,
        first_cluster: u32,
        num_clusters: usize,
    ) -> error::Result<()> {
        // Locate end of cluster chain
        let mut cluster = first_cluster;
        let mut cluster_count = 1;
        'search_loop: loop {
            cluster = match self.get_next_cluster(cluster)? {
                ClusterState::Free => return Err(error::Status::CorruptFilesystem),
                ClusterState::End => break 'search_loop,
                ClusterState::Some(next_cluster) => next_cluster,
            };
            cluster_count += 1;
        }

        // Allocate clusters
        for _ in cluster_count..num_clusters {
            let new_cluster = self.allocate_cluster()?;
            self.set_next_cluster(cluster, ClusterState::Some(new_cluster))?;
            cluster = new_cluster;
        }

        self.flush_buffer()?;

        Ok(())
    }

    pub fn shrink_cluster_chain(
        &mut self,
        first_cluster: u32,
        num_clusters: usize,
    ) -> error::Result<()> {
        let mut cluster = first_cluster;
        for _ in 0..num_clusters {
            cluster = match self.get_next_cluster(cluster)? {
                ClusterState::Some(next_cluster) => next_cluster,
                ClusterState::End => return Err(error::Status::InvalidArgument),
                ClusterState::Free => return Err(error::Status::CorruptFilesystem),
            };
        }

        let mut next_cluster = match self.get_next_cluster(cluster)? {
            ClusterState::End => return Ok(()),
            ClusterState::Free => return Err(error::Status::CorruptFilesystem),
            ClusterState::Some(next_cluster) => next_cluster,
        };

        self.set_next_cluster(cluster, ClusterState::End)?;
        loop {
            cluster = next_cluster;
            next_cluster = match self.get_next_cluster(cluster)? {
                ClusterState::End => return Ok(()),
                ClusterState::Free => return Err(error::Status::CorruptFilesystem),
                ClusterState::Some(next_cluster) => next_cluster,
            };
            self.free_cluster(cluster)?;
        }
    }

    pub fn free_cluster_chain(&mut self, first_cluster: Cluster) -> error::Result<()> {
        let mut cluster = first_cluster;

        loop {
            let next_cluster = self.get_next_cluster(cluster)?;

            self.free_cluster(cluster)?;

            cluster = match next_cluster {
                ClusterState::Some(cluster) => cluster,
                ClusterState::End => return Ok(()),
                ClusterState::Free => return Err(error::Status::CorruptFilesystem),
            }
        }
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
