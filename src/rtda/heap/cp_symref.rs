use core::panic;
use std::{
    cell::UnsafeCell,
    rc::{Rc, Weak},
};

use crate::classfile::ClassInfo;

use super::{
    class::{Class, SharedClass},
    constant_pool::ConstantPool,
    SharedConstantPool,
};

#[derive(Debug)]
pub struct SymRef {
    pub cp: SharedConstantPool,
    pub class_name: String,
    class: UnsafeCell<Option<SharedClass>>,
}

impl SymRef {
    pub(super) fn new(cp: SharedConstantPool, class_name: String, classInfo: &ClassInfo) -> SymRef {
        SymRef {
            cp,
            class_name,
            class: UnsafeCell::new(None),
        }
    }

    pub(super) fn from_class_name(cp: SharedConstantPool, className: &str) -> SymRef {
        SymRef {
            cp,
            class_name: className.to_string(),
            class: UnsafeCell::new(None),
        }
    }

    pub fn resolve_class(&self) -> SharedClass {
        let class = self.get_class_mut();
        if class.is_none() {
            class.replace(self.resolve_classref());
        }
        class.as_ref().unwrap().clone()
    }

    fn resolve_classref(&self) -> SharedClass {
        let cp = &self.cp;
        let class = &cp.class;
        let loader = class.loader.upgrade().unwrap();
        let loaded_class = loader.load_class(&self.class_name);
        if !loaded_class.is_accessible_to(&class) {
            panic!("java.lang.IllegalAccessError")
        }
        loaded_class
    }

    fn get_class_mut(&self) -> &mut Option<SharedClass> {
        unsafe { &mut *self.class.get() }
    }
}
