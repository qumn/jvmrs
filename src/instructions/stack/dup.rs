use crate::{instructions::base::instruction::Instruction, rtda::frame::Frame};

#[derive(Default, Debug)]
pub(crate) struct DUP;

impl Instruction for DUP {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v = stack.pop_slot();
        stack.push_slot(v.clone());
        stack.push_slot(v);
    }
}

#[derive(Default, Debug)]
pub(crate) struct DUP_X1;
impl Instruction for DUP_X1 {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v1 = stack.pop_slot();
        let v2 = stack.pop_slot();
        stack.push_slot(v1.clone());
        stack.push_slot(v2);
        stack.push_slot(v1);
    }
}

#[derive(Default, Debug)]
pub(crate) struct DUP_X2;
impl Instruction for DUP_X2 {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v1 = stack.pop_slot();
        let v2 = stack.pop_slot();
        let v3 = stack.pop_slot();
        stack.push_slot(v1.clone());
        stack.push_slot(v3);
        stack.push_slot(v2);
        stack.push_slot(v1);
    }
}

#[derive(Default, Debug)]
pub(crate) struct DUP2;
impl Instruction for DUP2 {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v1 = stack.pop_slot();
        let v2 = stack.pop_slot();
        stack.push_slot(v2.clone());
        stack.push_slot(v1.clone());
        stack.push_slot(v2);
        stack.push_slot(v1);
    }
}

#[derive(Default, Debug)]
pub(crate) struct DUP2_X1;
impl Instruction for DUP2_X1 {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v1 = stack.pop_slot();
        let v2 = stack.pop_slot();
        let v3 = stack.pop_slot();
        stack.push_slot(v2.clone());
        stack.push_slot(v1.clone());
        stack.push_slot(v3);
        stack.push_slot(v2);
        stack.push_slot(v1);
    }
}


#[derive(Default, Debug)]
pub(crate) struct DUP2_X2;
impl Instruction for DUP2_X2 {
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let v1 = stack.pop_slot();
        let v2 = stack.pop_slot();
        let v3 = stack.pop_slot();
        let v4 = stack.pop_slot();
        stack.push_slot(v2.clone());
        stack.push_slot(v1.clone());
        stack.push_slot(v4);
        stack.push_slot(v3);
        stack.push_slot(v2);
        stack.push_slot(v1);
    }
}