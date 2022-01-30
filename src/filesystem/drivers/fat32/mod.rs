use crate::{device::DeviceReference, error, filesystem::FilesystemStarter, locks::Mutex};
use alloc::{
    boxed::Box,
    string::{String, ToString},
    sync::Arc,
};

mod directory;
mod fat;
mod file;

const SECTOR_SIZE: usize = 512;

pub fn detect_fat32_filesystem(
    drive_lock: DeviceReference,
    start: usize,
    _: usize,
) -> error::Result<Option<FilesystemStarter>> {
    let mut drive = drive_lock.lock();

    // Get BPB
    let mut bpb = [0u8; 512];
    drive.read(start * SECTOR_SIZE, &mut bpb)?;

    bpb[0] = 0xFF;
    bpb[1] = 0xAB;

    drive.write(start * SECTOR_SIZE, &bpb)?;

    // Locate signature in BPB
    let bpb_signature = bpb[0x42];
    if bpb_signature != 0x28 && bpb_signature != 0x29 {
        return Ok(None);
    }

    // Locate system identifier string in BPB
    let system_identifier = String::from_utf8_lossy(&bpb[0x52..0x5A]);
    if system_identifier != "FAT32   " {
        return Ok(None);
    }

    // Locate bootable partition signature in BPB
    let boot_signature = &bpb[0x1FE..0x200];
    if boot_signature[0] != 0x55 || boot_signature[1] != 0xAA {
        return Ok(None);
    }

    // Get FSInfo
    let mut fs_info = [0u8; 512];
    let fs_info_sector = (bpb[0x30] as usize) | ((bpb[0x31] as usize) << 8);
    drive.read(fs_info_sector, &mut fs_info)?;

    // Verify lead, middle, and trailing signatures
    if fs_info[0] != 0x52 || fs_info[1] != 0x52 || fs_info[2] != 0x61 || fs_info[3] != 0x41 {
        return Ok(None);
    }

    if fs_info[0x1E4] != 0x72
        || fs_info[0x1E5] != 0x72
        || fs_info[0x1E6] != 0x41
        || fs_info[0x1E7] != 0x61
    {
        return Ok(None);
    }

    if fs_info[0x1FC] != 0x00
        || fs_info[0x1FD] != 0x00
        || fs_info[0x1FE] != 0x55
        || fs_info[0x1FF] != 0xAA
    {
        return Ok(None);
    }
    drop(fs_info);

    // Gather FAT info
    let sectors_per_cluster = bpb[0x0D];
    let reserved_sector_count = (bpb[0x0E] as u16) | ((bpb[0x0F] as u16) << 8);
    let num_fats = bpb[0x10];
    let fat_size = (bpb[0x24] as u32)
        | ((bpb[0x25] as u32) << 8)
        | ((bpb[0x26] as u32) << 16)
        | ((bpb[0x27] as u32) << 24);
    let bytes_per_sector = (bpb[0x0B] as u16) | ((bpb[0x0C] as u16) << 8);

    // Create FAT
    let fat = Arc::new(Mutex::new(fat::FAT::new(
        drive_lock.clone(),
        sectors_per_cluster,
        reserved_sector_count,
        num_fats,
        fat_size,
        bytes_per_sector,
    )));

    // Get volume name
    let volume_name = String::from_utf8_lossy(&bpb[0x47..0x52]).trim().to_string();

    // Get root directory cluster
    let root_directory_cluster = (bpb[0x2C] as u32)
        | ((bpb[0x2D] as u32) << 8)
        | ((bpb[0x2E] as u32) << 16)
        | ((bpb[0x2F] as u32) << 24);

    // Create filesystem starter
    Ok(Some(FilesystemStarter::new(
        Box::new(directory::Directory::new(root_directory_cluster, fat)),
        volume_name,
    )))
}
