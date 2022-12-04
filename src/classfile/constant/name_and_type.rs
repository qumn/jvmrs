use bytes::Buf;

use crate::classfile::class_reader::ClassReader;

use super::ConstantInfoRead;

#[derive(Debug)]
pub(crate) struct NameAndTypeInfo {
    pub(crate) name_index: u16,
    pub(crate) descriptor_index: u16,
}

impl ConstantInfoRead for NameAndTypeInfo {
    fn read_info(reader: &mut ClassReader) -> Self {
        NameAndTypeInfo {
            name_index: reader.get_u16(),
            descriptor_index: reader.get_u16(),
        }
    }
}
