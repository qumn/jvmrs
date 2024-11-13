use core::panic;
use std::{path::Iter, sync::Arc};

use bytes::Buf;

pub(crate) use self::{
    class::ClassInfo,
    member_ref::{FieldrefInfo, InterfaceMethodrefInfo, MemberrefInfo, MethodrefInfo},
    name_and_type::NameAndTypeInfo,
    numeric::{DoubleInfo, FloatInfo, IntegerInfo, LongInfo},
    string::{StringInfo, Utf8Info},
};

use super::class_reader::ClassReader;
mod class;
mod member_ref;
mod name_and_type;
mod numeric;
mod string;
mod tag;

// cheap for clone, because it's just contains a Arc pointer
#[derive(Clone, Debug)]
pub(crate) struct ConstantPool {
    constants: Arc<Vec<ConstantInfo>>,
}

impl ConstantPool {
    pub(crate) fn new(mut reader: &mut ClassReader) -> Self {
        let constant_pool_count = reader.get_u16() as usize;
        let mut constants = Vec::with_capacity(constant_pool_count);
        constants.push(ConstantInfo::None);
        let mut i = 1;
        while i < constant_pool_count as i32 {
            let constant_info = ConstantInfo::read_info(&mut reader);
            constants.push(constant_info);
            if let ConstantInfo::LongInfo(_) | ConstantInfo::DoubleInfo(_) =
                constants.last().unwrap()
            {
                constants.push(ConstantInfo::None);
                i += 1;
            }
            i += 1;
        }
        ConstantPool {
            constants: Arc::new(constants),
        }
    }

    pub(crate) fn get_constant(&self, index: u16) -> &ConstantInfo {
        &self.constants[index as usize]
    }

    pub(crate) fn get_name_and_type(&self, index: u16) -> (&str, &str) {
        match self.get_constant(index) {
            ConstantInfo::NameAndTypeInfo(info) => (
                self.get_utf8(info.name_index),
                self.get_utf8(info.descriptor_index),
            ),
            _ => panic!("type error: index {} is not a NameAndTypeInfo", index),
        }
    }
    pub(crate) fn get_class_name(&self, index: u16) -> &str {
        match self.get_constant(index) {
            ConstantInfo::ClassInfo(class_info) => self.get_utf8(class_info.name_index),
            _ => {
                panic!("type error: index {} is not ClassInfo", index)
            }
        }
    }

    pub(crate) fn get_utf8(&self, index: u16) -> &str {
        match self.get_constant(index) {
            ConstantInfo::Utf8Info(info) => info.str.as_str(),
            _ => {
                panic!("type error: index {} is not a Utf8Info", index);
            }
        }
    }

    pub(crate) fn get_int(&self, index: u16) -> i32 {
        match self.get_constant(index) {
            ConstantInfo::IntegerInfo(info) => info.val,
            _ => {
                panic!("type error: index {} is not a IntegerInfo", index);
            }
        }
    }

    pub(crate) fn get_long(&self, index: u16) -> i64 {
        match self.get_constant(index) {
            ConstantInfo::LongInfo(info) => info.val,
            _ => {
                panic!("type error: index {} is not a LongInfo", index);
            }
        }
    }

    pub(crate) fn get_float(&self, index: u16) -> f32 {
        match self.get_constant(index) {
            ConstantInfo::FloatInfo(info) => info.val,
            _ => {
                panic!("type error: index {} is not a FloatInfo", index);
            }
        }
    }

    pub(crate) fn get_double(&self, index: u16) -> f64 {
        match self.get_constant(index) {
            ConstantInfo::DoubleInfo(info) => info.val,
            _ => {
                panic!("type error: index {} is not a DoubleInfo", index);
            }
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.constants.len()
    }

    pub(crate) fn iter(&self) -> std::slice::Iter<'_, ConstantInfo> {
        self.constants.iter()
    }
}

trait ConstantInfoRead {
    fn read_info(reader: &mut ClassReader) -> Self;
}

#[derive(Debug)]
pub(crate) enum ConstantInfo {
    ClassInfo(ClassInfo),
    FieldrefInfo(FieldrefInfo),
    MethodrefInfo(MethodrefInfo),
    InterfaceMethodrefInfo(InterfaceMethodrefInfo),
    StringInfo(StringInfo),
    IntegerInfo(IntegerInfo),
    FloatInfo(FloatInfo),
    LongInfo(LongInfo),
    DoubleInfo(DoubleInfo),
    NameAndTypeInfo(NameAndTypeInfo),
    Utf8Info(Utf8Info),
    None,
    //MethodHandleInfo(MethodHandleInfo),
    //MethodTypeInfo(MethodTypeInfo),
    //InvokeDynamicInfo(InvokeDynamicInfo)
}

impl ConstantInfoRead for ConstantInfo {
    fn read_info(reader: &mut ClassReader) -> Self {
        let tag = reader.get_u8();
        match tag {
            tag::Integer => ConstantInfo::IntegerInfo(IntegerInfo::read_info(reader)),
            tag::Float => ConstantInfo::FloatInfo(FloatInfo::read_info(reader)),
            tag::Long => ConstantInfo::LongInfo(LongInfo::read_info(reader)),
            tag::Double => ConstantInfo::DoubleInfo(DoubleInfo::read_info(reader)),
            tag::Utf8 => ConstantInfo::Utf8Info(Utf8Info::read_info(reader)),
            tag::String => ConstantInfo::StringInfo(StringInfo::read_info(reader)),
            tag::Class => ConstantInfo::ClassInfo(ClassInfo::read_info(reader)),
            tag::Fieldref => ConstantInfo::FieldrefInfo(FieldrefInfo::read_info(reader)),
            tag::Methodref => ConstantInfo::MethodrefInfo(MethodrefInfo::read_info(reader)),
            tag::InterfaceMethodref => {
                ConstantInfo::InterfaceMethodrefInfo(InterfaceMethodrefInfo::read_info(reader))
            }
            tag::NameAndType => ConstantInfo::NameAndTypeInfo(NameAndTypeInfo::read_info(reader)),
            _ => {
                panic!("not support tag of constant: {}", tag);
                //ConstantInfo::None
            }
        }
    }
}
