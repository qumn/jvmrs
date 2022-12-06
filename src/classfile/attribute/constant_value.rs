use bytes::Buf;

use crate::classfile::ClassReader;



pub(crate) struct ConstantValueAttribute {
    constant_value_index: u16,
}

impl ConstantValueAttribute {
    pub(crate) fn new(read: &mut ClassReader) -> Self {
        let constant_value_index = read.get_u16();
        ConstantValueAttribute {
            constant_value_index,
        }
    }
    pub(crate) fn constant_value_index(&self) -> u16 {
        self.constant_value_index
    }
}