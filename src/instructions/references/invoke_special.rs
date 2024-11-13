use crate::instructions::base::instruction::{Index16Instruction, Instruction};

#[derive(Debug, Default)]
pub struct INVOKE_SPECIAL {
    delegation: Index16Instruction,
}

impl Instruction for INVOKE_SPECIAL {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.delegation.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        frame.operand_stack.pop_ref();
    }
}