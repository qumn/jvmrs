use crate::{
    instructions::base::{
        bytecode_reader::BytecodeReader,
        instruction::{Index8Instruction, Instruction},
    },
    rtda::frame::Frame,
};

#[derive(Debug, Default)]
pub(crate) struct FLOAD {
    delegation: Index8Instruction,
}
impl FLOAD {
    pub(crate) fn set_index(&mut self, index: usize) {
        self.delegation.Index = index;
    }
}
impl Instruction for FLOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.delegation.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut Frame) {
        fload(frame, self.delegation.Index);
    }
}

#[derive(Debug, Default)]
pub(crate) struct FLOAD_0 {}
impl Instruction for FLOAD_0 {
    fn execute(&mut self, frame: &mut Frame) {
        fload(frame, 0);
    }
}

#[derive(Debug, Default)]
pub(crate) struct FLOAD_1 {}
impl Instruction for FLOAD_1 {
    fn execute(&mut self, frame: &mut Frame) {
        fload(frame, 1);
    }
}

#[derive(Debug, Default)]
pub(crate) struct FLOAD_2 {}
impl Instruction for FLOAD_2 {
    fn execute(&mut self, frame: &mut Frame) {
        fload(frame, 2);
    }
}

#[derive(Debug, Default)]
pub(crate) struct FLOAD_3 {}
impl Instruction for FLOAD_3 {
    fn execute(&mut self, frame: &mut Frame) {
        fload(frame, 3);
    }
}

fn fload(frame: &mut Frame, index: usize) {
    let val = frame.local_vars.get_float(index);
    frame.operand_stack.push_float(val);
}
