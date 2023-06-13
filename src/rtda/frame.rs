use std::rc::Weak;

use super::{local_vars::LocalVars, operand_stack::OperandStack, thread::Thread};

#[derive(Debug)]
pub(crate) struct Frame {
    pub(crate) local_vars: LocalVars,
    pub(crate) operand_stack: OperandStack,
    pub(crate) thread: Weak<Thread>,
    pub(crate) next_pc: usize,
}

impl Frame {
    pub(crate) fn new(thread: Weak<Thread>, max_locals: usize, max_stack: usize) -> Self {
        Frame {
            local_vars: LocalVars::new(max_locals),
            operand_stack: OperandStack::new(max_stack),
            thread,
            next_pc: 0,
        }
    }
    pub(crate) fn PC(&self) -> usize {
        // thread is the frame parent, so the unwrap is safe here
        self.thread.upgrade().unwrap().pc
    }
}
