use crate::filesystem::{File, Metadata};
use alloc::{boxed::Box, string::String, vec::Vec};

pub trait Directory: Send {
    fn get_children(&self) -> crate::error::Result<Vec<(String, Metadata)>>; // Used to get sub files on initialization
    fn open_file(&self, filename: &str) -> crate::error::Result<Box<dyn File>>;
    fn open_directory(&self, directory_name: &str) -> crate::error::Result<Box<dyn Directory>>;
    fn make_file(&self, filename: &str) -> crate::error::Result<()>;
    fn make_directory(&self, directory_name: &str) -> crate::error::Result<()>;
    fn rename_file(&self, old_name: &str, new_name: &str) -> crate::error::Result<()>;
    fn rename_directory(&self, old_name: &str, new_name: &str) -> crate::error::Result<()>;
    fn remove(&self, name: &str) -> crate::error::Result<()>;
    fn update_metadata(&self, name: &str, new_metadata: Metadata) -> crate::error::Result<()>;
}
