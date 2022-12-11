use crate::{instructions::base::instruction::Instruction, rtda::frame::Frame};

#[derive(Debug, Default)]
pub(crate) struct DMUL;

impl Instruction for DMUL {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        stack.push_int(v1 * v2);
    }
}

#[derive(Debug, Default)]
pub(crate) struct FMUL;
impl Instruction for FMUL {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_float();
        let v1 = stack.pop_float();
        stack.push_float(v1 * v2);
    }
}

#[derive(Debug, Default)]
pub(crate) struct IMUL;
impl Instruction for IMUL {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        stack.push_int(v1 * v2);
    }
}

#[derive(Debug, Default)]
pub(crate) struct LMUL;
impl Instruction for LMUL {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        stack.push_long(v1 * v2);
    }
}