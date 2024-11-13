use crate::{
    instructions::base::instruction::{Index16Instruction, Instruction},
    rtda::{
        heap::{Object, SharedObject},
        slot::SlotVec,
    },
};

#[derive(Debug, Default)]
pub struct PUT_FIELD {
    delegation: Index16Instruction,
}

impl Instruction for PUT_FIELD {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.delegation.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let current_method = &frame.method;
        let current_class = &current_method.classMember.class;
        let cp = &current_class.constantPool;
        let field_ref = cp.get_field_ref(self.delegation.Index);
        let field = field_ref.resolve_field();
        let class = &field.class;

        if field.is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }

        if field.is_final() {
            if current_class.name != class.name || current_method.name != "<init>" {
                panic!("java.lang.IllegalAccessError");
            }
        }

        let descriptor = &field.descriptor;
        let slotId = field.slotId;
        let stack = &mut frame.operand_stack;

        match descriptor.chars().next().unwrap() {
            'Z' | 'B' | 'C' | 'S' | 'I' => {
                let val = stack.pop_int();
                let object = stack.pop_ref();
                if object.is_none() {
			        panic!("java.lang.NullPointerException")
                }
                let slots = &object.unwrap().fields;
                slots.set_int(slotId, val);
            }
            'F' => {
                let val = stack.pop_float();
                let object = stack.pop_ref();
                if object.is_none() {
			        panic!("java.lang.NullPointerException")
                }
                let slots = &object.unwrap().fields;
                slots.set_float(slotId, val);
            }
            'J' => {
                let val = stack.pop_long();
                let object = stack.pop_ref();
                if object.is_none() {
			        panic!("java.lang.NullPointerException")
                }
                let slots = &object.unwrap().fields;
                slots.set_long(slotId, val);
            }
            'D' => {
                let val = stack.pop_double();
                let object = stack.pop_ref();
                if object.is_none() {
			        panic!("java.lang.NullPointerException")
                }
                let slots = &object.unwrap().fields;
                slots.set_double(slotId, val);
            }
            'L' | '[' => {
                let val = stack.pop_ref();
                let object = stack.pop_ref();
                if object.is_none() {
			        panic!("java.lang.NullPointerException")
                }
                let slots = &object.unwrap().fields;
                slots.set_ref(slotId, val);
            }
            _ => {
                println!("put field default")
            }
        }
    }
}
