use crate::instructions::base::instruction::Instruction;

#[derive(Debug, Default)]
pub(crate) struct LCMP {}
impl Instruction for LCMP{
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let v2 = stack.pop_long();
        let v1 = stack.pop_long();

        if v1 > v2 {
            stack.push_int(1);
        }else if v1 == v2 {
            stack.push_int(0)
        }else {
            stack.push_int(-1);
        }
    }
}