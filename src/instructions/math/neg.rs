use crate::{instructions::base::instruction::Instruction, rtda::frame::Frame};

#[derive(Default, Debug)]
pub(crate) struct DNEG;
impl Instruction for DNEG {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v = stack.pop_double();
        stack.push_double(-v);
    }
}

#[derive(Default, Debug)]
pub(crate) struct FNEG;
impl Instruction for FNEG {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v = stack.pop_float();
        stack.push_float(-v);
    }
}

#[derive(Default, Debug)]
pub(crate) struct INEG;
impl Instruction for INEG {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v = stack.pop_int();
        stack.push_int(-v);
    }
}

#[derive(Default, Debug)]
pub(crate) struct LNEG;
impl Instruction for LNEG {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v = stack.pop_long();
        stack.push_long(-v);
    }
}