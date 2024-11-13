use std::collections::btree_set::SymmetricDifference;

use crate::classfile::{self, MemberrefInfo};

use super::{constant_pool::ConstantPool, cp_symref::SymRef, SharedConstantPool};

#[derive(Debug)]
pub struct MemberRef {
    pub symRef: SymRef,
    pub name: String,
    pub descriptor: String,
}

impl MemberRef {
    pub(super) fn new(cp: SharedConstantPool, class_name: &str, name: &str, descriptor: &str, memberRef: &MemberrefInfo) -> MemberRef {
        let symRef = SymRef::from_class_name(cp, &class_name);
        MemberRef {
            symRef,
            name: name.to_string(),
            descriptor: descriptor.to_string(),
        }
    }
}
