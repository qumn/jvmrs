use bytes::{Buf, Bytes};

use super::{class_reader::ClassReader, constant::ConstantPool};

pub(crate) enum AttributeInfo {
    Code(CodeAttribute),
    Deprecated(DeprecatedAttribute),
    Synthetic(SyntheticAttribute),
    ConstantValue(ConstantValueAttribute),
    Exceptions(ExceptionsAttribute),
    //InnerClasses(InnerClassesAttribute),
    SourceFile(SourceFileAttribute),
    LineNumberTable(LineNumberTableAttribute),
    //LocalVariableTable(LocalVariableTableAttribute),
    Unknown(UnparsedAttribute),
}

impl AttributeInfo {
    fn new(reader: &mut ClassReader, cp: ConstantPool) -> Self {
        let attr_name_index = reader.get_u16();
        let attr_name = cp.get_utf8(attr_name_index);
        let attr_len = reader.get_u32();
        match attr_name {
            "Code" => {
                let code_attr = CodeAttribute::new(reader, cp);
                AttributeInfo::Code(code_attr)
            }
            "Deprecated" => {
                let deprecated_attr = DeprecatedAttribute {};
                AttributeInfo::Deprecated(deprecated_attr)
            }
            "Synthetic" => {
                let synthetic_attr = SyntheticAttribute {};
                AttributeInfo::Synthetic(synthetic_attr)
            }
            "ConstantValue" => {
                let constant_value_attr = ConstantValueAttribute::new(reader);
                AttributeInfo::ConstantValue(constant_value_attr)
            }
            "Exceptions" => {
                let exceptions_attr = ExceptionsAttribute::new(reader);
                AttributeInfo::Exceptions(exceptions_attr)
            }
            "SourceFile" => {
                let source_file_attr = SourceFileAttribute::new(reader, cp);
                AttributeInfo::SourceFile(source_file_attr)
            }
            "LineNumberTable" => {
                let line_number_table_attr = LineNumberTableAttribute::new(reader);
                AttributeInfo::LineNumberTable(line_number_table_attr)
            }
            _ => {
                let unparsed_attr = UnparsedAttribute::new(reader, attr_name, attr_len);
                AttributeInfo::Unknown(unparsed_attr)
            }
        }
    }
}

pub(crate) fn read_attributes(reader: &mut ClassReader, cp: ConstantPool) -> Vec<AttributeInfo> {
    let attributes_count = reader.get_u16();
    let mut attributes = Vec::with_capacity(attributes_count as usize);
    for _ in 0..attributes_count {
        let attr_info = AttributeInfo::new(reader, cp.clone());
        attributes.push(attr_info);
    }
    attributes
}

pub(crate) struct CodeAttribute {
    cp: ConstantPool,
    max_stack: u16,
    max_locals: u16,
    code: Bytes,
    exception_table: Vec<ExceptionTableEntry>,
    attributes: Vec<AttributeInfo>,
}
impl CodeAttribute {
    fn new(reader: &mut ClassReader, cp: ConstantPool) -> Self {
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

pub(crate) struct UnparsedAttribute {
    name: String,
    len: u32,
    info: Bytes,
}
impl UnparsedAttribute {
    fn new(reader: &mut ClassReader, name: &str, attr_len: u32) -> Self {
        let info = reader.copy_to_bytes(attr_len as usize);
        UnparsedAttribute {
            name: name.to_string(),
            len: attr_len,
            info,
        }
    }
}

trait MarkerAttribute {}
pub(crate) struct DeprecatedAttribute {}
impl MarkerAttribute for DeprecatedAttribute {}
pub(crate) struct SyntheticAttribute {}
impl MarkerAttribute for SyntheticAttribute {}
pub(crate) struct ConstantValueAttribute {
    constant_value_index: u16,
}
impl ConstantValueAttribute {
    fn new(read: &mut ClassReader) -> Self {
        let constant_value_index = read.get_u16();
        ConstantValueAttribute {
            constant_value_index,
        }
    }
    pub(crate) fn constant_value_index(&self) -> u16 {
        self.constant_value_index
    }
}

pub(crate) struct ExceptionsAttribute {
    exception_index_table: Vec<u16>,
}
impl ExceptionsAttribute {
    fn new(reader: &mut ClassReader) -> Self {
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

pub(crate) struct SourceFileAttribute {
    cp: ConstantPool,
    source_file_index: u16,
}
impl SourceFileAttribute {
    fn new(reader: &mut ClassReader, cp: ConstantPool) -> Self {
        let source_file_index = reader.get_u16();
        SourceFileAttribute {
            cp,
            source_file_index,
        }
    }
}

pub(crate) struct LineNumberTableAttribute {
    line_number_table: Vec<LinenumberTableEntry>,
}
pub(crate) struct LinenumberTableEntry {
    start_pc: u16,
    line_number: u16,
}
impl LinenumberTableEntry {
    fn new(reader: &mut ClassReader) -> Self {
        let start_pc = reader.get_u16();
        let line_number = reader.get_u16();
        LinenumberTableEntry {
            start_pc,
            line_number,
        }
    }
}

impl LineNumberTableAttribute {
    fn new(reader: &mut ClassReader) -> Self {
        let line_number_table_length = reader.get_u16();
        let mut line_number_table = Vec::with_capacity(line_number_table_length as usize);
        for _ in 0..line_number_table_length {
            line_number_table.push(LinenumberTableEntry::new(reader));
        }
        LineNumberTableAttribute { line_number_table }
    }
}
