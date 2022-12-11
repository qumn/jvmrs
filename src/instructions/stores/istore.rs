use crate::{
    instructions::base::{
        bytecode_reader::BytecodeReader,
        instruction::{Index8Instruction, Instruction},
    },
    rtda::frame::Frame,
};

#[derive(Debug, Default)]
pub(crate) struct ISTORE {
    delegation: Index8Instruction,
}

impl ISTORE {
    pub(crate) fn set_index(&mut self, index: usize) {
        self.delegation.Index = index;
    }
}
impl Instruction for ISTORE {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.delegation.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut Frame) {
        istore(frame, self.delegation.Index);
    }
}

#[derive(Debug, Default)]
pub(crate) struct ISTORE_0 {}

impl Instruction for ISTORE_0 {
    fn execute(&mut self, frame: &mut Frame) {
        istore(frame, 0);
    }
}

#[derive(Debug, Default)]
pub(crate) struct ISTORE_1 {}

impl Instruction for ISTORE_1 {
    fn execute(&mut self, frame: &mut Frame) {
        istore(frame, 1);
    }
}

#[derive(Debug, Default)]
pub(crate) struct ISTORE_2 {}

impl Instruction for ISTORE_2 {
    fn execute(&mut self, frame: &mut Frame) {
        istore(frame, 2);
    }
}

#[derive(Debug, Default)]
pub(crate) struct ISTORE_3 {}

impl Instruction for ISTORE_3 {
    fn execute(&mut self, frame: &mut Frame) {
        istore(frame, 3);
    }
}

fn istore(frame: &mut Frame, index: usize) {
    let val = frame.operand_stack.pop_int();
    frame.local_vars.set_int(index, val);
}

