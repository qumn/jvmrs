use crate::{instructions::base::instruction::Instruction, rtda::frame::Frame};

#[derive(Default, Debug)]
pub(crate) struct DADD{}
impl Instruction for DADD {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_double();
        let v1 = stack.pop_double();
        stack.push_double(v1 + v2);
    }
}

#[derive(Default, Debug)]
pub(crate) struct FADD{}
impl Instruction for FADD {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_float();
        let v1 = stack.pop_float();
        stack.push_float(v1 + v2);
    }
}

#[derive(Default, Debug)]
pub(crate) struct IADD{}
impl Instruction for IADD {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        stack.push_int(v1 + v2);
    }
}

#[derive(Default, Debug)]
pub(crate) struct LADD{}
impl Instruction for LADD {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        stack.push_long(v1 + v2);
    }
}
