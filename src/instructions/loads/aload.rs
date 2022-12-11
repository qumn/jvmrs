use crate::{
    instructions::base::{
        bytecode_reader::BytecodeReader,
        instruction::{Index8Instruction, Instruction},
    },
    rtda::frame::Frame,
};

#[derive(Default, Debug)]
pub(crate) struct ALOAD {
    delegation: Index8Instruction,
}
impl ALOAD {
    pub(crate) fn set_index(&mut self, index: usize) {
        self.delegation.Index = index;
    }
}
impl Instruction for ALOAD {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.delegation.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut Frame) {
        aload(frame, self.delegation.Index);
    }
}

#[derive(Default, Debug)]
pub(crate) struct ALOAD_0 {}
impl Instruction for ALOAD_0 {
    fn execute(&mut self, frame: &mut Frame) {
        aload(frame, 0);
    }
}

#[derive(Default, Debug)]
pub(crate) struct ALOAD_1 {}
impl Instruction for ALOAD_1 {
    fn execute(&mut self, frame: &mut Frame) {
        aload(frame, 1);
    }
}

#[derive(Default, Debug)]
pub(crate) struct ALOAD_2 {}
impl Instruction for ALOAD_2 {
    fn execute(&mut self, frame: &mut Frame) {
        aload(frame, 2);
    }
}

#[derive(Default, Debug)]
pub(crate) struct ALOAD_3 {}
impl Instruction for ALOAD_3 {
    fn execute(&mut self, frame: &mut Frame) {
        aload(frame, 3);
    }
}

fn aload(frame: &mut Frame, index: usize) {
    let aref = frame.local_vars.get_ref(index);
    frame.operand_stack.push_ref(aref);
}
