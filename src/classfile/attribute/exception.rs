use bytes::Buf;

use crate::classfile::ClassReader;


pub(crate) struct ExceptionsAttribute {
    exception_index_table: Vec<u16>,
}
impl ExceptionsAttribute {
    pub(crate) fn new(reader: &mut ClassReader) -> Self {
        let number_of_exceptions = reader.get_u16();
        let mut exception_index_table = Vec::with_capacity(number_of_exceptions as usize);
        for _ in 0..number_of_exceptions {
            let exception_index = reader.get_u16();
            exception_index_table.push(exception_index);
        }
        ExceptionsAttribute {
            exception_index_table,
        }
    }
}