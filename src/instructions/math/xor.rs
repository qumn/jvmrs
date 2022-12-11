use crate::{instructions::base::instruction::Instruction, rtda::frame::Frame};

#[derive(Debug, Default)]
pub(crate) struct IXOR;

impl Instruction for IXOR {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        stack.push_int(v1 ^ v2);
    }
}

#[derive(Debug, Default)]
pub(crate) struct LXOR;
impl Instruction for LXOR {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        stack.push_long(v1 ^ v2);
    }
}