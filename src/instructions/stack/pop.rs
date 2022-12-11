use crate::{instructions::base::instruction::Instruction, rtda::frame::Frame};

#[derive(Debug, Default)]
pub(crate) struct POP;

impl Instruction for POP {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        stack.pop_slot();
    }
}

#[derive(Debug, Default)]
pub(crate) struct POP2;

impl Instruction for POP2 {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        stack.pop_slot();
        stack.pop_slot();
    }
}