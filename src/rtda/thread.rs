use std::collections::VecDeque;

use super::frame::Frame;


pub(crate) struct Thread {
    pub(crate) pc: usize,
    stack: VecDeque<Frame>
}
impl Thread {
    fn new() -> Self {
        Thread {
            pc: 0,
            stack: VecDeque::new()
        }
    }
    fn push_frame(&mut self, frame: Frame) {
        self.stack.push_front(frame);
    }
    fn pop_frame(&mut self) -> Frame {
        self.stack.pop_front().unwrap()
    }
    fn current_frame(&self) -> &Frame {
        self.stack.get(0).unwrap()
    }
}