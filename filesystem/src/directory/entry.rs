use crate::Metadata;
use alloc::string::String;

#[repr(C)]
pub struct DirectoryEntry {
    name: [u8; 256],
    class: usize,
    size: usize,
}

const DIRECTORY: usize = 0;
const FILE: usize = 1;

impl DirectoryEntry {
    pub fn new(name: String, metadata: Metadata) -> Self {
        let mut name_arr: [u8; 256] = [0; 256];
        let mut i = 0;
        for byte in name.as_bytes() {
            if i == 255 {
                break;
            }

            name_arr[i] = *byte;
            i += 1;
        }

        DirectoryEntry {
            name: name_arr,
            class: if metadata.is_directory() {
                DIRECTORY
            } else {
                FILE
            },
            size: metadata.size(),
        }
    }
}
