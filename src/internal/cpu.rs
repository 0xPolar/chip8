pub struct CPU {
    regs: [u8; 16],
    index: u16,
    program_counter: u16,
    stack_pointer: u8,
    stack: [u16; 16],
    delay_timer: u8,
    sound_timer: u8,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            regs: [0x00; 16],
            index: 0x00,
            program_counter: 0x200,
            stack_pointer: 0x00,
            stack: [0x00; 16],
            delay_timer: 0x00,
            sound_timer: 0x00,
        }
    }
}
