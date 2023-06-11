use super::{Slab, SlabInfo};
use core::ptr::NonNull;

pub(super) struct SlabList {
    head: Option<NonNull<Slab>>,
}

impl SlabList {
    pub(super) const fn new() -> Self {
        SlabList { head: None }
    }

    pub(super) fn head_mut(&mut self) -> Option<&mut Slab> {
        self.head.map(|mut head| unsafe { head.as_mut() })
    }

    pub(super) fn push(&mut self, info: &SlabInfo, mut new_slab: NonNull<Slab>) {
        self.head_mut()
            .map(|slab| slab.set_prev(info, Some(new_slab)));

        unsafe { new_slab.as_mut() }.set_next(info, self.head.take());
        self.head = Some(new_slab);
    }

    pub(super) fn pop(&mut self, info: &SlabInfo) -> Option<NonNull<Slab>> {
        let mut slab = match self.head.take() {
            Some(slab) => slab,
            None => return None,
        };

        unsafe { slab.as_ref() }.next().map(|mut next| {
            unsafe { next.as_mut() }.set_prev(info, None);
            self.head = Some(next);
        });

        unsafe { slab.as_mut() }.set_next(info, None);

        Some(slab)
    }
}
