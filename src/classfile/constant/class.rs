use bytes::Buf;

use crate::classfile::class_reader::ClassReader;

use super::ConstantInfoRead;

#[derive(Debug)]
pub(crate) struct ClassInfo {
    pub(crate) name_index: u16,
}

impl ConstantInfoRead for ClassInfo {
    fn read_info(reader: &mut ClassReader) -> Self {
        ClassInfo {
            name_index: reader.get_u16(),
        }
    }
}
