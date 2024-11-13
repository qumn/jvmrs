use std::rc::Weak;

use bytes::Bytes;

use crate::classfile::MemberInfo;

use super::{class::Class, class_member::ClassMember};

pub(crate) struct Method {
    classMember: ClassMember,
    maxStack: u16,
    maxLocals: u16,
    code: Bytes,
}

impl Method {
    pub(super) fn newMethods(class: &Class, memberinfos: Vec<&MemberInfo>) -> Vec<Method> {
        let wkClass = unsafe { Weak::from_raw(class as *const _) };
        let mut ret = Vec::with_capacity(memberinfos.len());
        for mi in memberinfos {
            let cm = ClassMember::new(mi, class);
            let codeAttr = mi.code_attribute().expect("expect contain code attribute");
            let method = Method {
                classMember: cm,
                maxStack: codeAttr.max_stack,
                maxLocals: codeAttr.max_locals,
                code: codeAttr.code.clone(),
            };
            ret.push(method);
        }

        return ret;
    }
}
