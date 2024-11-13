use crate::classfile::MemberInfo;

use super::{class::Class, class_member::ClassMember};

pub(crate) struct Field {
    classMember: ClassMember,
}

impl Field {
    fn newFields(class: &Class, memberInfos: Vec<&MemberInfo>) -> Vec<Field> {
        return memberInfos
            .iter()
            .map(|&mi| ClassMember::new(mi, class))
            .map(|cm| Field { classMember: cm })
            .collect();
    }
}
