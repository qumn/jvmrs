use std::ops::{Deref, DerefMut};

use bytes::{Buf, Bytes};

pub(crate) struct ClassReader(Bytes);

impl ClassReader {
    pub(crate) fn new(bytes: Vec<u8>) -> Self {
        ClassReader(Bytes::from(bytes))
    }

    pub(crate) fn get_u16s(&mut self, size: usize) -> Vec<u16> {
        let mut u16s = Vec::with_capacity(size);
        for _ in 0..size {
            u16s.push(self.0.get_u16());
        }
        u16s
    }
}

impl Deref for ClassReader {
    type Target = Bytes;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ClassReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
