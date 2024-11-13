use std::{collections::VecDeque, rc::Weak};

use super::{frame::Frame, heap::Method};

#[derive(Debug)]
pub(crate) struct Thread {
    pub(crate) pc: usize,
    stack: VecDeque<Frame>,
}
impl Thread {
    pub(crate) fn new() -> Self {
        Thread {
            pc: 0,
            stack: VecDeque::new(),
        }
    }
    pub(crate) fn push_frame(&mut self, frame: Frame) {
        self.stack.push_back(frame);
    }
    pub(crate) fn pop_frame(&mut self) -> Frame {
        self.stack.pop_front().unwrap()
    }
    pub(crate) fn current_frame(&self) -> &Frame {
        self.stack.get(0).unwrap()
    }
    pub(crate) fn current_frame_mut(&mut self) -> &mut Frame {
        self.stack.get_mut(0).unwrap()
    }
    pub(crate) fn reset_pc(&mut self) {
        self.pc = self.current_frame().next_pc;
    }

    pub(crate) fn new_frame(&mut self, method: &Method) -> &Frame {
        let frame = Frame::new(unsafe {Weak::from_raw(self as *const _)}, method);
        self.push_frame(frame);
        self.current_frame()
    }
}
