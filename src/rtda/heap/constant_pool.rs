use core::panic;
use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use bytes::BufMut;
use tracing::debug;

use crate::classfile::{self, ClassFile, FieldrefInfo};
use crate::classfile::{ConstantInfo, InterfaceMethodrefInfo};

use super::class::{self, Class, SharedClass};
use super::cp_classref::{self, ClassRef};
use super::cp_fieldref::FieldRef;
use super::cp_interface_methodref::InterfaceMethodRef;
use super::cp_methodref::MethodRef;

#[derive(Clone, Default, Debug)]
pub struct SharedConstantPool {
    constant_pool: Option<Rc<UnsafeCell<Box<ConstantPool>>>>,
}

impl SharedConstantPool {
    pub fn new(class: SharedClass, cfCp: &classfile::ConstantPool) -> Self {
        let mut scp = SharedConstantPool {
            constant_pool: Some(Rc::new(UnsafeCell::new(Box::new(ConstantPool::new(
                class,
                cfCp.len(),
            ))))),
        };
        scp.init(cfCp);
        scp
    }

    fn init(self: &mut Self, cfCp: &classfile::ConstantPool) {
        let mut idx: u16 = 0;
        while idx < (cfCp.len() as u16) {
            let ct = Constant::parse(cfCp.get_constant(idx), cfCp, &self);
            self.consts.push(ct);
            idx += 1;
        }
        debug!("init constant pool of {}", self.class.name);
    }
}

impl Deref for SharedConstantPool {
    type Target = ConstantPool;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.constant_pool.as_ref().unwrap().get() }
    }
}

impl DerefMut for SharedConstantPool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.constant_pool.as_ref().unwrap().get() }
    }
}

#[derive(Debug)]
pub struct ConstantPool {
    pub class: SharedClass,
    consts: Vec<Constant>,
}

impl ConstantPool {
    fn new(class: SharedClass, size: usize) -> Self {
        let consts = Vec::with_capacity(size);
        ConstantPool { class, consts }
    }

    pub fn get_const(&self, idx: u16) -> &Constant {
        &self.consts[idx as usize]
    }

    pub fn get_const_mut(&mut self, idx: u16) -> &mut Constant {
        &mut self.consts[idx as usize]
    }

    pub fn get_int(&self, idx: u16) -> i32 {
        match self.get_const(idx) {
            Constant::INTEGER(i) => *i,
            _ => panic!("type error: index {} is not a i32", idx),
        }
    }
    pub fn get_float(&self, idx: u16) -> f32 {
        match self.get_const(idx) {
            Constant::FLOAT(f) => *f,
            _ => panic!("type error: index {} is not a f32", idx),
        }
    }
    pub fn get_long(&self, idx: u16) -> i64 {
        match self.get_const(idx) {
            Constant::LONG(l) => *l,
            _ => panic!("type error: index {} is not a i32", idx),
        }
    }
    pub fn get_double(&self, idx: u16) -> f64 {
        match self.get_const(idx) {
            Constant::DOUBLE(d) => *d,
            _ => panic!("type error: index {} is not a i32", idx),
        }
    }

    pub fn get_string(&self, idx: u16) -> &str {
        match self.get_const(idx) {
            Constant::STRING(str) => str,
            _ => panic!("type error: index {} is not a String ", idx),
        }
    }

    pub fn get_class_name(&self, idx: u16) -> &str {
        match self.get_const(idx) {
            Constant::CLASSREF(class_ref) => &class_ref.class_name,
            _ => panic!("type error: index {} is not a ClassRef", idx),
        }
    }

    pub fn get_name_and_type(&self, idx: u16) -> (&str, &str) {
        match self.get_const(idx) {
            Constant::NAMEANDTYPEINFO(name, descriptor) => (name, descriptor),
            _ => panic!("type error: index {} is not a NameAndTypeInfo", idx),
        }
    }

    pub fn get_classref(&self, idx: u16) -> &ClassRef {
        match self.get_const(idx) {
            Constant::CLASSREF(class_ref) => class_ref,
            _ => panic!("type error: index {} is not a ClassRef", idx),
        }
    }

    pub fn get_field_ref(&self, idx: u16) -> &FieldRef {
        match self.get_const(idx) {
            Constant::FIELDREF(field_ref) => field_ref,
            _ => panic!("type error: index {} is not a FieldRef", idx),
        }
    }

    pub fn get_field_ref_mut(&mut self, idx: u16) -> &mut FieldRef {
        match self.get_const_mut(idx) {
            Constant::FIELDREF(field_ref) => field_ref,
            _ => panic!("type error: index {} is not a FieldRef", idx),
        }
    }

    pub fn get_method_ref(&self, idx: u16) -> &MethodRef {
        match self.get_const(idx) {
            Constant::METHODREF(method_ref) => method_ref,
            _ => panic!("type error: index {} is not a method_ref", idx),
        }
    }
}

#[derive(Debug)]
pub enum Constant {
    LONG(i64),
    DOUBLE(f64),
    INTEGER(i32),
    FLOAT(f32),
    STRING(String),
    CLASSREF(ClassRef),
    FIELDREF(FieldRef),
    METHODREF(MethodRef),
    INTERFACEMETHODREF(InterfaceMethodRef),
    NAMEANDTYPEINFO(String, String),
    // filling for long and double
    None,
}

impl Constant {
    fn parse(ci: &ConstantInfo, cfCp: &classfile::ConstantPool, cp: &SharedConstantPool) -> Self {
        match ci {
            ConstantInfo::LongInfo(long_info) => Constant::LONG(long_info.val),
            ConstantInfo::DoubleInfo(double_info) => Constant::DOUBLE(double_info.val),
            ConstantInfo::IntegerInfo(integer_info) => Constant::INTEGER(integer_info.val),
            ConstantInfo::FloatInfo(float_info) => Constant::FLOAT(float_info.val),
            ConstantInfo::StringInfo(string_info) => {
                Constant::STRING(string_info.get_string(cfCp).to_string())
            }
            ConstantInfo::ClassInfo(class_info) => {
                let class_name = cfCp.get_utf8(class_info.name_index).to_string();
                let class_ref = ClassRef::new(cp.clone(), class_name, class_info);
                Constant::CLASSREF(class_ref)
            }
            ConstantInfo::FieldrefInfo(field_ref_info) => {
                let class_name = cfCp.get_class_name(field_ref_info.class_index);
                let (name, descriptor) = cfCp.get_name_and_type(field_ref_info.name_and_type_index);
                let field_ref =
                    FieldRef::new(cp.clone(), class_name, name, descriptor, field_ref_info);
                Constant::FIELDREF(field_ref)
            }
            ConstantInfo::MethodrefInfo(method_ref_info) => {
                let class_name = cfCp.get_class_name(method_ref_info.class_index);
                let (name, descriptor) =
                    cfCp.get_name_and_type(method_ref_info.name_and_type_index);
                let method_ref =
                    MethodRef::new(cp.clone(), class_name, name, descriptor, method_ref_info);
                Constant::METHODREF(method_ref)
            }
            ConstantInfo::InterfaceMethodrefInfo(interface_method_ref_info) => {
                let class_name = cfCp.get_class_name(interface_method_ref_info.class_index);
                let (name, descriptor) =
                    cfCp.get_name_and_type(interface_method_ref_info.name_and_type_index);
                let interface_method_ref = InterfaceMethodRef::new(
                    cp.clone(),
                    class_name,
                    name,
                    descriptor,
                    interface_method_ref_info,
                );
                Constant::INTERFACEMETHODREF(interface_method_ref)
            }
            ConstantInfo::None => Constant::None,
            ConstantInfo::Utf8Info(utf8_info) => Constant::STRING(utf8_info.str.to_string()),
            ConstantInfo::NameAndTypeInfo(name_and_type_info) => {
                let name = cfCp.get_utf8(name_and_type_info.name_index).to_string();
                let descriptor = cfCp
                    .get_utf8(name_and_type_info.descriptor_index)
                    .to_string();
                Constant::NAMEANDTYPEINFO(name, descriptor)
            }
        }
    }
}
