use crate::{instructions::base::{bytecode_reader::BytecodeReader, instruction::Instruction, branch}, rtda::frame::Frame};

#[derive(Debug, Default)]
pub(crate) struct TABLE_SWITCH {
    default_offset: usize,
    low: usize,
    high: usize,
    jumpOffsets: Vec<usize>,
}

impl Instruction for TABLE_SWITCH {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        reader.skip_padding();
        self.default_offset = reader.read_i32() as usize;
        self.low = reader.read_i32() as usize;
        self.high = reader.read_i32() as usize;
        let jump_offset_count = self.high - self.low + 1;
        self.jumpOffsets = reader
            .read_i32s(jump_offset_count)
            .iter()
            .map(|i| *i as usize)
            .collect();
    }

    fn execute(&mut self, frame: &mut Frame) {
        let index = frame.operand_stack.pop_int() as usize;
        let offset = if index >= self.low && index <= self.high {
             self.jumpOffsets[index - self.low]
        }else {
            self.default_offset
        };
        branch::jump(frame, offset as i32)
    }
}
