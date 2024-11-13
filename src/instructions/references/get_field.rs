use crate::{
    instructions::base::instruction::{self, Index16Instruction, Instruction},
    rtda::{frame::Frame, heap::Object, slot::SlotVec},
};

#[derive(Debug, Default)]
pub struct GET_FEILD {
    delegation: Index16Instruction,
}

impl Instruction for GET_FEILD {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.delegation.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut Frame) {
        let current_method = &frame.method;
        let current_class = &current_method.classMember.class;
        let cp = &current_class.constantPool;
        let field_ref = cp.get_field_ref(self.delegation.Index);
        let field = field_ref.resolve_field();
        let class = &field.class;

        if field.is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }

        let descriptor = &field.descriptor;
        let slotId = field.slotId;
        let stack = &mut frame.operand_stack;
        let object = stack.pop_ref();
        if object.is_none() {
            panic!("java.lang.NullPointerException")
        }
        let slots = &object.unwrap().fields;

        match descriptor.chars().next().unwrap() {
            'Z' | 'B' | 'C' | 'S' | 'I' => stack.push_int(slots.get_int(slotId)),
            'F' => stack.push_float(slots.get_float(slotId)),
            'J' => stack.push_long(slots.get_long(slotId)),
            'D' => stack.push_double(slots.get_double(slotId)),
            'L' | '[' => stack.push_ref(slots.get_ref(slotId)),
            _ => {
                println!("putstatic default")
            }
        }
    }
}
