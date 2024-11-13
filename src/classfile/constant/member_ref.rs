use bytes::Buf;

use crate::classfile::{self, class_reader::ClassReader};

use super::ConstantInfoRead;
trait MemberrefInfoTrait {}

#[derive(Debug)]
pub(crate) struct MemberrefInfo {
    pub(crate) class_index: u16,
    pub(crate) name_and_type_index: u16,
}
impl ConstantInfoRead for MemberrefInfo {
    fn read_info(reader: &mut ClassReader) -> Self {
        MemberrefInfo {
            class_index: reader.get_u16(),
            name_and_type_index: reader.get_u16(),
        }
    }
}

pub(crate) type FieldrefInfo = MemberrefInfo;
pub(crate) type MethodrefInfo = MemberrefInfo;
pub(crate) type InterfaceMethodrefInfo = MemberrefInfo;
