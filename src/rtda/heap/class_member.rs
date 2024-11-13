use crate::classfile::MemberInfo;
use std::rc::Weak;

use super::class::Class;

pub(super) struct ClassMember {
    accessFlags: u16,
    name: String,
    descriptor: String,
    class: Weak<Class>,
}

impl ClassMember {
    pub(super) fn new(memberinfo: &MemberInfo, class: &Class) -> ClassMember {
        let class = unsafe { Weak::from_raw(class as *const _) };
        return ClassMember {
            accessFlags: memberinfo.access_flags,
            name: memberinfo.name().to_string(),
            descriptor: memberinfo.descriptor().to_string(),
            class: class,
        };
    }
}
