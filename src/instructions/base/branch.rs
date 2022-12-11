use crate::rtda::frame::Frame;


pub(crate) fn jump(frame: &mut Frame, offset: i32) {
    println!("frame: {:?}\tjump offset: {}", frame, offset);
    let pc = frame.PC();
    let next_pc = pc as i32 + offset;
    frame.next_pc = next_pc as usize;
}
