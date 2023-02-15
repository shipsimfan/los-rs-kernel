pub(super) struct Bitmap<const ENTRIES: usize>
where
    [usize; bitmap_size(ENTRIES)]: Sized,
{
    map: [usize; bitmap_size(ENTRIES)],
}

const USIZE_SIZE: usize = core::mem::size_of::<usize>();
const USIZE_BITS: usize = USIZE_SIZE * 8;

pub(super) const fn bitmap_size(entries: usize) -> usize {
    (entries + USIZE_SIZE - 1) / USIZE_SIZE
}

const fn convert_index(index: usize) -> (usize, u32) {
    let bit = (USIZE_BITS - 1 - (index % 64)) as u32;
    let offset = index / 64;
    (offset, bit)
}

const fn test_bit(value: usize, bit: u32) -> bool {
    (value.wrapping_shr(bit) & 1) == 1
}

impl<const ENTRIES: usize> Bitmap<ENTRIES>
where
    [usize; bitmap_size(ENTRIES)]: Sized,
{
    pub(super) const fn null() -> Self {
        Bitmap {
            map: [usize::MAX; bitmap_size(ENTRIES)],
        }
    }

    pub(super) fn get(&self, index: usize) -> bool {
        assert!(index < ENTRIES);
        let (offset, bit) = convert_index(index);
        test_bit(self.map[offset], bit)
    }

    pub(super) fn get_next(&self, start: usize, value: bool) -> Option<usize> {
        let (start_offset, mut start_bit) = convert_index(start);
        let (end_offset, final_bits) = convert_index(ENTRIES);

        let mut index = start;
        for offset in start_offset..end_offset {
            let end_bits = if offset == end_offset - 1 {
                final_bits
            } else {
                USIZE_BITS as u32
            };

            for bit in start_bit..end_bits {
                if test_bit(self.map[offset], bit) == value {
                    return Some(index);
                }

                index += 1;
            }

            start_bit = 0;
        }

        None
    }

    pub(super) fn set(&mut self, index: usize) {
        assert!(index < ENTRIES);
        let (offset, bit) = convert_index(index);

        self.map[offset] |= 1 << bit;
    }

    pub(super) fn clear(&mut self, index: usize) {
        assert!(index < ENTRIES);
        let (offset, bit) = convert_index(index);

        self.map[offset] &= !(1 << bit);
    }
}
