use std::rc::Weak;

use crate::classfile::{ClassInfo, ConstantPool};

use super::class::Class;

pub(super) struct SymRef {
    cp: Weak<ConstantPool>,
    className: String,
    class: Option<Class>,
}

impl SymRef {
    pub(super) fn new(cp: &ConstantPool, classInfo: &ClassInfo) -> SymRef {
        let cpWk = unsafe { Weak::from_raw(cp as *const _) };
        let className = classInfo.name(cp).to_string();
        SymRef {
            cp: cpWk,
            className,
            class: None,
        }
    }
    pub(super) fn from_class_name(cp: &ConstantPool, className: &str) -> SymRef {
        let cpWk = unsafe { Weak::from_raw(cp as *const _) };
        SymRef {
            cp: cpWk,
            className: className.to_string(),
            class: None,
        }
    }
}
