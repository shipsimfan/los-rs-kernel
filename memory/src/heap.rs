use super::KERNEL_VMA;
use core::{
    alloc::{GlobalAlloc, Layout},
    ptr::null_mut,
};

pub struct Heap;

#[repr(packed(1))]
struct Block {
    below_size: usize,
    signature: usize,
    above_size: usize,
}

const SIGNATURE: usize = 0x71926360D3B6CF37;
pub const HEAP_START_OFFSET: usize = 0x100000000000;
const HEAP_SIZE: usize = 0x100000000000;

static mut LOWEST_FREE_BLOCK: Option<*mut Block> = None;

pub unsafe fn initialize() {
    assert!(LOWEST_FREE_BLOCK.is_none());

    LOWEST_FREE_BLOCK = Some(&mut *((KERNEL_VMA + HEAP_START_OFFSET) as *mut Block));
    match LOWEST_FREE_BLOCK {
        Some(lowest_free_block) => {
            let lowest_free_block = &mut *lowest_free_block;
            *lowest_free_block = Block::new_bottom(HEAP_SIZE - 2 * core::mem::size_of::<Block>());

            let upper_block = lowest_free_block.get_next().unwrap();
            *upper_block = Block::new_top(lowest_free_block.above_size());
        }
        None => panic!("We should never reach here!"),
    }
}

impl Block {
    pub fn new(below_size: usize, below_free: bool, above_size: usize, above_free: bool) -> Self {
        if below_size == 0 || above_size == 0 {
            panic!("Creating 0 size block!");
        }

        Block {
            below_size: (below_size & !7) | if below_free { 1 } else { 0 },
            signature: SIGNATURE,
            above_size: (above_size & !7) | if above_free { 1 } else { 0 },
        }
    }

    pub fn new_bottom(above_size: usize) -> Self {
        Block {
            below_size: 2,
            signature: SIGNATURE,
            above_size: above_size | 1,
        }
    }

    pub fn new_top(below_size: usize) -> Self {
        Block {
            below_size: below_size | 1,
            signature: SIGNATURE,
            above_size: 2,
        }
    }

    pub unsafe fn get_prev(&self) -> Option<&mut Block> {
        if self.above_size == 2 {
            None
        } else {
            Some(
                &mut *(((self as *const _ as usize)
                    - (self.below_size & !7)
                    - core::mem::size_of::<Block>()) as *mut Block),
            )
        }
    }

    pub unsafe fn get_next(&self) -> Option<&mut Block> {
        if self.above_size == 2 {
            None
        } else {
            Some(
                &mut *(((self as *const _ as usize)
                    + (self.above_size & !7)
                    + core::mem::size_of::<Block>()) as *mut Block),
            )
        }
    }

    pub fn below_size(&self) -> usize {
        self.below_size & !7
    }

    pub fn above_size(&self) -> usize {
        self.above_size & !7
    }

    pub fn is_above_free(&self) -> bool {
        self.above_size & 1 != 0
    }

    pub fn set_below(&mut self, free: bool) {
        if self.below_size != 2 {
            if free {
                self.below_size |= 1;
            } else {
                self.below_size &= !1;
            }
        }
    }

    pub fn set_above(&mut self, free: bool) {
        if self.above_size != 2 {
            if free {
                self.above_size |= 1;
            } else {
                self.above_size &= !1;
            }
        }
    }

    pub fn set_below_size(&mut self, new_size: usize) {
        if new_size == 0 {
            panic!("Setting 0 size block!");
        }

        if self.below_size != 2 {
            let orig = self.below_size & 7;
            self.below_size = (new_size & !7) | orig;
        }
    }

    pub fn set_above_size(&mut self, new_size: usize) {
        if new_size == 0 {
            panic!("Setting 0 size block!");
        }

        if self.above_size != 2 {
            let orig = self.above_size & 7;
            self.above_size = (new_size & !7) | orig;
        }
    }
}

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let lowest_free_block = match LOWEST_FREE_BLOCK {
            Some(lowest_free_block) => lowest_free_block,
            None => panic!("Attempting to allocate before heap is initialized!"),
        };

        if layout.size() == 0 {
            return null_mut();
        }

        let layout_size = if layout.size() <= 8 {
            8
        } else {
            if layout.size() % 8 != 0 {
                ((layout.size() + 8) / 8) * 8
            } else {
                layout.size()
            }
        };

        let layout_align = if layout.align() <= 8 {
            8
        } else {
            layout.align()
        };

        // Loop through blocks
        let mut current_block_ptr = lowest_free_block;
        'block_loop: loop {
            let mut current_block = &mut *current_block_ptr;
            if current_block.signature != SIGNATURE {
                panic!("Heap corruption at {:p}", current_block as *mut _);
            }

            if current_block.is_above_free() {
                if current_block.above_size() == layout_size {
                    let ptr = current_block as *mut _ as usize + core::mem::size_of::<Block>();
                    if ptr % layout_align == 0 {
                        current_block.set_above(false);
                        let next_block = current_block.get_next().unwrap();
                        next_block.set_below(false);
                        return ptr as *mut u8;
                    }
                } else if current_block.above_size() >= layout_size + core::mem::size_of::<Block>()
                {
                    // Check pointer alignment
                    let ptr = current_block as *const _ as usize + core::mem::size_of::<Block>();
                    if ptr % layout_align != 0 {
                        let previous_block = current_block.get_prev().unwrap();
                        let next_block = current_block.get_next().unwrap();

                        // If not aligned, attempt to move the lower block up to alignment
                        let increase = layout_align - (ptr % layout_align);
                        let new_size_below = current_block.below_size() + increase;
                        let new_size_above = current_block.above_size() - increase;

                        if new_size_above == layout_size {
                            previous_block.set_above_size(new_size_below);
                            next_block.set_below_size(new_size_above);
                            next_block.set_below(false);

                            current_block = &mut *((current_block as *const _ as usize + increase)
                                as *mut Block);
                            *current_block = Block::new(
                                new_size_below,
                                previous_block.is_above_free(),
                                new_size_above,
                                false,
                            );

                            return (ptr + increase) as *mut u8;
                        } else if new_size_above >= layout_size + core::mem::size_of::<Block>() {
                            previous_block.set_above_size(new_size_below);
                            next_block.set_below_size(new_size_above);

                            current_block = &mut *((current_block as *const _ as usize + increase)
                                as *mut Block);
                            *current_block = Block::new(
                                new_size_below,
                                previous_block.is_above_free(),
                                new_size_above,
                                true,
                            );
                        } else {
                            current_block_ptr = match current_block.get_next() {
                                Some(block) => block,
                                None => panic!("Out of kernel heap memory!"),
                            };
                            continue 'block_loop;
                        }
                    }

                    // Create the new block
                    let ptr = current_block as *const _ as usize + core::mem::size_of::<Block>();
                    let new_block_start = ptr + layout_size;
                    let new_block_size =
                        current_block.above_size() - layout_size - core::mem::size_of::<Block>();

                    let next_block = current_block.get_next().unwrap() as *mut Block;

                    current_block.set_above(false);

                    if new_block_size != 0 {
                        current_block.set_above_size(layout_size);

                        let next_block = &mut *next_block;

                        let new_block = &mut *(new_block_start as *mut Block);
                        *new_block = Block::new(layout_size, false, new_block_size, true);

                        next_block.set_below_size(new_block_size);
                        next_block.set_below(false);
                    } else {
                        let next_block = &mut *next_block;
                        next_block.set_below(false);
                    }

                    return ptr as *mut u8;
                }
            }

            current_block_ptr = match current_block.get_next() {
                Some(block) => block,
                None => panic!("Out of kernel heap memory!"),
            }
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        let current_block = &mut *((ptr as usize - core::mem::size_of::<Block>()) as *mut Block);

        if current_block.signature != SIGNATURE {
            panic!("Heap corruption at {:p}", current_block as *mut _);
        }

        if current_block.is_above_free() {
            panic!("Freeing a block of memory that is already free!");
        }

        // Merge previous block
        let current_block = match current_block.get_prev() {
            Some(previous_block) => {
                if previous_block.is_above_free() {
                    let block_size = previous_block.above_size()
                        + current_block.above_size()
                        + core::mem::size_of::<Block>();
                    previous_block.set_above_size(block_size);
                    previous_block
                } else {
                    current_block
                }
            }
            None => current_block,
        };

        let next_block = current_block.get_next().unwrap();

        // Merge next block
        if next_block.is_above_free() {
            let block_size = current_block.above_size()
                + next_block.above_size()
                + core::mem::size_of::<Block>();
            current_block.set_above_size(block_size);
            let next_block = current_block.get_next().unwrap();
            next_block.set_below_size(block_size);
        } else {
            next_block.set_below(true);
            next_block.set_below_size(current_block.above_size());
        }

        // Set current block free
        current_block.set_above(true);
    }
}
