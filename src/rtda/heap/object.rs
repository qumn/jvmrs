use std::{cell::RefCell, rc::{Rc, Weak}, sync::Mutex};

use crate::rtda::slot::SlotVec;

use super::class::{Class, SharedClass};

pub type SharedObject = Option<Rc<Box<Object>>>;

#[derive(Clone, Debug)]
pub struct Object {
    pub class: SharedClass,
    pub fields: SlotVec,
}

impl Object {
    pub fn new_object(class: SharedClass) -> SharedObject {
        let instanceSlot = class.instanceSlotCount;
        Some(Rc::new(Box::new(Object {
            class: class.clone(),
            fields: SlotVec::new(instanceSlot),
        })))
    }

    pub fn is_instance_of(&self, class: SharedClass) -> bool {
       class.is_assignable_from(&self.class)
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.class.name == other.class.name
    }
}