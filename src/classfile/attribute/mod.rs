use bytes::Buf;

mod code;
mod line_number;
mod marks;
mod source_file;
mod unparsed;
mod exception;
mod constant_value;

use self::{
    code::CodeAttribute,
    line_number::LineNumberTableAttribute,
    marks::{DeprecatedAttribute, SyntheticAttribute},
    source_file::SourceFileAttribute,
    unparsed::UnparsedAttribute, exception::ExceptionsAttribute, constant_value::ConstantValueAttribute,
};

use super::{class_reader::ClassReader, constant::ConstantPool};

pub(crate) fn read_attributes(reader: &mut ClassReader, cp: ConstantPool) -> Vec<AttributeInfo> {
    let attributes_count = reader.get_u16();
    let mut attributes = Vec::with_capacity(attributes_count as usize);
    for _ in 0..attributes_count {
        let attr_info = AttributeInfo::new(reader, cp.clone());
        attributes.push(attr_info);
    }
    attributes
}

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