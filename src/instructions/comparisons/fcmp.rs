use crate::{instructions::base::instruction::Instruction, rtda::frame::Frame};

#[derive(Debug, Default)]
pub(crate) struct FCMPG {}
impl Instruction for FCMPG {
    fn execute(&mut self, frame: &mut Frame) {
        _fcmp(frame, true)
    }
}

#[derive(Debug, Default)]
pub(crate) struct FCMPL {}
impl Instruction for FCMPL {
    fn execute(&mut self, frame: &mut Frame) {
        _fcmp(frame, false)
    }
}

fn _fcmp(frame: &mut Frame, gFlag: bool) {
    let stack = &mut frame.operand_stack;
    let v2 = stack.pop_float();
    let v1 = stack.pop_float();
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
