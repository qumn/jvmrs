use std::ops::Deref;

use crate::classfile::{self, ClassInfo};

use super::{constant_pool::ConstantPool, cp_symref::SymRef, SharedConstantPool};

#[derive(Debug)]
pub struct ClassRef {
    symRef: SymRef,
}

impl ClassRef {
    pub fn new(cp: SharedConstantPool, class_name: String, classInfo: &ClassInfo) -> ClassRef {
        let symRef = SymRef::new(cp, class_name, classInfo);
        ClassRef { symRef }
    }
}

impl Deref for ClassRef {
    type Target = SymRef;

    fn deref(&self) -> &Self::Target {
        &self.symRef
    }
}
