use crate::instructions::base::instruction::Instruction;

#[derive(Debug, Default)]
pub(crate) struct ISHL;

impl Instruction for ISHL {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        let s = v2 & 0x1f;
        stack.push_int(v1 << s);
    }
}

#[derive(Debug, Default)]
pub(crate) struct ISHR;
impl Instruction for ISHR {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_int();
        let v1 = stack.pop_int();
        let s = v2 & 0x1f;
        stack.push_int(v1 >> s);
    }
}

#[derive(Debug, Default)]
pub(crate) struct IUSHR;
impl Instruction for IUSHR {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_int();
        let v1 = stack.pop_int() as u32;
        let s = v2 & 0x1f;
        stack.push_int((v1 >> s) as i32);
    }
}

#[derive(Debug, Default)]
pub(crate) struct LSHL;
impl Instruction for LSHL {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_int();
        let v1 = stack.pop_long();
        let s = v2 & 0x3f;
        stack.push_long(v1 << s);
    }
}

#[derive(Debug, Default)]
pub(crate) struct LSHR;
impl Instruction for LSHR {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_int();
        let v1 = stack.pop_long();
        let s = v2 & 0x3f;
        stack.push_long(v1 >> s);
    }
}

#[derive(Debug, Default)]
pub(crate) struct LUSHR;
impl Instruction for LUSHR {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_int();
        let v1 = stack.pop_long() as u64;
        let s = v2 & 0x3f;
        stack.push_long((v1 >> s) as i64);
    }
}