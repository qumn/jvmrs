use crate::{instructions::base::{instruction::{BranchInstruction, Instruction}, bytecode_reader::BytecodeReader}, rtda::frame::Frame};

#[derive(Debug, Default)]
pub(crate) struct GOTO {
    branch: BranchInstruction
}

impl Instruction for GOTO {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.branch.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut Frame) {
        self.branch.jump(frame);
    }
}