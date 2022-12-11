use crate::instructions::base::instruction::Instruction;

#[derive(Debug, Default)]
pub(crate) struct F2D {}

impl Instruction for F2D {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let f = stack.pop_float();
        stack.push_double(f as f64);
    }
}

#[derive(Debug, Default)]
pub(crate) struct F2I {}
impl Instruction for F2I {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let f = stack.pop_float();
        stack.push_int(f as i32);
    }
}

#[derive(Debug, Default)]
pub(crate) struct F2L {}
impl Instruction for F2L {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let f = stack.pop_float();
        stack.push_long(f as i64);
    }
}
