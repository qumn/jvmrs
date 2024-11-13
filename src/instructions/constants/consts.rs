use crate::instructions::base::instruction::Instruction;

#[derive(Debug, Default)]
pub(crate) struct ACONST_NULL {}
impl Instruction for ACONST_NULL {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        frame.operand_stack.push_ref(None);
    }
}
#[derive(Debug, Default)]
pub(crate) struct DCONST_0 {}
impl Instruction for DCONST_0 {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        frame.operand_stack.push_double(0.0);
    }
}
#[derive(Debug, Default)]
pub(crate) struct DCONST_1 {}
impl Instruction for DCONST_1 {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        frame.operand_stack.push_double(1.0);
    }
}
#[derive(Debug, Default)]
pub(crate) struct FCONST_0 {}
impl Instruction for FCONST_0 {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        frame.operand_stack.push_float(0.0);
    }
}
#[derive(Debug, Default)]
pub(crate) struct FCONST_1 {}
impl Instruction for FCONST_1 {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        frame.operand_stack.push_float(1.0);
    }
}
#[derive(Debug, Default)]
pub(crate) struct FCONST_2 {}
impl Instruction for FCONST_2 {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        frame.operand_stack.push_float(2.0);
    }
}
#[derive(Debug, Default)]
pub(crate) struct ICONST_M1 {}
impl Instruction for ICONST_M1 {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        frame.operand_stack.push_int(-1);
    }
}
#[derive(Debug, Default)]
pub(crate) struct ICONST_0 {}
impl Instruction for ICONST_0 {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        frame.operand_stack.push_int(0);
    }
}
#[derive(Debug, Default)]
pub(crate) struct ICONST_1 {}
impl Instruction for ICONST_1 {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        frame.operand_stack.push_int(1);
    }
}
#[derive(Debug, Default)]
pub(crate) struct ICONST_2 {}
impl Instruction for ICONST_2 {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        frame.operand_stack.push_int(2);
    }
}
#[derive(Debug, Default)]
pub(crate) struct ICONST_3 {}
impl Instruction for ICONST_3 {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        frame.operand_stack.push_int(3);
    }
}
#[derive(Debug, Default)]
pub(crate) struct ICONST_4 {}
impl Instruction for ICONST_4 {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        frame.operand_stack.push_int(4);
    }
}
#[derive(Debug, Default)]
pub(crate) struct ICONST_5 {}
impl Instruction for ICONST_5 {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        frame.operand_stack.push_int(5);
    }
}
#[derive(Debug, Default)]
pub(crate) struct LCONST_0 {}
impl Instruction for LCONST_0 {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        frame.operand_stack.push_long(0);
    }
}
#[derive(Debug, Default)]
pub(crate) struct LCONST_1 {}
impl Instruction for LCONST_1 {
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        frame.operand_stack.push_long(1);
    }
}
