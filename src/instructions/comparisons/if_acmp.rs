use crate::instructions::base::instruction::{BranchInstruction, Instruction};



#[derive(Debug, Default)]
pub(crate) struct IF_ACMPEQ {
    branch: BranchInstruction,
}

impl Instruction for IF_ACMPEQ{
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.branch.fetch_operands(reader)
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let val2 = frame.operand_stack.pop_ref();
        let val1 = frame.operand_stack.pop_ref();
        if val2 == val1 {
            self.branch.jump(frame);
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct IF_ACMPNE {
    branch: BranchInstruction
}
impl Instruction for IF_ACMPNE {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.branch.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let val2 = frame.operand_stack.pop_ref();
        let val1 = frame.operand_stack.pop_ref();
        if val1 != val2 {
            self.branch.jump(frame);
        }
    }
}

