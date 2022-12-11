use crate::instructions::base::instruction::{BranchInstruction, Instruction};

#[derive(Debug, Default)]
pub(crate) struct IF_ICMPEQ {
    branch: BranchInstruction,
}
impl Instruction for IF_ICMPEQ{
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.branch.fetch_operands(reader)
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let val2 = frame.operand_stack.pop_int();
        let val1 = frame.operand_stack.pop_int();
        if val2 == val1 {
            self.branch.jump(frame);
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct IF_ICMPNE {
    branch: BranchInstruction,
}
impl Instruction for IF_ICMPNE {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.branch.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let val2 = frame.operand_stack.pop_int();
        let val1 = frame.operand_stack.pop_int();
        if val1 != val2 {
            self.branch.jump(frame);
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct IF_ICMPLT {
    branch: BranchInstruction,
}
impl Instruction for IF_ICMPLT {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.branch.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let val2 = frame.operand_stack.pop_int();
        let val1 = frame.operand_stack.pop_int();
        if val1 < val2 {
            self.branch.jump(frame);
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct IF_ICMPLE {
    branch: BranchInstruction,
}
impl Instruction for IF_ICMPLE {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.branch.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let val2 = frame.operand_stack.pop_int();
        let val1 = frame.operand_stack.pop_int();
        if val1 <= val2 {
            self.branch.jump(frame);
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct IF_ICMPGT {
    branch: BranchInstruction,
}
impl Instruction for IF_ICMPGT {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.branch.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let val2 = frame.operand_stack.pop_int();
        let val1 = frame.operand_stack.pop_int();
        if val1 > val2 {
            self.branch.jump(frame);
        }
    }
}
#[derive(Debug, Default)]
pub(crate) struct IF_ICMPGE {
    branch: BranchInstruction,
}
impl Instruction for IF_ICMPGE {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.branch.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let val2 = frame.operand_stack.pop_int();
        let val1 = frame.operand_stack.pop_int();
        if val1 >= val2 {
            self.branch.jump(frame);
        }
    }
}