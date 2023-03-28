pub(super) trait Checksum: Sized {
    fn length(&self) -> usize;

    fn verify_checksum(&self) -> bool {
        let slice =
            unsafe { core::slice::from_raw_parts(self as *const _ as *const u8, self.length()) };

        let mut sum: u8 = 0;
        for byte in slice {
            sum = sum.wrapping_add(*byte);
        }

        sum == 0
    }
}
