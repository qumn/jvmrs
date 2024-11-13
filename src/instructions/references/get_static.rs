use crate::{
    instructions::base::instruction::{Index16Instruction, Instruction},
    rtda::{frame::Frame, slot::SlotVec},
};

#[derive(Default, Debug)]
pub struct GET_STATIC {
    delegation: Index16Instruction,
}

impl Instruction for GET_STATIC {

    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.delegation.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut Frame) {
        let current_class = &frame.method.class;
        let cp = &current_class.constantPool;
        let field_ref = cp.get_field_ref(self.delegation.Index);
        let field = field_ref.resolve_field();
        let class = &field.class;
        if !field.is_static() {
            panic!("java.lang.IncompatibleClassChangeError");
        }

        let descriptor = &field.descriptor;
        let slotId = field.slotId;
        let stack = &mut frame.operand_stack;
        let slots = &class.staticVars;

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
