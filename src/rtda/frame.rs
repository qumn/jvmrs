use super::{local_vars::LocalVars, operand_stack::OperandStack};

pub(crate) struct Frame {
    local_vars: LocalVars,
    operand_stack: OperandStack,
}

impl Frame{
    fn new(max_locals: usize, max_stack: usize) -> Self {
        Frame {
            local_vars: LocalVars::new(max_locals),
            operand_stack: OperandStack::new(max_stack)
        }
    }
}
