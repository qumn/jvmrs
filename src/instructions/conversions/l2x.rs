use crate::{instructions::base::instruction::Instruction, rtda::frame::Frame};

#[derive(Debug, Default)]
pub(crate) struct L2D {}
impl Instruction for L2D {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let l = stack.pop_long();
        stack.push_double(l as f64);
    }
}

#[derive(Debug, Default)]
pub(crate) struct L2F {}
impl Instruction for L2F {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let l = stack.pop_long();
        stack.push_float(l as f32);
    }
}

#[derive(Debug, Default)]
pub(crate) struct L2I{}
impl Instruction for L2I {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let l = stack.pop_long();
        stack.push_int(l as i32);
    }
}

#[cfg(test)]
mod test{
    #[test]
    fn test(){
        let a = -1;
        let b: usize = a as usize;
        println!("b: {}", b);
    }
}
