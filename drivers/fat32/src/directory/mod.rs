use crate::{
    cluster_chain::{Cluster, ClusterChain},
    fat::FAT,
};
use alloc::{borrow::ToOwned, boxed::Box, string::String, vec::Vec};
use base::multi_owner::Owner;
use filesystem::{DirectoryTrait, Metadata};
use iter::DirectoryIterator;
use process::{Mutex, ProcessTypes};

mod entry;
mod iter;

pub struct Directory<T: ProcessTypes + 'static> {
    cluster_chain: ClusterChain<T>,
}

impl<T: ProcessTypes + 'static> Directory<T> {
    pub fn new(
        first_cluster: Cluster,
        fat: Owner<FAT<T>, Mutex<FAT<T>, T>>,
    ) -> base::error::Result<Box<dyn DirectoryTrait>> {
        Ok(Box::new(Directory {
            cluster_chain: ClusterChain::new(first_cluster, fat)?,
        }))
    }
}

impl<T: ProcessTypes + 'static> DirectoryTrait for Directory<T> {
    fn get_children(&self) -> base::error::Result<Vec<(String, Metadata)>> {
        let mut children = Vec::new();
        let mut iter = DirectoryIterator::new(&self.cluster_chain)?;
        while let Some(entry) = iter.next()? {
            children.push((
                entry.name().to_owned(),
                Metadata::new(entry.file_size(), entry.is_directory()),
            ))
        }

        Ok(children)
    }
}
