use crate::{instructions::base::instruction::Instruction, rtda::frame::Frame};

#[derive(Default, Debug)]
pub(crate) struct SWAP;

impl Instruction for SWAP {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v1 = stack.pop_slot();
        let v2 = stack.pop_slot();
        stack.push_slot(v1);
        stack.push_slot(v2);
    }
}