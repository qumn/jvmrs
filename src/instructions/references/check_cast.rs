use crate::{instructions::base::instruction::{Index16Instruction, Instruction}, rtda::heap::cp_classref};

#[derive(Debug, Default)]
pub struct CHECK_CAST {
    delegation: Index16Instruction,
}

impl Instruction for CHECK_CAST {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.delegation.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let stack = &mut frame.operand_stack;
        let object_ref = stack.pop_ref();
        stack.push_ref(object_ref.clone());
        if object_ref.is_none() {
            return
        }

        let cp = &frame.method.class.constantPool;
        let class_ref = cp.get_classref(self.delegation.Index);
        let class = class_ref.resolve_class();
        if !object_ref.unwrap().is_instance_of(class) {
            panic!("java.lang.ClassCastEception");
        }
    }
}