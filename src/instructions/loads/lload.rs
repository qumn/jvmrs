use crate::{
    instructions::base::{
        bytecode_reader::BytecodeReader,
        instruction::{Index8Instruction, Instruction},
    },
    rtda::frame::Frame,
};

#[derive(Debug, Default)]
pub(crate) struct LLOAD {
    delegation: Index8Instruction,
}
impl LLOAD {
    pub(crate) fn set_index(&mut self, index: usize) {
        self.delegation.Index = index;
    }
}
impl Instruction for LLOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.delegation.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut Frame) {
        lload(frame, self.delegation.Index);
    }
}

#[derive(Debug, Default)]
pub(crate) struct LLOAD_0 {}
impl Instruction for LLOAD_0 {
    fn execute(&mut self, frame: &mut Frame) {
        lload(frame, 0);
    }
}

#[derive(Debug, Default)]
pub(crate) struct LLOAD_1 {}
impl Instruction for LLOAD_1 {
    fn execute(&mut self, frame: &mut Frame) {
        lload(frame, 1)
    }
}

#[derive(Debug, Default)]
pub(crate) struct LLOAD_2 {}
impl Instruction for LLOAD_2 {
    fn execute(&mut self, frame: &mut Frame) {
        lload(frame, 2);
    }
}

#[derive(Debug, Default)]
pub(crate) struct LLOAD_3 {}
impl Instruction for LLOAD_3 {
    fn execute(&mut self, frame: &mut Frame) {
        lload(frame, 3);
    }
}

fn lload(frame: &mut Frame, index: usize) {
    let l = frame.local_vars.get_long(index);
    frame.operand_stack.push_long(l);
}
