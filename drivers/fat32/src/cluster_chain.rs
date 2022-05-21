use crate::fat::FAT;
use alloc::vec::Vec;
use base::multi_owner::Owner;
use process::{Mutex, ProcessTypes};

pub type Cluster = u32;

pub struct ClusterChain<T: ProcessTypes + 'static> {
    chain: Vec<Cluster>,
    fat: Owner<FAT<T>, Mutex<FAT<T>, T>>,
}

impl<T: ProcessTypes + 'static> ClusterChain<T> {
    pub fn new(
        first_cluster: Cluster,
        fat: Owner<FAT<T>, Mutex<FAT<T>, T>>,
    ) -> base::error::Result<Self> {
        let mut chain = fat.lock(|fat| fat.get_cluster_chain(first_cluster))?;

        Ok(ClusterChain { chain, fat })
    }

    pub fn fat(&self) -> &Owner<FAT<T>, Mutex<FAT<T>, T>> {
        &self.fat
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

    pub fn push(&mut self, new_cluster: Cluster) {
        self.chain.push(new_cluster)
    }
}
