use core::{panic, fmt};

use crate::instructions::{
    base::instruction::Instruction,
    constants::nop::NOP,
    loads::{aload::ALOAD, dload::DLOAD, iload::ILOAD, lload::LLOAD},
    math::iinc::IINC,
    stores::{astore::ASTORE, dstore::DSTORE, fstore::FSTORE, istore::ISTORE, lstore::LSTORE},
};

#[derive(Debug)]
pub(crate) struct WIDE {
    modified_instruction: Box<dyn Instruction>,
}
impl Default for WIDE {
    fn default() -> Self {
        Self {
            modified_instruction: Box::new(NOP {}),
        }
    }
}
impl Instruction for WIDE {
    fn fetch_operands(
        &mut self,
        reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader,
    ) {
        let opcode = reader.read_u8();
        match opcode {
            0x15 => {
                let mut inst = Box::new(ILOAD::default());
                inst.set_index(reader.read_i16() as usize);
                self.modified_instruction = inst
            }
            0x16 => {
                let mut inst = Box::new(LLOAD::default());
                inst.set_index(reader.read_i16() as usize);
                self.modified_instruction = inst
            }
            0x17 => {
                let mut inst = Box::new(ILOAD::default());
                inst.set_index(reader.read_i16() as usize);
                self.modified_instruction = inst
            }
            0x18 => {
                let mut inst = Box::new(DLOAD::default());
                inst.set_index(reader.read_i16() as usize);
                self.modified_instruction = inst
            }
            0x19 => {
                let mut inst = Box::new(ALOAD::default());
                inst.set_index(reader.read_i16() as usize);
                self.modified_instruction = inst
            }
            0x36 => {
                let mut inst = Box::new(ISTORE::default());
                inst.set_index(reader.read_i16() as usize);
                self.modified_instruction = inst
            }
            0x37 => {
                let mut inst = Box::new(LSTORE::default());
                inst.set_index(reader.read_i16() as usize);
                self.modified_instruction = inst;
            }
            0x38 => {
                let mut inst = Box::new(FSTORE::default());
                inst.set_index(reader.read_i16() as usize);
                self.modified_instruction = inst
            }
            0x39 => {
                let mut inst = Box::new(DSTORE::default());
                inst.set_index(reader.read_i16() as usize);
                self.modified_instruction = inst
            }
            0x3a => {
                let mut inst = Box::new(ASTORE::default());
                inst.set_index(reader.read_i16() as usize);
                self.modified_instruction = inst
            }
            0x84 => {
                let mut inst = Box::new(IINC::default());
                inst.set_index(reader.read_i16() as usize);
                inst.set_val(reader.read_i16() as i32);
                self.modified_instruction = inst
            }
            _ => {
                panic!("Unsupported opcode: 0x{:x}!", opcode);
            }
        }
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        self.modified_instruction.execute(frame);
    }
}
