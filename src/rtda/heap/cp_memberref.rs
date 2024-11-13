use std::collections::btree_set::SymmetricDifference;

use crate::classfile::{self, MemberrefInfo};

use super::cp_symref::SymRef;

pub(super) struct MemberRef {
    symRef: SymRef,
    name: String,
    descriptor: String,
}

impl MemberRef {
    pub(super) fn new(cp: &classfile::ConstantPool, memberRef: &MemberrefInfo) -> MemberRef {
        let class_name = cp.get_class_name(memberRef.class_index).to_string();
        let symRef = SymRef::from_class_name(cp, &class_name);
        let (name, descriptor) = cp.get_name_and_type(memberRef.name_and_type_index);
        MemberRef {
            symRef,
            name: name.to_string(),
            descriptor: descriptor.to_string(),
        }
    }
}
