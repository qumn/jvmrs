use crate::{
    instructions::base::{
        bytecode_reader::BytecodeReader,
        instruction::{Index8Instruction, Instruction},
    },
    rtda::frame::Frame,
};

#[derive(Default, Debug)]
pub(crate) struct ILOAD {
    delegation: Index8Instruction,
}
impl ILOAD {
    pub(crate) fn set_index(&mut self, index: usize) {
        self.delegation.Index = index;
    }
}
impl Instruction for ILOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.delegation.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut Frame) {
        iload(frame, self.delegation.Index);
    }
}

#[derive(Default, Debug)]
pub(crate) struct ILOAD_0 {}
impl Instruction for ILOAD_0 {
    fn execute(&mut self, frame: &mut Frame) {
        iload(frame, 0);
    }
}

#[derive(Default, Debug)]
pub(crate) struct ILOAD_1 {}
impl Instruction for ILOAD_1 {
    fn execute(&mut self, frame: &mut Frame) {
        iload(frame, 1);
    }
}

#[derive(Default, Debug)]
pub(crate) struct ILOAD_2 {}
impl Instruction for ILOAD_2 {
    fn execute(&mut self, frame: &mut Frame) {
        iload(frame, 2);
    }
}

#[derive(Default, Debug)]
pub(crate) struct ILOAD_3 {}
impl Instruction for ILOAD_3 {
    fn execute(&mut self, frame: &mut Frame) {
        iload(frame, 3);
    }
}

fn iload(frame: &mut Frame, index: usize) {
    let val = frame.local_vars.get_int(index);
    frame.operand_stack.push_int(val);
}
