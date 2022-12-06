use bytes::Buf;

use crate::classfile::{ClassReader, ConstantPool};

pub(crate) struct SourceFileAttribute {
    cp: ConstantPool,
    source_file_index: u16,
}
impl SourceFileAttribute {
    pub(crate) fn new(reader: &mut ClassReader, cp: ConstantPool) -> Self {
        let source_file_index = reader.get_u16();
        SourceFileAttribute {
            cp,
            source_file_index,
        }
    }
}
