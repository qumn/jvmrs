use crate::{
    instructions::base::{
        bytecode_reader::BytecodeReader,
        instruction::{Index8Instruction, Instruction},
    },
    rtda::frame::Frame,
};

#[derive(Debug, Default)]
pub(crate) struct DLOAD {
    delegation: Index8Instruction,
}
impl DLOAD {
    pub(crate) fn set_index(&mut self, index: usize) {
        self.delegation.Index = index;
    }
}
impl Instruction for DLOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.delegation.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut Frame) {
        dload(frame, self.delegation.Index);
    }
}

#[derive(Debug, Default)]
pub(crate) struct DLOAD_0 {}
impl Instruction for DLOAD_0 {
    fn execute(&mut self, frame: &mut Frame) {
        dload(frame, 0);
    }
}

#[derive(Debug, Default)]
pub(crate) struct DLOAD_1 {}
impl Instruction for DLOAD_1 {
    fn execute(&mut self, frame: &mut Frame) {
        dload(frame, 1);
    }
}

#[derive(Debug, Default)]
pub(crate) struct DLOAD_2 {}
impl Instruction for DLOAD_2 {
    fn execute(&mut self, frame: &mut Frame) {
        dload(frame, 2);
    }
}

#[derive(Debug, Default)]
pub(crate) struct DLOAD_3 {}
impl Instruction for DLOAD_3 {
    fn execute(&mut self, frame: &mut Frame) {
        dload(frame, 3);
    }
}

fn dload(frame: &mut Frame, index: usize) {
    let d = frame.local_vars.get_double(index);
    frame.operand_stack.push_double(d);
}
