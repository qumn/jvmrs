use tracing::debug;

use crate::{
    classfile::MemberInfo,
    instructions::{base::bytecode_reader::BytecodeReader, factory},
    rtda::{heap::Method, thread::Thread},
};

pub(crate) fn interpret(method: &Method) {
    let mut reader = BytecodeReader::new(method.code.to_vec());
    let mut thread = Thread::new();
    let mut _frame = thread.new_frame(method);
    // refactor thread pc
    loop {
        thread.reset_pc();
        let pc = thread.pc;
        reader.reset(pc);
        let opcode = reader.read_u8();
        let mut inst = factory::new_instruction(opcode);
        inst.fetch_operands(&mut reader);
        thread.current_frame_mut().next_pc = reader.pc;
        debug!("pc: {}, inst: {:?}", pc, inst);
        inst.execute(&mut thread.current_frame_mut());
    }
}
