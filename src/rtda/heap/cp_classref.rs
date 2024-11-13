use crate::classfile::{self, ClassInfo, ConstantPool};

use super::cp_symref::SymRef;

struct ClassRef {
    symRef: SymRef,
}

impl ClassRef {
    fn new(cp: &ConstantPool, classInfo: &ClassInfo) -> ClassRef {
        let symRef = SymRef::new(cp, classInfo);
        ClassRef { symRef }
    }
}
