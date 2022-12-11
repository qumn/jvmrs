use crate::instructions::base::instruction::Instruction;

#[derive(Debug, Default)]
pub(crate) struct NOP {}

impl Instruction for NOP{
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        // nothing to do
    }
}