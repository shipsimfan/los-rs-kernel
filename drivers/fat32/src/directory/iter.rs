use super::entry::{
    DirectoryEntry, DiskDirectoryEntry, LongDirectoryEntry, ATTRIBUTE_DIRECTORY,
    ATTRIBUTE_LONG_FILE_NAME, ATTRIBUTE_VOLUME_ID,
};
use crate::{
    cluster_chain::ClusterChain,
    fat::{ClusterState, FAT},
};
use alloc::{borrow::ToOwned, boxed::Box, string::String, vec::Vec};
use base::error::FAT32_FS_DRIVER_MODULE_NUMBER;
use process::ProcessTypes;

#[derive(Debug)]
enum IteratorError {
    OutOfRange,
}

struct Buffer<'a, 'b, T: ProcessTypes + 'static> {
    buffer: Box<[u8]>,
    current_cluster_index: usize,
    cluster_chain: &'a mut ClusterChain,
    modified: bool,
    fat: &'b mut FAT<T>,
}

pub struct DirectoryIterator<'a, 'b, T: ProcessTypes + 'static> {
    buffer: Buffer<'a, 'b, T>,
    current_index: Option<usize>,
    cluster_size: usize,
}

impl<'a, 'b, T: ProcessTypes + 'static> Buffer<'a, 'b, T> {
    pub fn new(
        cluster_chain: &'a mut ClusterChain,
        fat: &'b mut FAT<T>,
    ) -> base::error::Result<Self> {
        // Create the buffer
        let mut buffer;

        // Get first cluster
        buffer = Vec::with_capacity(fat.bytes_per_cluster());
        unsafe { buffer.set_len(buffer.capacity()) };

        fat.read_cluster(cluster_chain.first(), buffer.as_mut_slice())?;

        Ok(Buffer {
            buffer: buffer.into_boxed_slice(),
            current_cluster_index: 0,
            cluster_chain,
            modified: false,
            fat,
        })
    }

    pub fn read(&self, offset: usize, buffer: &mut [u8]) -> base::error::Result<()> {
        if offset + buffer.len() > self.buffer.len() {
            return Err(Box::new(IteratorError::OutOfRange));
        }

        for i in 0..buffer.len() {
            buffer[i] = self.buffer[i + offset];
        }

        Ok(())
    }

    pub fn write(&mut self, offset: usize, buffer: &[u8]) -> base::error::Result<()> {
        if offset + buffer.len() > self.buffer.len() {
            return Err(Box::new(IteratorError::OutOfRange));
        }

        self.modified = true;
        for i in 0..buffer.len() {
            self.buffer[i + offset] = buffer[i];
        }

        Ok(())
    }

    pub fn flush_buffer(&mut self) -> base::error::Result<()> {
        if self.modified {
            self.fat.write_cluster(
                self.cluster_chain.get(self.current_cluster_index).unwrap(),
                &self.buffer,
            )?;
            self.modified = false;
        }

        self.fat.flush_buffer()
    }

    pub fn set_current_cluster_index(
        &mut self,
        new_cluster_index: usize,
    ) -> base::error::Result<()> {
        if new_cluster_index == self.current_cluster_index {
            return Ok(());
        }

        if new_cluster_index == self.cluster_chain.len() {
            // Allocate new cluster
            let new_cluster = self.fat.allocate_cluster()?;
            let last_cluster_index = self.cluster_chain.len() - 1;
            self.fat.set_next_cluster(
                self.cluster_chain.get(last_cluster_index).unwrap(),
                ClusterState::Some(new_cluster),
            )?;
            self.cluster_chain.push(new_cluster);
        } else if new_cluster_index > self.cluster_chain.len() {
            return Err(Box::new(IteratorError::OutOfRange));
        }

        self.current_cluster_index = new_cluster_index;
        self.fat.read_cluster(
            self.cluster_chain.get(self.current_cluster_index).unwrap(),
            &mut self.buffer,
        )
    }
}

impl<'a, 'b, T: ProcessTypes + 'static> DirectoryIterator<'a, 'b, T> {
    pub fn new(
        cluster_chain: &'a mut ClusterChain,
        fat: &'b mut FAT<T>,
    ) -> base::error::Result<Self> {
        let cluster_size = fat.bytes_per_cluster();
        let buffer = Buffer::new(cluster_chain, fat)?;

        Ok(DirectoryIterator {
            buffer,
            current_index: None,
            cluster_size,
        })
    }

    pub fn next(&mut self) -> base::error::Result<Option<DirectoryEntry>> {
        let mut entry_buffer = [0u8; core::mem::size_of::<DiskDirectoryEntry>()];
        let mut long_filename = [0u16; 256];
        let mut long_checksum = 0;
        let mut next_ord = 0;
        loop {
            self.increament_index();

            let (cluster_index, offset) = self.get_cluster_index_and_offset();

            self.buffer.set_current_cluster_index(cluster_index)?;
            self.buffer.read(offset, &mut entry_buffer)?;
            let entry = DiskDirectoryEntry::from_slice(&entry_buffer);

            if entry.filename[0] == 0 {
                return Ok(None);
            }

            if entry.filename[0] == 0xE5 {
                long_filename[0] = 0;
                next_ord = 0;
                continue;
            }

            if entry.attributes & ATTRIBUTE_LONG_FILE_NAME == ATTRIBUTE_LONG_FILE_NAME {
                let long_entry = LongDirectoryEntry::from_entry(&entry);

                if long_entry.order() & 0x40 != 0 {
                    next_ord = long_entry.order() & !0x40;
                    long_checksum = long_entry.checksum();
                    if (next_ord as usize) * 13 < 256 {
                        long_filename[next_ord as usize * 13] = 0;
                    }
                }

                if next_ord == 0 {
                    long_filename[0] = 0;
                    continue;
                }
                if long_entry.order() & !0x40 != next_ord || long_entry.checksum() != long_checksum
                {
                    long_filename[0] = 0;
                    next_ord = 0;
                    continue;
                }

                next_ord -= 1;
                let offset = next_ord as usize * 13;
                for i in 0..5 {
                    long_filename[offset + i] = long_entry.name1()[i];
                }

                for i in 0..6 {
                    long_filename[offset + i + 5] = long_entry.name2()[i];
                }

                for i in 0..2 {
                    long_filename[offset + i + 11] = long_entry.name3()[i];
                }

                continue;
            }

            if entry.attributes & ATTRIBUTE_VOLUME_ID != 0 {
                long_filename[0] = 0;
                next_ord = 0;
                continue;
            }

            let mut filename;
            if long_filename[0] == 0 {
                if entry.attributes & ATTRIBUTE_DIRECTORY == 0 {
                    filename = String::new();
                    for i in 0..8 {
                        if i == 0 && entry.filename[0] == 0x05 {
                            filename.push((0xE5 as char).to_ascii_lowercase());
                        } else {
                            filename.push((entry.filename[i] as char).to_ascii_lowercase());
                        }
                    }
                    filename = filename.trim().to_owned();
                    if entry.filename[8] != b' '
                        || entry.filename[9] != b' '
                        || entry.filename[10] != b' '
                    {
                        filename.push('.');
                        for i in 8..11 {
                            filename.push((entry.filename[i] as char).to_ascii_lowercase());
                        }

                        filename = filename.trim().to_owned();
                    }
                } else {
                    filename = String::from_utf8_lossy(&entry.filename)
                        .trim()
                        .to_ascii_lowercase();
                }
            } else {
                filename = String::new();
                for c in long_filename {
                    if c == 0 {
                        break;
                    }

                    filename.push(unsafe { char::from_u32_unchecked(c as u32) });
                }
            }

            if filename == ".." || filename == "." {
                continue;
            }

            return Ok(Some(DirectoryEntry::new(
                filename,
                entry.attributes & ATTRIBUTE_DIRECTORY != 0,
                (entry.first_cluster_low as u32) | ((entry.first_cluster_high as u32) << 16),
                entry.file_size as usize,
            )));
        }
    }

    pub fn create(&mut self, entry: DirectoryEntry) -> base::error::Result<()> {
        // Create disk entry and long directory entries
        let (disk_entry, long_entries) = entry.to_disk_entries()?;

        // Find a space for the entries
        let num_entries = 1 + long_entries.len();
        let mut current_free_entries = 0;
        let mut entry_buffer = [0u8; core::mem::size_of::<DiskDirectoryEntry>()];
        loop {
            self.increament_index();

            let (cluster_index, offset) = self.get_cluster_index_and_offset();
            self.buffer.set_current_cluster_index(cluster_index)?;
            self.buffer.read(offset, &mut entry_buffer)?;
            let entry = DiskDirectoryEntry::from_slice(&entry_buffer);

            if entry.filename[0] != 0 && entry.filename[0] != 0xE5 {
                current_free_entries = 0;
                continue;
            }

            current_free_entries += 1;
            if current_free_entries >= num_entries {
                break;
            }
        }

        // Write the entries
        let (cluster_index, offset) = self.get_cluster_index_and_offset();
        self.buffer.set_current_cluster_index(cluster_index)?;
        let disk_entry_slice = disk_entry.to_slice();
        self.buffer.write(offset, &disk_entry_slice)?;

        for entry in long_entries {
            self.decreament_index();
            let slice = entry.to_slice();

            let (cluster_index, offset) = self.get_cluster_index_and_offset();
            self.buffer.set_current_cluster_index(cluster_index)?;
            self.buffer.write(offset, &slice)?;
        }

        Ok(())
    }

    pub fn remove(&mut self) -> base::error::Result<()> {
        let mut write_buffer = [0u8; core::mem::size_of::<DiskDirectoryEntry>()];
        let mut read_buffer = [0u8; core::mem::size_of::<DiskDirectoryEntry>()];

        // Check next entry
        self.increament_index();
        let (next_cluster_index, next_offset) = self.get_cluster_index_and_offset();
        self.buffer.set_current_cluster_index(next_cluster_index)?;
        self.buffer.read(next_offset, &mut read_buffer)?;

        // Initialize the write buffer
        if read_buffer[0] != 0 {
            write_buffer[0] = 0xE5;
        }

        // Clear current entry
        self.decreament_index();
        let (cluster_index, offset) = self.get_cluster_index_and_offset();
        self.buffer.set_current_cluster_index(cluster_index)?;
        self.buffer.write(offset, &write_buffer)?;

        // Remove long directory entries
        let mut next_ord = 1;
        loop {
            // Read entry
            self.decreament_index();
            let (cluster_index, offset) = self.get_cluster_index_and_offset();
            self.buffer.set_current_cluster_index(cluster_index)?;
            self.buffer.read(offset, &mut read_buffer)?;
            let entry = DiskDirectoryEntry::from_slice(&read_buffer);

            // Check to see if valid
            if entry.attributes & ATTRIBUTE_LONG_FILE_NAME != ATTRIBUTE_LONG_FILE_NAME {
                break;
            }

            if entry.filename[0] & !0x40 != next_ord {
                break;
            }

            // Remove if valid
            self.buffer.write(offset, &write_buffer)?;

            if entry.filename[0] & 0x40 != 0 {
                break;
            }

            next_ord += 1;
        }

        Ok(())
    }

    pub fn write_metadata(&mut self, new_entry: DirectoryEntry) -> base::error::Result<()> {
        let mut buffer = [0u8; core::mem::size_of::<DiskDirectoryEntry>()];
        let (cluster_index, offset) = self.get_cluster_index_and_offset();
        self.buffer.set_current_cluster_index(cluster_index)?;
        self.buffer.read(offset, &mut buffer)?;
        let mut entry = DiskDirectoryEntry::from_slice(&buffer);

        entry.file_size = new_entry.file_size() as u32;
        entry.first_cluster_low = (new_entry.first_cluster() & 0xFFFF) as u16;
        entry.first_cluster_high = (new_entry.first_cluster().wrapping_shr(16) & 0xFFFF) as u16;

        let buffer = entry.to_slice();
        self.buffer.write(offset, &buffer)?;

        Ok(())
    }

    pub fn flush_buffer(&mut self) -> base::error::Result<()> {
        self.buffer.flush_buffer()
    }

    #[inline(always)]
    fn get_cluster_index_and_offset(&self) -> (usize, usize) {
        let byte_index = self.current_index.unwrap() * core::mem::size_of::<DiskDirectoryEntry>();
        (
            byte_index / self.cluster_size,
            byte_index % self.cluster_size,
        )
    }

    #[inline(always)]
    fn increament_index(&mut self) {
        match &mut self.current_index {
            None => self.current_index = Some(0),
            Some(idx) => *idx += 1,
        }
    }

    #[inline(always)]
    fn decreament_index(&mut self) {
        match &mut self.current_index {
            None => {}
            Some(idx) => {
                if *idx == 0 {
                    self.current_index = None
                } else {
                    *idx -= 1;
                }
            }
        }
    }
}

impl base::error::Error for IteratorError {
    fn module_number(&self) -> i32 {
        FAT32_FS_DRIVER_MODULE_NUMBER
    }

    fn error_number(&self) -> base::error::Status {
        match self {
            IteratorError::OutOfRange => base::error::Status::OutOfRange,
        }
    }
}

impl core::fmt::Display for IteratorError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            IteratorError::OutOfRange => write!(f, "Out of range buffer read"),
        }
    }
}
