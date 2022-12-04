use core::panic;

use bytes::Buf;

use super::{ConstantInfoRead, ConstantPool};
use crate::classfile::class_reader::ClassReader;

#[derive(Debug)]
pub(crate) struct StringInfo {
    pub(crate) string_index: u16,
}
impl ConstantInfoRead for StringInfo {
    fn read_info(reader: &mut ClassReader) -> Self {
        StringInfo {
            string_index: reader.get_u16(),
        }
    }
}
impl StringInfo {
    pub(crate) fn get_string<'a>(&self, cp: &'a ConstantPool) -> &'a str {
        let utf8_info = cp.get_constant(self.string_index);
        match utf8_info {
            super::ConstantInfo::Utf8Info(val) => &val.str,
            _ => {
                panic!("get_string: not a utf8_info");
            }
        }
    }
}

#[derive(Debug)]
pub(crate) struct Utf8Info {
    pub(crate) str: String,
}
impl ConstantInfoRead for Utf8Info {
    fn read_info(reader: &mut ClassReader) -> Self {
        let length = reader.get_u16() as usize;
        let bytes = reader.copy_to_bytes(length);
        Utf8Info {
            str: String::from_utf8(bytes.to_vec()).unwrap(),
        }
    }
}
