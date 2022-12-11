use crate::instructions::base::{instruction::Instruction, branch};

#[derive(Debug, Default)]
pub(crate) struct GOTO_W {
    offset: i32,
}

impl Instruction for GOTO_W {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.offset = reader.read_i32();
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        branch::jump(frame, self.offset);
    }
}