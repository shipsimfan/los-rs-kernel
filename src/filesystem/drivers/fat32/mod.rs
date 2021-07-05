use crate::{device::DeviceBox, error, filesystem::FilesystemStarter, logln};

pub fn detect_fat32_filesystem(
    drive: DeviceBox,
    start: usize,
    size: usize,
) -> Result<Option<FilesystemStarter>, error::Status> {
    logln!("Detecting FAT32 . . . ");

    Ok(None)
}
