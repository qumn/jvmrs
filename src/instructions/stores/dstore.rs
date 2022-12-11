use crate::{
    instructions::base::{
        bytecode_reader::BytecodeReader,
        instruction::{Index8Instruction, Instruction},
    },
    rtda::frame::Frame,
};

#[derive(Debug, Default)]
pub(crate) struct DSTORE {
    delegation: Index8Instruction,
}
impl DSTORE {
    pub(crate) fn set_index(&mut self, index: usize) {
        self.delegation.Index = index;
    }
}

impl Instruction for DSTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.delegation.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut Frame) {
        dstore(frame, self.delegation.Index);
    }
}

#[derive(Debug, Default)]
pub(crate) struct DSTORE_0 {}

impl Instruction for DSTORE_0 {
    fn execute(&mut self, frame: &mut Frame) {
        dstore(frame, 0);
    }
}

#[derive(Debug, Default)]
pub(crate) struct DSTORE_1 {}

impl Instruction for DSTORE_1 {
    fn execute(&mut self, frame: &mut Frame) {
        dstore(frame, 1);
    }
}

#[derive(Debug, Default)]
pub(crate) struct DSTORE_2 {}

impl Instruction for DSTORE_2 {
    fn execute(&mut self, frame: &mut Frame) {
        dstore(frame, 2);
    }
}

#[derive(Debug, Default)]
pub(crate) struct DSTORE_3 {}

impl Instruction for DSTORE_3 {
    fn execute(&mut self, frame: &mut Frame) {
        dstore(frame, 3);
    }
}

fn dstore(frame: &mut Frame, index: usize) {
    let val = frame.operand_stack.pop_double();
    frame.local_vars.set_double(index, val);
}
