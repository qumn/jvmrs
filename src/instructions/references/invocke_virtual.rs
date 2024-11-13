use crate::instructions::base::instruction::{Index16Instruction, Instruction};

#[derive(Default, Debug)]
pub struct INVOKE_VIRTUAL {
    delegation: Index16Instruction,
}

impl Instruction for INVOKE_VIRTUAL {
    fn fetch_operands(&mut self, reader: &mut crate::instructions::base::bytecode_reader::BytecodeReader) {
        self.delegation.fetch_operands(reader);
    }
    fn execute(&mut self, frame: &mut crate::rtda::frame::Frame) {
        let class = &frame.method.class;
        let cp = &class.constantPool;
        let method_ref = cp.get_method_ref(self.delegation.Index);
        if method_ref.name == "println" {
            let stack = &mut frame.operand_stack;
            let descriptor = &method_ref.descriptor;
            match descriptor.as_ref() {
                "(Z)V" => print!("{}\n", stack.pop_int() != 0),
                "(C)V" => print!("{}\n", stack.pop_int()),
                "(I)V" | "(B)V" | "(S)V" => print!("{}\n", stack.pop_int()),
                "(F)V" => print!("{}\n", stack.pop_float()),
                "(J)V" => print!("{}\n", stack.pop_long()),
                "(D)V" => print!("{}\n", stack.pop_double()),
                _ => panic!("println: {}", descriptor),
            }
        }
    }
}
