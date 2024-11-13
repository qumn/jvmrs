use std::ops::Deref;

use bitflags::Flags;
use tracing::debug;

use crate::classfile::MemberInfo;

use super::{access_flag::AccessFlags, class::Class, class_member::ClassMember, SharedClass};

#[derive(Clone, Debug, PartialEq)]
pub struct Field {
    classMember: ClassMember,
    pub constValueIndex: u16,
    pub slotId: usize,
}

impl Field {
    pub fn newFields(class: SharedClass, memberInfos: Vec<&MemberInfo>) -> Vec<Field> {
        let mut fields = Vec::with_capacity(memberInfos.len());
        for mi in memberInfos {
            let cm = ClassMember::new(mi, class.clone());
            let field = Field {
                classMember: cm,
                slotId: 0,
                constValueIndex: get_const_value_index(mi),
            };
            fields.push(field);
        }
        debug!("init fields of class {}: {:?}", class.name, fields);
        fields
    }

    pub fn is_long_or_double(&self) -> bool {
        self.classMember.descriptor == "J" || self.classMember.descriptor == "D"
    }

    pub fn is_accessible_to(&self, class: &Class) -> bool {
        return true;
    }
}

impl Deref for Field {
    type Target = ClassMember;

    fn deref(&self) -> &Self::Target {
        &self.classMember
    }
}

fn get_const_value_index(memberInfo: &MemberInfo) -> u16 {
    memberInfo
        .constant_value_attribut()
        .map_or(0, |c| c.constant_value_index())
}
