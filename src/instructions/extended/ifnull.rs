use crate::instructions::base::instruction::{BranchInstruction, Instruction};

#[derive(Debug, Default)]
pub(crate) struct IFNULL {
    branch: BranchInstruction
}
impl Instruction for IFNULL {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.branch.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let ref_ = stack.pop_ref();
        if ref_.is_null() {
            self.branch.jump(frame);
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct IFNONNULL {
    branch: BranchInstruction
}
impl Instruction for IFNONNULL {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.branch.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let ref_ = stack.pop_ref();
        if !ref_.is_null() {
            self.branch.jump(frame);
        }
    }
}