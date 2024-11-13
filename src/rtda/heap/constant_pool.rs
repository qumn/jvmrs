use std::rc::Weak;

use bytes::BufMut;

use crate::classfile::ConstantInfo;
use crate::classfile::{self, ClassFile};

use super::class::Class;

struct ConstantPool {
    class: Weak<Class>,
    consts: Vec<Constant>,
}

impl ConstantPool {
    pub(super) fn new(class: &Class, cfCp: &classfile::ConstantPool) {
        let mut consts = Vec::with_capacity(cfCp.len());
        let mut idx: u16 = 0;
        while idx < cfCp.len() as u16 {
            let ct = Constant::parse(cfCp.get_constant(idx), cfCp);
            consts.push(ct);
        }
    }
}

enum Constant {
    LONG(i64),
    DOUBLE(f64),
    INTEGER(i32),
    FLOAT(f32),
    STRING(String),
    CLASSREF(),
    FEILDREF(),
    METHODREF(),
    INTERFACEREF(),
    // filling for long and double
    None,
}

impl Constant {
    fn parse(ci: &ConstantInfo, cfCp: &classfile::ConstantPool) -> Self {
        match ci {
            ConstantInfo::LongInfo(long_info) => Constant::LONG(long_info.val),
            ConstantInfo::DoubleInfo(double_info) => Constant::DOUBLE(double_info.val),
            ConstantInfo::IntegerInfo(integer_info) => Constant::INTEGER(integer_info.val),
            ConstantInfo::FloatInfo(float_info) => Constant::FLOAT(float_info.val),
            ConstantInfo::StringInfo(string_info) => {
                Constant::STRING(string_info.get_string(cfCp).to_string())
            }
            ConstantInfo::ClassInfo(class_info) => todo!(),
            ConstantInfo::FieldrefInfo(memberref_info) => todo!(),
            ConstantInfo::MethodrefInfo(memberref_info) => todo!(),
            ConstantInfo::InterfaceMethodrefInfo(memberref_info) => todo!(),
            ConstantInfo::NameAndTypeInfo(name_and_type_info) => todo!(),
            ConstantInfo::None => Constant::None,
            _ => {
                todo!()
            }
        }
    }
}
