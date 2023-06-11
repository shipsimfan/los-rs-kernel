use super::SlabInfo;

pub(super) struct Object {
    next: u16,
}

const OBJECT_POISON: u8 = 0xF1;
const PADDING_POISON: u8 = 0x4A;

impl Object {
    pub(super) fn initialize(&mut self, info: &SlabInfo, next: Option<u16>) {
        assert!(info.object_size() >= core::mem::size_of::<Object>());

        let next = if let Some(next) = next {
            assert_ne!(next, u16::MAX);
            next
        } else {
            u16::MAX
        };

        self.next = next;

        let poison = self.as_slice_mut(info.total_object_size());

        for i in core::mem::size_of::<Object>()..info.object_size() {
            poison[i] = OBJECT_POISON;
        }

        for i in info.object_size()..info.total_object_size() {
            poison[i] = PADDING_POISON;
        }
    }

    pub(super) fn next(&self, info: &SlabInfo) -> Option<u16> {
        self.verify_poison(info.object_size(), info.total_object_size());

        if self.next == u16::MAX {
            None
        } else {
            Some(self.next)
        }
    }

    fn verify_poison(&self, object_size: usize, total_object_size: usize) {
        let poison = self.as_slice(total_object_size);

        for i in core::mem::size_of::<Object>()..object_size {
            assert_eq!(poison[i], OBJECT_POISON);
        }

        for i in object_size..total_object_size {
            assert_eq!(poison[i], OBJECT_POISON);
        }
    }

    fn as_slice(&self, total_object_size: usize) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self as *const Self as *const u8, total_object_size) }
    }

    fn as_slice_mut(&mut self, total_object_size: usize) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self as *mut Self as *mut u8, total_object_size) }
    }
}
