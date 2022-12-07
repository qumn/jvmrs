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

#[cfg(test)]
mod test {
    use std::{mem};

    const a: usize = 1;
    #[test]
    fn test_get_u16s() {
        let bytes = vec![0x00, 0x01, 0x00, 0x02, 0x00, 0x03];
        let mut reader = super::ClassReader::new(bytes);
        let u16s = reader.get_u16s(3);

        let b = &a as *const usize;
        //let c = size_of::<*const i32>();
        let c = mem::size_of::<*const i32>();
        println!("{:?}", c);
        assert_eq!(u16s, vec![1, 2, 3]);
    }
}
