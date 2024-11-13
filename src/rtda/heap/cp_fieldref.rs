use crate::classfile::{self, ConstantPool, FieldrefInfo};

use super::{cp_memberref::MemberRef, field::Field};

struct FieldRef {
    memberRef: MemberRef,
    field: Option<Field>,
}

impl FieldRef {
    fn new(cp: &ConstantPool, refInfo: &classfile::FieldrefInfo) -> FieldRef {
        let memberRef = MemberRef::new(cp, refInfo);
        FieldRef {
            memberRef,
            field: None,
        }
    }

    fn field(&mut self) -> &Field {
        if self.field.is_none() {
            self.field = Some(self.resolveField());
        }
        self.field.as_ref().unwrap()
    }

    fn resolveField(&self) -> Field {
        todo!()
    }
}
