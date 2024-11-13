use bytes::Buf;

use crate::classfile::{self, class_reader::ClassReader, ClassFile};

use super::ConstantInfoRead;

#[derive(Debug)]
pub(crate) struct ClassInfo {
    pub(crate) name_index: u16,
}

impl ClassInfo {
    pub fn name<'a>(self: &Self, cp: &'a classfile::ConstantPool) -> &'a str {
        cp.get_utf8(self.name_index)
    }
}

impl ConstantInfoRead for ClassInfo {
    fn read_info(reader: &mut ClassReader) -> Self {
        ClassInfo {
            name_index: reader.get_u16(),
        }
    }
}
