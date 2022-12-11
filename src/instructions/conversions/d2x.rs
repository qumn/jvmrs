use tracing_subscriber::field::debug;

use crate::instructions::base::instruction::Instruction;

#[derive(Debug, Default)]
pub(crate) struct D2F {}
impl Instruction for D2F {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let d = stack.pop_double();
        stack.push_float(d as f32);
    }
}

#[derive(Debug, Default)]
pub(crate) struct D2I {}
impl Instruction for D2I {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let d = stack.pop_double();
        stack.push_int(d as i32);
    }
}

#[derive(Debug, Default)]
pub(crate) struct D2L {}
impl Instruction for D2L {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let d = stack.pop_double();
        stack.push_long(d as i64);
    }
}
