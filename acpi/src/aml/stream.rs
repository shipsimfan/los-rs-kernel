use super::{Error, Result};

pub(super) struct Stream<'a> {
    bytes: &'a [u8],
    offset: usize,
    base_offset: usize,
}

impl<'a> Stream<'a> {
    pub(super) fn new(bytes: &'a [u8], base_offset: usize) -> Self {
        Stream {
            bytes,
            offset: 0,
            base_offset,
        }
    }

    pub(super) fn peek(&self) -> Option<u8> {
        self.bytes.get(self.offset).map(|byte| *byte)
    }

    pub(super) fn offset(&self) -> usize {
        self.offset + self.base_offset
    }

    pub(super) fn collect(&mut self, amount: usize) -> Result<&'a [u8]> {
        let end = self.offset + amount;
        if end > self.bytes.len() {
            return Err(Error::unexpected_end_of_stream(self.bytes.len()));
        }

        let ret = &self.bytes[self.offset..end];
        self.offset = end;
        Ok(ret)
    }

    pub(super) fn collect_to_stream(&mut self, amount: usize) -> Result<Stream<'a>> {
        let base_offset = self.offset();
        self.collect(amount)
            .map(|bytes| Stream::new(bytes, base_offset))
    }
}

impl<'a> Iterator for Stream<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.peek();
        self.offset += 1;
        ret
    }
}
