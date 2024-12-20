use std::rc::Weak;

use bytes::Buf;
use tracing::debug;

pub(crate) use self::{
    attribute::{read_attributes, AttributeInfo},
    class_reader::ClassReader,
    constant::*,
    member_info::{read_members, MemberInfo},
};

mod attribute;
mod class_reader;
mod constant;
mod member_info;
//pub(crate) use class_reader::*;

pub(crate) struct ClassFile {
    pub(crate) minor_version: u16,
    pub(crate) major_version: u16,
    pub(crate) constant_pool: ConstantPool,
    pub(crate) access_flags: u16,
    pub(crate) this_class: u16,
    pub(crate) super_class: u16,
    pub(crate) interfaces: Vec<u16>,
    pub(crate) fields: Vec<MemberInfo>,
    pub(crate) methods: Vec<MemberInfo>,
    pub(crate) attributes: Vec<AttributeInfo>,
}

impl ClassFile {
    pub(crate) fn new(reader: ClassReader) -> Self {
        let mut reader = reader;
        let magic = reader.get_u32();
        if magic != 0xCAFEBABE {
            panic!("java.lang.ClassFormatError: magic!");
        }
        let minor_version = reader.get_u16();
        let major_version = reader.get_u16();
        debug!("minor: {}\t major: {}", minor_version, major_version);
        let constant_pool = ConstantPool::new(&mut reader);
        let access_flags = reader.get_u16();
        let this_class = reader.get_u16();
        let super_class = reader.get_u16();
        let interfaces = read_interfaces(&mut reader);
        let fields = read_members(&mut reader, constant_pool.clone());
        let methods = read_members(&mut reader, constant_pool.clone());
        let attributes = read_attributes(&mut reader, constant_pool.clone());
        ClassFile {
            minor_version,
            major_version,
            constant_pool,
            access_flags,
            this_class,
            super_class,
            interfaces,
            fields,
            methods,
            attributes,
        }
    }

    pub(crate) fn class_name(&self) -> &str {
        self.constant_pool.get_class_name(self.this_class)
    }

    pub(crate) fn super_class_name(&self) -> &str {
        if self.super_class > 0 {
            return self.constant_pool.get_class_name(self.super_class);
        }
        ""
    }

    pub(crate) fn interface_names(&self) -> Vec<String> {
        self.interfaces
            .iter()
            .map(|&cpIdx| self.constant_pool.get_class_name(cpIdx).to_string())
            .collect()
    }
}

fn read_interfaces(reader: &mut ClassReader) -> Vec<u16> {
    let len = reader.get_u16();
    reader.get_u16s(len as usize)
}

impl std::fmt::Debug for ClassFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClassFile")
            .field("minor_version", &self.minor_version)
            .field("major_version", &self.major_version)
            .field("access_flags", &self.access_flags)
            .field("this_class", &self.this_class)
            .field("super_class", &self.super_class)
            .field("interfaces", &self.interfaces)
            .field("fields", &self.fields)
            .field("methods", &self.methods)
            //.field("attributes", &self.attributes)
            .finish()
    }
}
