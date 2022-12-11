use crate::instructions::base::instruction::Instruction;


#[derive(Debug, Default)]
pub(crate) struct BIPUSH {
    val: i8
}
impl Instruction for BIPUSH {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.val = reader.read_i8();
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        frame.operand_stack.push_int(self.val as i32);
    }
}

#[derive(Debug, Default)]
pub(crate) struct SIPUSH {
    val: i16
}
impl Instruction for SIPUSH {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.val = reader.read_i16();
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        frame.operand_stack.push_int(self.val as i32);
    }
}