use crate::instructions::base::instruction::{Index16Instruction, Instruction};

#[derive(Debug, Default)]
pub struct INSTANCE_OF {
    delegation: Index16Instruction,
}

impl Instruction for INSTANCE_OF {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.delegation.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let object_ref = stack.pop_ref();
        if object_ref.is_none() {
            stack.push_int(0);
        }
        
        let cp = &frame.method.class.constantPool;
        let class_ref = cp.get_classref(self.delegation.Index);
        let class = class_ref.resolve_class();
        if object_ref.unwrap().is_instance_of(class) {
            stack.push_int(1);
        }else {
            stack.push_int(0);
        }

    }
}

