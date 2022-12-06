use bytes::{Bytes, Buf};

use crate::classfile::ClassReader;

pub(crate) struct UnparsedAttribute {
    name: String,
    len: u32,
    info: Bytes,
}
impl UnparsedAttribute {
    pub(crate) fn new(reader: &mut ClassReader, name: &str, attr_len: u32) -> Self {
        let info = reader.copy_to_bytes(attr_len as usize);
        UnparsedAttribute {
            name: name.to_string(),
            len: attr_len,
            info,
        }
    }
}