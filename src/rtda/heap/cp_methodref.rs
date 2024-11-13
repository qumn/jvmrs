use crate::classfile::{self, ConstantPool, MethodrefInfo};

use super::{cp_memberref::MemberRef, method::Method};

pub(crate) struct MethodRef {
    memberRef: MemberRef,
    method: Option<Method>,
}

impl MethodRef {
    pub(crate) fn new(cp: &ConstantPool, refInfo: &classfile::MethodrefInfo) -> MethodRef {
        let memberRef = MemberRef::new(cp, refInfo);
        MethodRef {
            memberRef,
            method: None,
        }
    }
    pub(crate) fn method(&mut self) -> &Method {
        if self.method.is_none() {
            self.method = Some(self.resolveMethod())
        }
        (self.method.as_ref()).unwrap()
    }

    fn resolveMethod(&self) -> Method {
        todo!()
    }
}
