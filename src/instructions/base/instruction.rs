use core::fmt;
use std::fmt::Pointer;

use crate::rtda::frame::Frame;

use super::{bytecode_reader::BytecodeReader, branch};

pub(crate) trait Instruction: fmt::Debug {
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        // nothing to do
    }
    fn execute(&mut self, frame: &mut Frame);
}

#[derive(Debug, Default)]
pub(crate) struct BranchInstruction {
    offset: i32,
}

impl BranchInstruction {
    pub(crate) fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        let offset = reader.read_i16();
        self.offset = offset as i32;
    }
    pub(crate) fn jump(&mut self, frame: &mut Frame){
        branch::jump(frame, self.offset)
    }
}

#[derive(Debug, Default)]
pub(crate) struct Index8Instruction {
    pub(crate) Index: usize,
}
impl Index8Instruction {
    pub(crate) fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.Index = reader.read_u8() as usize;
    }
}

#[derive(Debug)]
pub(crate) struct Index16Instruction {
    Index: usize,
}

impl Index16Instruction {
    pub(crate) fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        self.Index = reader.read_u16() as usize;
    }
}
