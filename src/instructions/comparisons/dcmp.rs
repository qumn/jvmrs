use crate::{instructions::base::instruction::Instruction, rtda::frame::Frame};

#[derive(Debug, Default)]
pub(crate) struct DCMPG {}

impl Instruction for DCMPG {
    fn execute(&mut self, frame: &mut Frame) {
        _dcmp(frame, true)
    }
}

#[derive(Debug, Default)]
pub(crate) struct DCMPL {}

impl Instruction for DCMPL {
    fn execute(&mut self, frame: &mut Frame) {
        _dcmp(frame, false)
    }
}

fn _dcmp(frame: &mut Frame, gFlag: bool) {
    let stack = &mut frame.operand_stack;
    let v2 = stack.pop_double();
    let v1 = stack.pop_double();
    if v1 > v2 {
        stack.push_int(1)
    } else if v1 == v2 {
        stack.push_int(0)
    } else if v1 < v2 {
        stack.push_int(-1)
    } else if gFlag {
        stack.push_int(1)
    } else {
        stack.push_int(-1)
    }
}
