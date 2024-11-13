use tracing::debug;

use crate::{
    classfile::FieldrefInfo,
    instructions::base::instruction::{Index16Instruction, Instruction}, rtda::slot::SlotVec,
};

#[derive(Default, Debug)]
pub struct PUT_STATIC {
    delegation: Index16Instruction,
}

impl Instruction for PUT_STATIC {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.delegation.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        debug!("start execute put_static");
        let current_method = &frame.method;
        let current_class = &current_method.classMember.class;
        let cp = &current_class.constantPool;
        let field_ref = cp.get_field_ref(self.delegation.Index);
        let field = field_ref.resolve_field();
        let class = &field.class;

        if !field.is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }
        if field.is_final() {
            if current_class.name != class.name || current_method.name != "<clinit>" {
                panic!("java.lang.IllegalAccessError");
            }
        }

        let descriptor = &field.descriptor;
        let slotId = field.slotId;
        let stack = &mut frame.operand_stack;
        let slots = &class.staticVars;

        match descriptor.chars().next().unwrap() {
            'Z' | 'B' | 'C' | 'S' | 'I' => slots.set_int(slotId, stack.pop_int()),
            'F' => slots.set_float(slotId, stack.pop_float()),
            'J' => slots.set_long(slotId, stack.pop_long()),
            'D' => slots.set_double(slotId, stack.pop_double()),
            'L' | '[' => slots.set_ref(slotId, stack.pop_ref()),
            _ => {
                println!("putstatic default")
            }
        }
    }
}
