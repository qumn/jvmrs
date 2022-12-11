use crate::instructions::base::instruction::Instruction;

#[derive(Debug, Default)]
pub(crate) struct I2L {}
impl Instruction for I2L {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let i = stack.pop_int();
        stack.push_long(i as i64);
    }
}

#[derive(Debug, Default)]
pub(crate) struct I2D {}
impl Instruction for I2D {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let i = stack.pop_int();
        stack.push_double(i as f64);
    }
}

#[derive(Debug, Default)]
pub(crate) struct I2F {}
impl Instruction for I2F {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let i = stack.pop_int();
        stack.push_float(i as f32);
    }
}

#[derive(Debug, Default)]
pub(crate) struct I2B;
impl Instruction for I2B {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let i = stack.pop_int();
        stack.push_int(i as i8 as i32);
    }
}

#[derive(Debug, Default)]
pub(crate) struct I2C;
impl Instruction for I2C {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let i = stack.pop_int();
        stack.push_int(i as u16 as i32);
    }
}

#[derive(Debug, Default)]
pub(crate) struct I2S;
impl Instruction for I2S {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let i = stack.pop_int();
        stack.push_int(i as i16 as i32);
    }
}