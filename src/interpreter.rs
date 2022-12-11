use crate::{
    classfile::MemberInfo,
    instructions::{base::bytecode_reader::BytecodeReader, factory},
    rtda::thread::Thread,
};

pub(crate) fn interpret(method_info: &MemberInfo) {
    let code_attribute = method_info.code_attribute().unwrap();
    let max_locals = code_attribute.max_locals as usize;
    let max_stack = code_attribute.max_stack as usize;
    let mut reader = BytecodeReader::new(code_attribute.code.to_vec());
    let mut thread = Thread::new();
    let mut _frame = thread.new_frame(max_locals, max_stack);
    loop {
        thread.reset_pc();
        let pc = thread.pc;
        reader.reset(pc);
        let opcode = reader.read_u8();
        let mut inst = factory::new_instruction(opcode);
        inst.fetch_operands(&mut reader);
        thread.current_frame_mut().next_pc = reader.pc;
        println!("pc: {}, inst: {:?}", pc, inst);
        inst.execute(&mut thread.current_frame_mut());
    }
}
