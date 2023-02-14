pub(crate) const NUM_VECTORS: usize = 256;

pub(crate) const NUM_EXCEPTIONS: usize = 32;

pub(crate) const FIRST_IRQ: usize = NUM_EXCEPTIONS;
pub(crate) const NUM_IRQS: usize = NUM_VECTORS - NUM_EXCEPTIONS;
