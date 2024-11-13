use std::{cell::RefCell, ptr, rc::Rc};

use super::heap::{Object, SharedObject};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Slot {
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Ref(SharedObject),
}

impl Slot {
    pub(crate) fn null() -> Self {
        Slot::Ref(None)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SlotVec {
    vars: RefCell<Vec<Slot>>,
}

impl SlotVec {
    pub(crate) fn new(max_locals: usize) -> Self {
        SlotVec {
            vars: RefCell::new(vec![Slot::null(); max_locals]),
        }
    }
    pub(crate) fn set_int(&self, index: usize, val: i32) {
        self.vars.borrow_mut()[index] = Slot::Int(val);
    }

    pub(crate) fn get_int(&self, index: usize) -> i32 {
        match self.vars.borrow()[index] {
            Slot::Int(val) => val,
            _ => panic!("type mismatch"),
        }
    }

    pub(crate) fn set_float(&self, index: usize, val: f32) {
        self.vars.borrow_mut()[index] = Slot::Float(val);
    }
    pub(crate) fn get_float(&self, index: usize) -> f32 {
        match self.vars.borrow()[index] {
            Slot::Float(val) => val,
            _ => panic!("type mismatch"),
        }
    }

    pub(crate) fn set_long(&self, index: usize, val: i64) {
        self.vars.borrow_mut()[index] = Slot::Long(val);
        // long consumes two slots
        self.vars.borrow_mut()[index + 1] = Slot::null()
    }
    pub(crate) fn get_long(&self, index: usize) -> i64 {
        match self.vars.borrow_mut()[index] {
            Slot::Long(val) => val,
            _ => panic!("type mismatch"),
        }
    }

    pub(crate) fn set_double(&self, index: usize, val: f64) {
        self.vars.borrow_mut()[index] = Slot::Double(val);
        // double consumes two slots
        self.vars.borrow_mut()[index + 1] = Slot::null()
    }
    pub(crate) fn get_double(&self, index: usize) -> f64 {
        match self.vars.borrow()[index] {
            Slot::Double(val) => val,
            _ => panic!("type mismatch"),
        }
    }
    // TODO: consider GC
    pub(crate) fn set_ref(&self, index: usize, val: SharedObject) {
        self.vars.borrow_mut()[index] = Slot::Ref(val);
    }
    pub(crate) fn get_ref(&self, index: usize) -> SharedObject {
        match self.vars.borrow().get(index) {
            Some(Slot::Ref(val)) => val.clone(),
            _ => panic!("type mismatch"),
        }
    }
    pub(crate) fn get_slot(&self, index: usize) -> Slot {
        self.vars.borrow()[index].clone()
    }
    pub(crate) fn set_slot(&mut self, index: usize, val: Slot) {
        self.vars.borrow_mut()[index] = val;
    }
}
