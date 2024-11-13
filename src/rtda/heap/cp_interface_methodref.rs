use crate::classfile::{self, InterfaceMethodrefInfo};

use super::{class, constant_pool::ConstantPool, cp_memberref::MemberRef, method::Method, SharedConstantPool};

#[derive(Debug)]
pub struct InterfaceMethodRef {
    memberRef: MemberRef,
    method: Option<Method>,
}

impl InterfaceMethodRef {
    pub fn new(
        cp: SharedConstantPool,
        class_name: &str,
        name: &str,
        descriptor: &str,
        refInfo: &classfile::InterfaceMethodrefInfo,
    ) -> InterfaceMethodRef {
        let memberRef = MemberRef::new(cp,class_name, name, descriptor, refInfo);
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
