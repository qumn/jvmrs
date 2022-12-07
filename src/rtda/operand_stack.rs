use super::{slot::SlotVec, object::Object};

pub(crate) struct OperandStack {
    size: usize,
    vec: SlotVec,
}

impl OperandStack {
    pub(crate) fn new(max_size: usize) -> Self {
        OperandStack {
            size: 0,
            vec: SlotVec::new(max_size),
        }
    }

    pub(crate) fn push_int(&mut self, val: i32) {
        self.vec.set_int(self.size, val);
        self.size += 1;
    }
    pub(crate) fn pop_int(&mut self) -> i32 {
        self.size -= 1;
        self.vec.get_int(self.size)
    }

    pub(crate) fn push_float(&mut self, val: f32) {
        self.vec.set_float(self.size, val);
        self.size += 1;
    }
    pub(crate) fn pop_float(&mut self) -> f32 {
        self.size -= 1;
        self.vec.get_float(self.size)
    }

    pub(crate) fn push_long(&mut self, val: i64) {
        self.vec.set_long(self.size, val);
        self.size += 2;
    }
    pub(crate) fn pop_long(&mut self) -> i64 {
        self.size -= 2;
        self.vec.get_long(self.size)
    }

    pub(crate) fn push_double(&mut self, val: f64) {
        self.vec.set_double(self.size, val);
        self.size += 2;
    }
    pub(crate) fn pop_double(&mut self) -> f64 {
        self.size -= 2;
        self.vec.get_double(self.size)
    }
    pub(crate) fn push_ref(&mut self, val: *mut Object) {
        self.vec.set_ref(self.size, val);
        self.size += 1;
    }
    pub(crate) fn pop_ref(&mut self) -> *mut Object {
        self.size -= 1;
        self.vec.get_ref(self.size)
    }
}

#[cfg(test)]
mod test {
    use crate::rtda::slot::{self, Slot};

    #[test] 
    fn test_operand_stack() {
        use super::*;
        let mut stack = OperandStack::new(10);
        stack.push_int(100);
        stack.push_float(100.0);
        stack.push_long(100);
        stack.push_double(100.0);
        stack.push_ref(std::ptr::null_mut());
        assert!(stack.pop_ref().is_null());
        assert_eq!(stack.pop_double(), 100.0);
        assert_eq!(stack.pop_long(), 100);
        assert_eq!(stack.pop_float(), 100.0);
    }
}