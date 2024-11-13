use crate::classfile::MemberInfo;
use std::rc::Weak;

use super::{access_flag::AccessFlags, class::Class, SharedClass};

#[derive(Debug, Clone)]
pub struct ClassMember {
    pub accessFlags: u16,
    pub name: String,
    pub descriptor: String,
    pub class: SharedClass
}

impl ClassMember {
    pub(super) fn new(memberinfo: &MemberInfo, class: SharedClass) -> ClassMember {
        return ClassMember {
            accessFlags: memberinfo.access_flags,
            name: memberinfo.name().to_string(),
            descriptor: memberinfo.descriptor().to_string(),
            class: class,
        };
    }

    pub fn is_static(&self) -> bool {
        self.accessFlags & AccessFlags::ACC_STATIC.bits() != 0
    }

    pub fn is_public(&self) -> bool {
        self.accessFlags & AccessFlags::ACC_PUBLIC.bits() != 0
    }

    pub fn is_final(&self) -> bool {
        self.accessFlags & AccessFlags::ACC_FINAL.bits() != 0
    }
}

impl PartialEq for ClassMember {
    fn eq(&self, other: &Self) -> bool {
        let flag = self.accessFlags == other.accessFlags
            && self.name == other.name
            && self.descriptor == other.descriptor;
        flag
    }
}
