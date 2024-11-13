use crate::{
    instructions::base::instruction::{Index16Instruction, Instruction},
    rtda::{frame::Frame, heap::Object},
};

#[derive(Default, Debug)]
pub struct NEW {
    delegation: Index16Instruction,
}

impl Instruction for NEW {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.delegation.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut Frame) {
        let class = &frame.method.classMember.class;
        let cp = &class.constantPool;
        let classRef = cp.get_classref(self.delegation.Index);
        let shared_object = class.new_object();
        frame.operand_stack.push_ref(shared_object);
    }
}
