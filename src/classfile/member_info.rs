use bytes::Buf;

use super::{
    attribute::{read_attributes, AttributeInfo, code::CodeAttribute},
    class_reader::ClassReader,
    constant::ConstantPool,
};

pub(crate) fn read_members(reader: &mut ClassReader, cp: ConstantPool) -> Vec<MemberInfo> {
    let member_len = reader.get_u16();
    let mut members = Vec::with_capacity(member_len as usize);
    for _ in 0..member_len {
        members.push(MemberInfo::new(reader, cp.clone()));
    }
    members
}

pub(crate) struct MemberInfo {
    pub(crate) cp: ConstantPool,
    pub(crate) access_flags: u16,
    pub(crate) name_index: u16,
    pub(crate) descriptor_index: u16,
    pub(crate) attributes: Vec<AttributeInfo>,
}

impl std::fmt::Debug for MemberInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

impl MemberInfo {
    pub(crate) fn new(reader: &mut ClassReader, cp: ConstantPool) -> Self {
        let access_flags = reader.get_u16();
        let name_index = reader.get_u16();
        let descriptor_index = reader.get_u16();
        let attributes = read_attributes(reader, cp.clone());
        MemberInfo {
            cp: cp,
            access_flags,
            name_index,
            descriptor_index,
            attributes,
        }
    }

    pub(crate) fn access_flags(&self) -> u16 {
        self.access_flags
    }

    pub fn name(&self) -> &str {
        self.cp.get_utf8(self.name_index)
    }
    pub fn descriptor(&self) -> &str {
        self.cp.get_utf8(self.descriptor_index)
    }
    pub fn code_attribute(&self) -> Option<&CodeAttribute> {
        for attr in &self.attributes {
            match attr {
                AttributeInfo::Code(code_attr) => return Some(code_attr),
                _ => {}
            }
        }
        None
    }
}
