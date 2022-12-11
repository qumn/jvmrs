use crate::{
    instructions::base::{
        bytecode_reader::BytecodeReader,
        instruction::{Index8Instruction, Instruction},
    },
    rtda::frame::Frame,
};

#[derive(Debug, Default)]
pub(crate) struct ASTORE {
    delegation: Index8Instruction,
}
impl ASTORE {
    pub(crate) fn set_index(&mut self, index: usize) {
        self.delegation.Index = index;
    }
}

impl Instruction for ASTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.delegation.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut Frame) {
        astore(frame, self.delegation.Index);
    }
}

#[derive(Debug, Default)]
pub(crate) struct ASTORE_0 {}

impl Instruction for ASTORE_0 {
    fn execute(&mut self, frame: &mut Frame) {
        astore(frame, 0);
    }
}

#[derive(Debug, Default)]
pub(crate) struct ASTORE_1 {}

impl Instruction for ASTORE_1 {
    fn execute(&mut self, frame: &mut Frame) {
        astore(frame, 1);
    }
}

#[derive(Debug, Default)]
pub(crate) struct ASTORE_2 {}

impl Instruction for ASTORE_2 {
    fn execute(&mut self, frame: &mut Frame) {
        astore(frame, 2);
    }
}

#[derive(Debug, Default)]
pub(crate) struct ASTORE_3 {}

impl Instruction for ASTORE_3 {
    fn execute(&mut self, frame: &mut Frame) {
        astore(frame, 3);
    }
}

fn astore(frame: &mut Frame, index: usize) {
    let val = frame.operand_stack.pop_ref();
    frame.local_vars.set_ref(index, val);
}
