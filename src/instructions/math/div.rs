use crate::{instructions::base::instruction::Instruction, rtda::frame::Frame};

#[derive(Debug, Default)]
pub(crate) struct DDIV;
impl Instruction for DDIV {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_double();
        let v1 = stack.pop_double();
        stack.push_double(v1 / v2);
    }
}

#[derive(Debug, Default)]
pub(crate) struct FDIV;
impl Instruction for FDIV {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_float();
        let v1 = stack.pop_float();
        stack.push_float(v1 / v2);
    }
}

#[derive(Debug, Default)]
pub(crate) struct IDIV;
impl Instruction for IDIV {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        stack.push_int(v1 / v2);
    }
}

#[derive(Debug, Default)]
pub(crate) struct LDIV;
impl Instruction for LDIV {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        stack.push_long(v1 / v2);
    }
}