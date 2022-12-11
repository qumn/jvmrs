use crate::instructions::base::instruction::Instruction;

#[derive(Debug, Default)]
pub(crate) struct IINC {
    pub(crate) index: usize,
    pub(crate) val: i32,
}
impl IINC {
    pub fn new() -> IINC {
        IINC::default()
    }
    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }
    pub fn set_val(&mut self, val: i32) {
        self.val = val;
    }
}

impl Instruction for IINC {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.index = reader.read_u8() as usize;
        self.val = reader.read_i8() as i32;
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let old = frame.local_vars.get_int(self.index);
        let new = old + self.val;
        frame.local_vars.set_int(self.index, new);
    }
}