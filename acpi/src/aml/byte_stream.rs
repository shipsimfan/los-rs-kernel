use super::Result;

pub(super) struct ByteStream<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> ByteStream<'a> {
    pub(super) fn new(bytes: &'a [u8]) -> Self {
        ByteStream { bytes, offset: 0 }
    }

    pub(super) fn peek(&self) -> Option<u8> {
        self.bytes.get(self.offset).map(|byte| *byte)
    }

    pub(super) fn offset(&self) -> usize {
        self.offset
    }

    pub(super) fn collect(&mut self, amount: usize) -> Result<&'a [u8]> {
        let end = self.offset + amount;
        if end > self.bytes.len() {
            return Err(super::Error::UnexpectedEndOfStream);
        }

        let ret = &self.bytes[self.offset..end];
        self.offset = end;
        Ok(ret)
    }
}

impl<'a> Iterator for ByteStream<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.peek();
        self.offset += 1;
        ret
    }
}
