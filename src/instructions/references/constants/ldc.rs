use crate::rtda::heap::Constant;
use crate::{
    classfile::ConstantPool,
    instructions::base::instruction::{Index16Instruction, Index8Instruction, Instruction},
    rtda::frame::Frame,
};

#[derive(Debug, Default)]
pub struct LDC {
    delegation: Index8Instruction,
}

impl Instruction for LDC {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.delegation.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut Frame) {
        _ldc(frame, self.delegation.Index as u16)
    }
}

#[derive(Debug, Default)]
pub struct LDC_W {
    delegation: Index16Instruction,
}

impl Instruction for LDC_W {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.delegation.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut Frame) {
        _ldc(frame, self.delegation.Index)
    }
}

fn _ldc(frame: &mut Frame, idx: u16) {
    let stack = &mut frame.operand_stack;
    let cp = &frame.method.class.constantPool;
    let c = cp.get_const(idx);

    match c {
        Constant::INTEGER(int) => {
            stack.push_int(*int);
        }
        Constant::FLOAT(float) => {
            stack.push_float(*float);
        }
        _ => panic!("todo: ldc! {:?}", c),
    }
}

#[derive(Debug, Default)]
pub struct LDC2_W {
    delegation: Index16Instruction,
}

impl Instruction for LDC2_W {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.delegation.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut Frame) {
        let stack = &mut frame.operand_stack;
        let cp = &frame.method.class.constantPool;
        let c = cp.get_const(self.delegation.Index);

        match c {
            Constant::LONG(long) => {
                stack.push_long(*long);
            }
            Constant::DOUBLE(double) => {
                stack.push_double(*double);
            }
            _ => {
                panic!("java.lang.ClassFormatError")
            }
        }
    }
}
