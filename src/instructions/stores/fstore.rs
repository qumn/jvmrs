use crate::{
    instructions::base::{
        bytecode_reader::BytecodeReader,
        instruction::{Index8Instruction, Instruction},
    },
    rtda::frame::Frame,
};

#[derive(Debug, Default)]
pub(crate) struct FSTORE {
    delegation: Index8Instruction,
}
impl FSTORE {
    pub(crate) fn set_index(&mut self, index: usize) {
        self.delegation.Index = index;
    }
}

impl Instruction for FSTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.delegation.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut Frame) {
        fstore(frame, self.delegation.Index);
    }
}

#[derive(Debug, Default)]
pub(crate) struct FSTORE_0 {}

impl Instruction for FSTORE_0 {
    fn execute(&mut self, frame: &mut Frame) {
        fstore(frame, 0);
    }
}

#[derive(Debug, Default)]
pub(crate) struct FSTORE_1 {}

impl Instruction for FSTORE_1 {
    fn execute(&mut self, frame: &mut Frame) {
        fstore(frame, 1);
    }
}

#[derive(Debug, Default)]
pub(crate) struct FSTORE_2 {}

impl Instruction for FSTORE_2 {
    fn execute(&mut self, frame: &mut Frame) {
        fstore(frame, 2);
    }
}

#[derive(Debug, Default)]
pub(crate) struct FSTORE_3 {}

impl Instruction for FSTORE_3 {
    fn execute(&mut self, frame: &mut Frame) {
        fstore(frame, 3);
    }
}

fn fstore(frame: &mut Frame, index: usize) {
    let val = frame.operand_stack.pop_float();
    frame.local_vars.set_float(index, val);
}
