use crate::classfile::{self, InterfaceMethodrefInfo};

use super::{cp_memberref::MemberRef, method::Method};

struct InterfaceMethodRef {
    memberRef: MemberRef,
    method: Option<Method>,
}

impl InterfaceMethodRef {
    fn new(
        cp: &classfile::ConstantPool,
        refInfo: &classfile::InterfaceMethodrefInfo,
    ) -> InterfaceMethodRef {
        let memberRef = MemberRef::new(cp, refInfo);
        InterfaceMethodRef {
            memberRef,
            method: None,
        }
    }

    fn method(&mut self) -> &Method {
        if self.method.is_none() {
            self.method = Some(self.resolveMethod())
        }
        self.method.as_ref().unwrap()
    }

    fn resolveMethod(&self) -> Method {
        todo!()
    }
}
