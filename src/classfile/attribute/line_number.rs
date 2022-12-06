use bytes::Buf;

use crate::classfile::ClassReader;
pub(crate) struct LineNumberTableAttribute {
    line_number_table: Vec<LinenumberTableEntry>,
}

impl LineNumberTableAttribute {
    pub(crate) fn new(reader: &mut ClassReader) -> Self {
        let line_number_table_length = reader.get_u16();
        let mut line_number_table = Vec::with_capacity(line_number_table_length as usize);
        for _ in 0..line_number_table_length {
            line_number_table.push(LinenumberTableEntry::new(reader));
        }
        LineNumberTableAttribute { line_number_table }
    }
}

pub(crate) struct LinenumberTableEntry {
    start_pc: u16,
    line_number: u16,
}
impl LinenumberTableEntry {
    pub(crate) fn new(reader: &mut ClassReader) -> Self {
        let start_pc = reader.get_u16();
        let line_number = reader.get_u16();
        LinenumberTableEntry {
            start_pc,
            line_number,
        }
    }
}
