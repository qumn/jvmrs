use std::process::exit;

use crate::instructions::base::instruction::Instruction;

use super::base::bytecode_reader::BytecodeReader;

#[derive(Debug, Default)]
pub(crate) struct RETURN_INST {}

impl Instruction for RETURN_INST {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        // exit the program a temporary way
        exit(0);
    }
}
