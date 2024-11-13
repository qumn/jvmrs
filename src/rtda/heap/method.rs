use std::{ops::Deref, rc::Weak};

use bytes::Bytes;

use crate::classfile::MemberInfo;

use super::{class::Class, class_member::ClassMember, SharedClass};

#[derive(Debug, Clone, PartialEq)]
pub struct Method {
    pub classMember: ClassMember,
    pub max_stack: u16,
    pub max_locals: u16,
    pub code: Bytes,
}

impl Method {
    pub(super) fn newMethods(class: SharedClass, memberinfos: Vec<&MemberInfo>) -> Vec<Method> {
        let mut ret = Vec::with_capacity(memberinfos.len());
        for mi in memberinfos {
            let cm = ClassMember::new(mi, class.clone());
            let codeAttr = mi.code_attribute();
            let method = match codeAttr {
                Some(codeAttr) => Method {
                    classMember: cm,
                    max_stack: codeAttr.max_stack,
                    max_locals: codeAttr.max_locals,
                    code: codeAttr.code.clone(),
                },
                _ => Method {
                    classMember: cm,
                    max_stack: 0,
                    max_locals: 0,
                    code: Bytes::new(),
                },
            };
            ret.push(method);
        }

        return ret;
    }
}

impl Deref for Method {
    type Target = ClassMember;

    fn deref(&self) -> &Self::Target {
        &self.classMember
    }
}
