#![no_std]

use alloc::{
    boxed::Box,
    string::{String, ToString},
};
use base::{log_info, log_warn, multi_owner::Owner};
use device::Device;
use fat::FAT;
use filesystem::Volume;
use process::{Mutex, ProcessTypes};

use crate::directory::Directory;

extern crate alloc;

mod cluster_chain;
mod directory;
mod fat;

static mut FAT32_INITLAIZED: bool = false;

const SECTOR_SIZE: usize = 512;

const MODULE_NAME: &str = "FAT32";

pub fn initialize<T: ProcessTypes + 'static>() {
    log_info!("Initializing . . . ");

    unsafe {
        assert!(!FAT32_INITLAIZED);
        FAT32_INITLAIZED = true;
    }

    filesystem::register_filesystem_driver(detect_fat32_filesystem::<T>);

    log_info!("Initialized!");
}

fn detect_fat32_filesystem<T: ProcessTypes + 'static>(
    drive_lock: &Owner<Box<dyn Device>, Mutex<Box<dyn Device>, T>>,
    start: usize,
    size: usize,
) -> base::error::Result<Option<Owner<Volume<T>, Mutex<Volume<T>, T>>>> {
    log_info!("Detecting if a volume is fat32 . . .");

    drive_lock.lock(|drive| inner_detect_fat32_filesystem(drive_lock.clone(), drive, start, size))
}

fn inner_detect_fat32_filesystem<T: ProcessTypes + 'static>(
    drive_lock: Owner<Box<dyn Device>, Mutex<Box<dyn Device>, T>>,
    drive: &mut Box<dyn Device>,
    start: usize,
    _: usize,
) -> base::error::Result<Option<Owner<Volume<T>, Mutex<Volume<T>, T>>>> {
    // Get the BIOS Parameter Block
    let mut bpb = Box::new([0u8; SECTOR_SIZE]);
    drive.read(start, bpb.as_mut_slice())?;

    // Locate signature in bpb
    let bpb_signature = bpb[0x42];
    if bpb_signature != 0x28 && bpb_signature != 0x29 {
        log_warn!("Invalid BPB signature");
        return Ok(None);
    }

    // Locate system identifier string in BPB
    let system_identifier = String::from_utf8_lossy(&bpb[0x52..0x5A]);
    if system_identifier != "FAT32   " {
        log_warn!("Invalid system identifier");
        return Ok(None);
    }

    // Locate bootable partition signature in BPB
    let boot_signature = &bpb[0x1FE..0x200];
    if boot_signature[0] != 0x55 || boot_signature[1] != 0xAA {
        log_warn!("Invalid boot signature");
        return Ok(None);
    }

    // Get FSInfo
    let mut fs_info = Box::new([0u8; 512]);
    let fs_info_sector = (bpb[0x30] as usize) | ((bpb[0x31] as usize) << 8);
    drive.read(fs_info_sector, fs_info.as_mut_slice())?;

    // Verify lead, middle, and trailing signatures
    if fs_info[0] != 0x52 || fs_info[1] != 0x52 || fs_info[2] != 0x61 || fs_info[3] != 0x41 {
        log_warn!("Invalid lead signature");
        return Ok(None);
    }

    if fs_info[0x1E4] != 0x72
        || fs_info[0x1E5] != 0x72
        || fs_info[0x1E6] != 0x41
        || fs_info[0x1E7] != 0x61
    {
        log_warn!("Invalid middle signature");
        return Ok(None);
    }

    if fs_info[0x1FC] != 0x00
        || fs_info[0x1FD] != 0x00
        || fs_info[0x1FE] != 0x55
        || fs_info[0x1FF] != 0xAA
    {
        log_warn!("Invalid trailing signature");
        return Ok(None);
    }
    drop(fs_info);

    // Get volume name and serial
    let serial = (bpb[0x43] as u32)
        | ((bpb[0x44] as u32) << 8)
        | ((bpb[0x45] as u32) << 16)
        | ((bpb[0x46] as u32) << 24);
    let name = String::from_utf8_lossy(&bpb[0x47..0x52]).trim().to_string();

    log_info!("Valid volume detected: \"{}\"", name);

    // Get root directory cluster
    let root_directory_cluster = (bpb[0x2C] as u32)
        | ((bpb[0x2D] as u32) << 8)
        | ((bpb[0x2E] as u32) << 16)
        | ((bpb[0x2F] as u32) << 24);

    // Gather FAT info
    let bytes_per_sector = (bpb[0x0B] as u16) | ((bpb[0x0C] as u16) << 8);
    let sectors_per_cluster = bpb[0x0D];
    let reserved_sector_count = (bpb[0x0E] as u16) | ((bpb[0x0F] as u16) << 8);
    let num_fats = bpb[0x10];
    let fat_size = (bpb[0x24] as u32)
        | ((bpb[0x25] as u32) << 8)
        | ((bpb[0x26] as u32) << 16)
        | ((bpb[0x27] as u32) << 24);

    drop(bpb);

    // Create FAT
    let fat = FAT::new(
        drive_lock,
        bytes_per_sector,
        sectors_per_cluster,
        reserved_sector_count,
        num_fats,
        fat_size,
        start / SECTOR_SIZE,
    );

    // Create root directory
    let root_directory = Directory::new(root_directory_cluster, fat)?;

    // Create volume
    Ok(Some(Volume::new(
        name,
        serial as u64,
        root_directory,
        Box::new(fat),
    )?))
}
