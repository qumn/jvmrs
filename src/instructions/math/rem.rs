use crate::{instructions::base::instruction::Instruction, rtda::frame::Frame};

#[derive(Default, Debug)]
pub(crate) struct DREM;
impl Instruction for DREM {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_double();
        let v1 = stack.pop_double();
        stack.push_double(v1 % v2);
    }
}


#[derive(Default, Debug)]
pub(crate) struct FREM;
impl Instruction for FREM {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_float();
        let v1 = stack.pop_float();
        stack.push_float(v1 % v2);
    }
}

#[derive(Default, Debug)]
pub(crate) struct IREM;
impl Instruction for IREM {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        if v2 == 0 {
            panic!("java.lang.ArithmeticException: % by zero");
        }
        stack.push_int(v1 % v2);
    }
}

#[derive(Default, Debug)]
pub(crate) struct LREM;
impl Instruction for LREM {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();
        if v2 == 0 {
            panic!("java.lang.ArithmeticException: % by zero");
        }
        stack.push_long(v1 % v2);
    }
}