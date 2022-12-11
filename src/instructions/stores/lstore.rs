use crate::{
    instructions::base::{
        bytecode_reader::BytecodeReader,
        instruction::{Index8Instruction, Instruction},
    },
    rtda::frame::Frame,
};

#[derive(Debug, Default)]
pub(crate) struct LSTORE {
    delegation: Index8Instruction,
}

impl LSTORE {
    pub(crate) fn set_index(&mut self, index: usize) {
        self.delegation.Index = index;
    }
}
impl Instruction for LSTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.delegation.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut Frame) {
        lstore(frame, self.delegation.Index);
    }
}

#[derive(Debug, Default)]
pub(crate) struct LSTORE_0 {}

impl Instruction for LSTORE_0 {
    fn execute(&mut self, frame: &mut Frame) {
        lstore(frame, 0);
    }
}

#[derive(Debug, Default)]
pub(crate) struct LSTORE_1 {}

impl Instruction for LSTORE_1 {
    fn execute(&mut self, frame: &mut Frame) {
        lstore(frame, 1);
    }
}

#[derive(Debug, Default)]
pub(crate) struct LSTORE_2 {}

impl Instruction for LSTORE_2 {
    fn execute(&mut self, frame: &mut Frame) {
        lstore(frame, 2);
    }
}

#[derive(Debug, Default)]
pub(crate) struct LSTORE_3 {}

impl Instruction for LSTORE_3 {
    fn execute(&mut self, frame: &mut Frame) {
        lstore(frame, 3);
    }
}

fn lstore(frame: &mut Frame, index: usize) {
    let val = frame.operand_stack.pop_long();
    frame.local_vars.set_long(index, val);
}


