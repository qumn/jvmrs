use std::rc::Weak;

use super::{heap::Method, local_vars::LocalVars, operand_stack::OperandStack, thread::Thread};

#[derive(Debug)]
pub(crate) struct Frame {
    pub local_vars: LocalVars,
    pub operand_stack: OperandStack,
    pub thread: Weak<Thread>,
    pub next_pc: usize,
    pub method: Method
}

impl Frame {
    pub(crate) fn new(thread: Weak<Thread>, method: &Method) -> Self {
        Frame {
            local_vars: LocalVars::new(method.max_locals.into()),
            operand_stack: OperandStack::new(method.max_stack.into()),
            thread,
            next_pc: 0,
            method: method.clone()
        }
    }
    pub(crate) fn PC(&self) -> usize {
        // thread is the frame parent, so the unwrap is safe here
        self.thread.upgrade().unwrap().pc
    }
}
