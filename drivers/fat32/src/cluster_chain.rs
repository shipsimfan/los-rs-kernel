use crate::fat::{ClusterState, FAT};
use alloc::vec::Vec;
use process::ProcessTypes;

pub type Cluster = u32;

pub struct ClusterChain {
    chain: Vec<Cluster>,
}

impl ClusterChain {
    pub fn new<T: ProcessTypes + 'static>(
        first_cluster: Cluster,
        fat: &mut FAT<T>,
    ) -> base::error::Result<Self> {
        Ok(ClusterChain {
            chain: fat.get_cluster_chain(first_cluster)?,
        })
    }

    pub fn first(&self) -> Cluster {
        self.chain[0]
    }

    pub fn len(&self) -> usize {
        self.chain.len()
    }

    pub fn get(&self, index: usize) -> Option<Cluster> {
        self.chain.get(index).map(|cluster| *cluster)
    }

    pub fn as_slice(&self) -> &[Cluster] {
        self.chain.as_slice()
    }

    pub fn shrink<T: ProcessTypes + 'static>(
        &mut self,
        new_length: usize,
        fat: &mut FAT<T>,
    ) -> base::error::Result<()> {
        let last_cluster = self.chain[new_length - 1];

        fat.set_next_cluster(last_cluster, ClusterState::End)?;
        for i in new_length..self.chain.len() {
            fat.free_cluster(self.chain[i])?;
        }

        self.chain.truncate(new_length);

        Ok(())
    }

    pub fn grow<T: ProcessTypes + 'static>(
        &mut self,
        new_length: usize,
        fat: &mut FAT<T>,
    ) -> base::error::Result<()> {
        for i in self.chain.len()..new_length {
            let new_cluster = fat.allocate_cluster()?;
            fat.set_next_cluster(self.chain[i - 1], ClusterState::Some(new_cluster))?;
            self.chain.push(new_cluster);
        }

        fat.set_next_cluster(*self.chain.last().unwrap(), ClusterState::End)?;

        Ok(())
    }

    pub fn push(&mut self, new_cluster: Cluster) {
        self.chain.push(new_cluster)
    }
}
