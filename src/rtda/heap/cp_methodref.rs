use std::ops::Deref;

use crate::classfile::{self, MemberrefInfo, MethodrefInfo};

use super::{class, constant_pool::ConstantPool, cp_memberref::MemberRef, method::Method, SharedConstantPool};

#[derive(Debug)]
pub(crate) struct MethodRef {
    pub memberRef: MemberRef,
    method: Option<Method>,
}

impl MethodRef {
    pub(crate) fn new(cp: SharedConstantPool, class_name: &str, name: &str, descriptor: &str, refInfo: &classfile::MethodrefInfo) -> MethodRef {
        let memberRef = MemberRef::new(cp, class_name, name, descriptor, refInfo);
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


impl Deref for MethodRef {
    type Target = MemberRef;

    fn deref(&self) -> &Self::Target {
        &self.memberRef
    }
}