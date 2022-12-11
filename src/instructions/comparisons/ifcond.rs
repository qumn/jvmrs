use crate::instructions::base::instruction::{BranchInstruction, Instruction};

#[derive(Debug, Default)]
pub(crate) struct IFEQ {
    branch: BranchInstruction,
}
impl Instruction for IFEQ {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.branch.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let val = frame.operand_stack.pop_int();
        if val == 0 {
            self.branch.jump(frame);
        }
    }
}
#[derive(Debug, Default)]
pub(crate) struct IFNE {
    branch: BranchInstruction,
}
impl Instruction for IFNE {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.branch.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let val = frame.operand_stack.pop_int();
        if val != 0 {
            self.branch.jump(frame);
        }
    }
}
#[derive(Debug, Default)]
pub(crate) struct IFLT {
    branch: BranchInstruction,
}
impl Instruction for IFLT {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.branch.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let val = frame.operand_stack.pop_int();
        if val < 0 {
            self.branch.jump(frame);
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct IFLE {
    branch: BranchInstruction,
}
impl Instruction for IFLE {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.branch.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let val = frame.operand_stack.pop_int();
        if val <= 0 {
            self.branch.jump(frame);
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct IFGT {
    branch: BranchInstruction,
}
impl Instruction for IFGT {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.branch.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let val = frame.operand_stack.pop_int();
        if val > 0 {
            self.branch.jump(frame);
        }
    }
}


#[derive(Debug, Default)]
pub(crate) struct IFGE {
    branch: BranchInstruction,
}
impl Instruction for IFGE {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.branch.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let val = frame.operand_stack.pop_int();
        if val >= 0 {
            self.branch.jump(frame);
        }
    }
}