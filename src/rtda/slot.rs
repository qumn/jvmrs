use super::heap::Object;

#[derive(Debug, Clone)]
pub(crate) enum Slot {
    Int(i32),
    Float(f32),
    Long(i64),
    Double(f64),
    Ref(*mut Object),
}

impl Slot {
    pub(crate) fn null() -> Self {
        Slot::Ref(std::ptr::null_mut())
    }
}

#[derive(Debug)]
pub(crate) struct SlotVec {
    vars: Vec<Slot>
}

impl SlotVec {
    pub(crate) fn new(max_locals: usize) -> Self {
        SlotVec {
            vars: vec![Slot::null(); max_locals]
        }
    }
    pub(crate) fn set_int(&mut self, index: usize, val: i32) {
        self.vars[index] = Slot::Int(val);
    }

    pub(crate) fn get_int(&self, index: usize) -> i32 {
        match self.vars[index] {
            Slot::Int(val) => val,
            _ => panic!("type mismatch")
        }
    }

    pub(crate) fn set_float(&mut self, index: usize, val: f32) {
        self.vars[index] = Slot::Float(val);
    }
    pub(crate) fn get_float(&self, index: usize) -> f32 {
        match self.vars[index] {
            Slot::Float(val) => val,
            _ => panic!("type mismatch")
        }
    }

    pub(crate) fn set_long(&mut self, index: usize, val: i64) {
        self.vars[index] = Slot::Long(val);
        // long consumes two slots
        self.vars[index+1] = Slot::null()
    }
    pub(crate) fn get_long(&self, index: usize) -> i64 {
        match self.vars[index] {
            Slot::Long(val) => val,
            _ => panic!("type mismatch")
        }
    }

    pub(crate) fn set_double(&mut self, index: usize, val: f64) {
        self.vars[index] = Slot::Double(val);
        // double consumes two slots
        self.vars[index+1] = Slot::null()
    }
    pub(crate) fn get_double(&self, index: usize) -> f64 {
        match self.vars[index] {
            Slot::Double(val) => val,
            _ => panic!("type mismatch")
        }
    }
    // TODO: consider GC
    pub(crate) fn set_ref(&mut self, index: usize, val: *mut Object) {
        self.vars[index] = Slot::Ref(val);
    }
    pub(crate) fn get_ref(&self, index: usize) -> *mut Object {
        match self.vars[index] {
            Slot::Ref(val) => val,
            _ => panic!("type mismatch")
        }
    }
    pub(crate) fn get_slot(&self, index: usize) -> Slot {
        self.vars[index].clone()
    }
    pub(crate) fn set_slot(&mut self, index: usize, val: Slot){
        self.vars[index] = val;
    }
}
