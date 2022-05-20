#![allow(dead_code)]

// Device paths
pub const PCI_IDE_PATH: &str = "/pci/1_1";
pub const IDE_PATH: &str = "/ide";

// Status
pub const STATUS_BUSY: usize = 0x80; // ATA_SR_BSY
pub const STATUS_DRIVE_READY: usize = 0x40; // ATA_SR_DRDY
pub const STATUS_DRIVE_FAULT: usize = 0x20; // ATA_SR_DF
pub const STATUS_DRIVE_SEEK_COMPLETE: usize = 0x10; // ATA_SR_DSC
pub const STATUS_DATA_REQUEST_READY: usize = 0x08; // ATA_SR_DRQ
pub const STATUS_CORRECTED_DATA: usize = 0x04; // ATA_SR_CORR
pub const STATUS_INDEX: usize = 0x02; // ATA_SR_IDX
pub const STATUS_ERROR: usize = 0x01; // ATA_SR_ERR

// Errors
pub const ERROR_BAD_BLOCK: usize = 0x80; // ATA_ER_BBK
pub const ERROR_UNCORRECTABLE_DATA: usize = 0x40; // ATA_ER_UNC
pub const ERROR_MEDIA_CHANGED: usize = 0x20; // ATA_ER_MC
pub const ERROR_ID_MARK_NOT_FOUND: usize = 0x10; // ATA_ER_IDNF
pub const ERROR_MEDIA_CHANGE_REQUEST: usize = 0x08; // ATA_ER_MCR
pub const ERROR_COMMAND_ABORTED: usize = 0x04; // ATA_ER_ABRT
pub const ERROR_TRACK_0_NOT_FOUND: usize = 0x02; // ATA_ER_TK0NF
pub const ERROR_NO_ADDRESS_MARK: usize = 0x01; // ATA_ER_AMNF

// ATA Commands
pub const COMMAND_READ_PIO: usize = 0x20; // ATA_CMD_READ_PIO
pub const COMMAND_READ_PIO_EXT: usize = 0x24; // ATA_CMD_READ_PIO_EXT
pub const COMMAND_READ_DMA: usize = 0xC8; // ATA_CMD_READ_DMA
pub const COMMAND_READ_DMA_EXT: usize = 0x25; // ATA_CMD_READ_DMA_EXT
pub const COMMAND_WRITE_PIO: usize = 0x30; // ATA_CMD_WRITE_PIO
pub const COMMAND_WRITE_PIO_EXT: usize = 0x34; // ATA_CMD_WRITE_PIO_EXT
pub const COMMAND_WRITE_DMA: usize = 0xCA; // ATA_CMD_WRITE_DMA
pub const COMMAND_WRITE_DMA_EXT: usize = 0x35; // ATA_CMD_WRITE_DMA_EXT
pub const COMMAND_CACHE_FLUSH: usize = 0xE7; // ATA_CMD_CACHE_FLUSH
pub const COMMAND_CACHE_FLUSH_EXT: usize = 0xEA; // ATA_CMD_CACHE_FLUSH_EXT
pub const COMMAND_PACKET: usize = 0xA0; // ATA_CMD_PACKET
pub const COMMAND_IDENTIFY_PACKET: usize = 0xA1; // ATA_CMD_IDENTIFY_PACKET
pub const COMMAND_IDENTIFY: usize = 0xEC; // ATA_CMD_IDENTIFY

// ATAPI Commands
pub const COMMAND_ATAPI_READ: usize = 0xA8; // ATAPI_CMD_READ
pub const COMMAND_ATAPI_EJECT: usize = 0x1B; // ATAPI_CMD_EJECT

// Identification Space
pub const IDENT_DEVICE_TYPE: usize = 0; // ATA_IDENT_DEVICETYPE
pub const IDENT_CYLINDERS: usize = 2; // ATA_IDENT_CYLINDERS
pub const IDENT_HEADS: usize = 6; // ATA_IDENT_HEADS
pub const IDENT_SECTORS: usize = 12; // ATA_IDENT_SECTORS
pub const IDENT_SERIAL: usize = 20; // ATA_IDENT_SERIAL
pub const IDENT_MODEL: usize = 54; // ATA_IDENT_MODEL
pub const IDENT_CAPABILITIES: usize = 98; // ATA_IDENT_CAPABILITIES
pub const IDENT_FIELD_VALID: usize = 106; // ATA_IDENT_FIELDVALID
pub const IDENT_MAX_LBA: usize = 120; // ATA_IDENT_MAX_LBA
pub const IDENT_COMMAND_SETS: usize = 164; // ATA_IDENT_COMMANDSETS
pub const IDENT_MAX_LBA_EXT: usize = 200; // ATA_IDENT_MAX_LBA_EXT

// Drive types
pub const DRIVE_TYPE_ATA: usize = 0x00; // IDE_ATA
pub const DRIVE_TYPE_ATAPI: usize = 0x01; // IDE_ATAPI

// Task file
pub const REGISTER_DATA: usize = 0x00; // ATA_REG_DATA
pub const REGISTER_ERROR: usize = 0x01; // ATA_REG_ERROR
pub const REGISTER_FEATURES: usize = 0x01; // ATA_REG_FEATURES
pub const REGISTER_SECTOR_COUNT_0: usize = 0x02; // ATA_REG_SECCOUNT0
pub const REGISTER_LBA_0: usize = 0x03; // ATA_REG_LBA0
pub const REGISTER_LBA_1: usize = 0x04; // ATA_REG_LBA1
pub const REGISTER_LBA_2: usize = 0x05; // ATA_REG_LBA2
pub const REGISTER_DRIVE_SELECT: usize = 0x06; // ATA_REG_HDDEVSEL
pub const REGISTER_COMMAND: usize = 0x07; // ATA_REG_COMMAND
pub const REGISTER_STATUS: usize = 0x07; // ATA_REG_STATUS
pub const REGISTER_SECTOR_COUNT_1: usize = 0x08; // ATA_REG_SECCOUNT1
pub const REGISTER_LBA_3: usize = 0x09; // ATA_REG_LBA3
pub const REGISTER_LBA_4: usize = 0x0A; // ATA_REG_LBA4
pub const REGISTER_LBA_5: usize = 0x0B; // ATA_REG_LBA5
pub const REGISTER_CONTROL: usize = 0x0C; // ATA_REG_CONTROL
pub const REGISTER_ALT_STATUS: usize = 0x0C; // ATA_REG_ALTSTATUS
pub const REGISTER_DEV_ADDRESS: usize = 0x0D; // ATA_REG_DEVADDRESS

// Directions
pub const DIRECTION_READ: usize = 0x00;
pub const DIRECTION_WRITE: usize = 0x01;

// Channels
#[derive(Debug, Clone)]
#[repr(usize)]
pub enum Channel {
    Primary,
    Secondary,
}

// Drives
#[derive(Debug, Clone)]
#[repr(usize)]
pub enum Drive {
    Master,
    Slave,
}

impl Channel {
    pub fn reg(&self, register: usize) -> usize {
        match &self {
            Channel::Primary => register,
            Channel::Secondary => 0x100 | register,
        }
    }
}

impl From<usize> for Channel {
    fn from(other: usize) -> Self {
        if other == 0 {
            Channel::Primary
        } else {
            Channel::Secondary
        }
    }
}

impl alloc::fmt::Display for Channel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Channel::Primary => "primary",
                Channel::Secondary => "secondary",
            }
        )
    }
}

impl Drive {
    pub fn select(&self) -> usize {
        match self {
            Drive::Master => 0,
            Drive::Slave => (1 << 4),
        }
    }
}

impl From<usize> for Drive {
    fn from(other: usize) -> Self {
        if other == 0 {
            Drive::Master
        } else {
            Drive::Slave
        }
    }
}

impl alloc::fmt::Display for Drive {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Drive::Master => "master",
                Drive::Slave => "slave",
            }
        )
    }
}
