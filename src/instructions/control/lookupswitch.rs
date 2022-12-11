use crate::{instructions::base::{instruction::Instruction, branch}, rtda::frame::Frame};

#[derive(Debug, Default)]
pub(crate) struct LOOKUP_SWITCH {
    default_offset: usize,
    npairs: usize,
    match_offsets: Vec<(usize, usize)>,
}

impl Instruction for LOOKUP_SWITCH {
    fn fetch_operands(
        &mut self,
        reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader,
    ) {
        reader.skip_padding();
        self.default_offset = reader.read_i32() as usize;
        self.npairs = reader.read_i32() as usize;
        self.match_offsets = Vec::with_capacity(self.npairs);
        for _ in 0..self.npairs {
            let match_ = reader.read_i32() as usize;
            let offset = reader.read_i32() as usize;
            self.match_offsets.push((match_, offset));
        }
    }
    fn execute(&mut self, frame: &mut Frame) {
        let index =  frame.operand_stack.pop_int() as usize;
        for (i, o) in &self.match_offsets {
            if *i == index {
                branch::jump(frame, *o as i32);
                return;
            }
        }
        branch::jump(frame, self.default_offset as i32);
    }
}
