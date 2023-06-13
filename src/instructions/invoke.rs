use crate::instructions::base::instruction::Instruction;

use super::base::bytecode_reader::BytecodeReader;

#[derive(Debug, Default)]
pub(crate) struct INVOKE_VIRTUAL {}

impl Instruction for INVOKE_VIRTUAL {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do
        reader.read_u16();
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        // nothing to do
        println!("INVOKE_VIRTUAL");
        let val = frame.operand_stack.pop_int();
        println!("{}", val);
    }
}
