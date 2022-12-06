use bytes::{Bytes, Buf};

use crate::classfile::{ConstantPool, ClassReader};

use super::{AttributeInfo, read_attributes};


pub(crate) struct CodeAttribute {
    cp: ConstantPool,
    max_stack: u16,
    max_locals: u16,
    code: Bytes,
    exception_table: Vec<ExceptionTableEntry>,
    attributes: Vec<AttributeInfo>,
}

impl CodeAttribute {
    pub(crate) fn new(reader: &mut ClassReader, cp: ConstantPool) -> Self {
        let max_stack = reader.get_u16();
        let max_locals = reader.get_u16();
        let code_len = reader.get_u32();
        let code = reader.copy_to_bytes(code_len as usize);
        let exception_table_len = reader.get_u16();
        let exception_table = Self::read_exception_table(reader, exception_table_len as usize);
        let attributes = read_attributes(reader, cp.clone());
        CodeAttribute {
            cp: cp,
            max_stack,
            max_locals,
            code,
            exception_table,
            attributes,
        }
    }
    fn read_exception_table(reader: &mut ClassReader, len: usize) -> Vec<ExceptionTableEntry> {
        let mut vec = Vec::with_capacity(len);
        for _ in 0..len {
            vec.push(ExceptionTableEntry::new(reader));
        }
        vec
    }
}

struct ExceptionTableEntry {
    start_pc: u16,
    end_pc: u16,
    handler_pc: u16,
    catch_pc: u16,
}
impl ExceptionTableEntry {
    fn new(reader: &mut ClassReader) -> Self {
        let start_pc = reader.get_u16();
        let end_pc = reader.get_u16();
        let handler_pc = reader.get_u16();
        let catch_pc = reader.get_u16();
        ExceptionTableEntry {
            start_pc,
            end_pc,
            handler_pc,
            catch_pc,
        }
    }
}