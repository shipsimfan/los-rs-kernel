use super::order_to_size;
use crate::memory::buddy::MAX_ORDER;
use core::ptr::NonNull;

#[repr(C)]
pub(super) struct Page {
    header_poison: u64,
    next: Option<NonNull<Page>>,
    order: u8,
}

const HEADER_POISON: u64 = 0xE18E3EA7023551FB;
const FOOTER_POISON: u64 = 0x6AC66D0950305DA9;

impl Page {
    pub(super) fn new(address: NonNull<u8>, order: u8) -> NonNull<Page> {
        let mut page = address.cast::<Page>();
        unsafe { page.as_mut() }.initialize(order);
        page
    }

    pub(super) fn next(&self) -> Option<NonNull<Page>> {
        self.check_poisons();
        self.next
    }

    pub(super) fn order(&self) -> u8 {
        self.order
    }

    pub(super) fn set_next(&mut self, next: Option<NonNull<Page>>) {
        self.check_poisons();
        self.next = next;
    }

    pub(super) fn split(&mut self) -> NonNull<Page> {
        self.check_poisons();
        assert!(self.order > 0);

        let mut buddy = NonNull::new(
            (self as *const Page as usize + order_to_size(self.order - 1)) as *mut Page,
        )
        .unwrap();

        unsafe { buddy.as_mut() }.initialize(self.order - 1);
        self.initialize(self.order - 1);

        buddy
    }

    fn initialize(&mut self, order: u8) {
        assert!(order < MAX_ORDER);
        assert_eq!((self as *const Page as usize) % order_to_size(order), 0);

        self.next = None;
        self.order = order;

        self.set_poisons();
    }

    fn check_poisons(&self) {
        let adjust = self.calculate_poison_adjust();
        assert_eq!(self.header_poison, HEADER_POISON + adjust);
        assert_eq!(self.footer_poison(), FOOTER_POISON - adjust);
    }

    fn set_poisons(&mut self) {
        let adjust = self.calculate_poison_adjust();
        self.header_poison = HEADER_POISON + adjust;
        self.set_footer_poison(FOOTER_POISON - adjust);
    }

    fn calculate_poison_adjust(&self) -> u64 {
        self.order as u64 * 7
    }

    fn footer_poison(&self) -> u64 {
        let mut ptr = self as *const _ as usize;
        ptr += order_to_size(self.order) - core::mem::size_of::<u64>();

        unsafe { *(ptr as *const u64) }
    }

    fn set_footer_poison(&mut self, value: u64) {
        let mut ptr = self as *const _ as usize;
        ptr += order_to_size(self.order) - core::mem::size_of::<u64>();
        *unsafe { &mut *(ptr as *mut u64) } = value;
    }
}
