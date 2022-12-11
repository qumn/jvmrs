use std::{cmp, mem, ptr};
pub(crate) struct BytecodeReader {
    pub(crate) pc: usize,
    code: Vec<u8>,
}

macro_rules! buf_get_impl {
    ($this:ident, $typ:tt::$conv:tt) => {{
        const SIZE: usize = mem::size_of::<$typ>();
        // try to convert directly from the bytes
        // this Option<ret> trick is to avoid keeping a borrow on self
        // when advance() is called (mut borrow) and to call bytes() only once
        let ret = $this.code[$this.pc..]
            .get(..SIZE)
            .map(|src| unsafe { $typ::$conv(*(src as *const _ as *const [_; SIZE])) });

        if let Some(ret) = ret {
            // if the direct conversion was possible, advance and return
            $this.advance(SIZE);
            return ret;
        } else {
            // if not we copy the bytes in a temp buffer then convert
            let mut buf = [0; SIZE];
            $this.copy_to_slice(&mut buf); // (do the advance)
            return $typ::$conv(buf);
        }
    }};
    (be => $this:ident, $typ:tt, $len_to_read:expr) => {{
        debug_assert!(mem::size_of::<$typ>() >= $len_to_read);

        let mut buf = [0; (mem::size_of::<$typ>())];
        $this.copy_to_slice(&mut buf[mem::size_of::<$typ>() - ($len_to_read)..]);
        return $typ::from_be_bytes(buf);
    }};
}

impl BytecodeReader {
    pub(crate) fn new(code: Vec<u8>) -> Self {
        BytecodeReader { code, pc: 0 }
    }
    pub(crate) fn read_i8(&mut self) -> i8 {
        buf_get_impl!(self, i8::from_be_bytes)
    }
    pub(crate) fn read_u8(&mut self) -> u8 {
        buf_get_impl!(self, u8::from_be_bytes)
    }
    pub(crate) fn read_i16(&mut self) -> i16 {
        buf_get_impl!(self, i16::from_be_bytes)
    }
    pub(crate) fn read_u16(&mut self) -> u16 {
        buf_get_impl!(self, u16::from_be_bytes)
    }
    pub(crate) fn read_i32(&mut self) -> i32 {
        buf_get_impl!(self, i32::from_be_bytes)
    }
    pub(crate) fn read_i32s(&mut self, size: usize) -> Vec<i32> {
        let mut res = vec![0; size];
        for i in 0..size{
            res[i] = self.read_i32();
        }
        return res
    }
    pub(crate) fn read_u32(&mut self) -> u32 {
        buf_get_impl!(self, u32::from_be_bytes)
    }
    pub(crate) fn read_u32s(&mut self, size: usize) -> Vec<u32> {
        let mut res = vec![0; size];
        for i in 0..size{
            res[i] = self.read_u32();
        }
        return res
    }
    pub(crate) fn read_i64(&mut self) -> i64 {
        buf_get_impl!(self, i64::from_be_bytes)
    }
    pub(crate) fn read_u64(&mut self) -> u64 {
        buf_get_impl!(self, u64::from_be_bytes)
    }
    pub(crate) fn read_f32(&mut self) -> f32 {
        buf_get_impl!(self, f32::from_be_bytes)
    }
    pub(crate) fn read_f64(&mut self) -> f64 {
        buf_get_impl!(self, f64::from_be_bytes)
    }
    fn remaining(&self) -> usize {
        self.code.len() - self.pc
    }
    pub(crate) fn reset(&mut self, pc: usize) {
        self.pc = pc;
    }
    fn advance(&mut self, size: usize) {
        self.pc += size
    }
    pub(crate) fn skip_padding(&mut self){
        while self.pc % 4 != 0 {
            self.read_u8();
        }
    }
    fn copy_to_slice(&mut self, dst: &mut [u8]) {
        let mut off = 0;

        assert!(self.remaining() >= dst.len());

        while off < dst.len() {
            let cnt;

            unsafe {
                let src = &self.code[self.pc..];
                cnt = cmp::min(src.len(), dst.len() - off);

                ptr::copy_nonoverlapping(src.as_ptr(), dst[off..].as_mut_ptr(), cnt);

                off += cnt;
            }
            self.advance(cnt);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bytecodeReader_should_work() {
        let mut reader = BytecodeReader::new(vec![
            0x12, 0x34, 0x56, 0x78, 0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef, 64, 5, 191, 10,
            139, 20, 7, 217, 64, 73, 15, 218,
        ]);
        let double: f64 = 2.71828182845;
        let float: f32 = 3.1415926;
        assert_eq!(reader.read_i32(), 0x12345678);
        assert_eq!(reader.read_i64(), 0x1234567890abcdef);
        assert_eq!(reader.read_f64(), double);
        assert_eq!(reader.read_f32(), float)
    }
}
