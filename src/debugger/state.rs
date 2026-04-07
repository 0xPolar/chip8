use std::collections::HashSet;

use crate::internal::chip8::Chip8;

#[allow(non_snake_case)]
#[derive(Clone)]
pub struct EmulatorSnapshot {
    pub regs: [u8; 16],
    pub index: u16,
    pub PC: u16,
    pub SP: u8,
    pub stack: [u16; 16],
    pub DT: u8,
    pub ST: u8,

    pub waiting: bool,
    pub memory: Vec<u8>,
    pub current_opcode: u16,
}

impl Default for EmulatorSnapshot {
    fn default() -> Self {
        Self {
            regs: [0; 16],
            index: 0,
            PC: 0x200,
            SP: 0,
            stack: [0x0; 16],
            DT: 0,
            ST: 0,

            waiting: false,
            memory: vec![0; 4096],
            current_opcode: 0,
        }
    }
}

pub struct DebugState {
    pub snapshot: EmulatorSnapshot,
    pub paused: bool,
    pub step_requested: bool,
    pub breakpoints: HashSet<u16>,
    pub reset_requested: bool,
}

impl DebugState {
    pub fn new() -> Self {
        DebugState {
            snapshot: EmulatorSnapshot::default(),
            paused: false,
            step_requested: false,
            breakpoints: HashSet::new(),
            reset_requested: false,
        }
    }

    pub fn update(&mut self, c: &Chip8) {
        let s = &mut self.snapshot;

        s.regs = *c.registers();
        s.index = c.index_register();
        s.PC = c.program_counter();
        s.SP = c.stack_pointer();
        s.stack = *c.stack();
        s.DT = c.delay_timer();
        s.ST = c.sound_timer();

        s.memory = c.memory().to_vec();
        s.waiting = c.waiting();

        let pc = s.PC as usize;
        if pc + 1 < s.memory.len() {
            s.current_opcode = (s.memory[pc] as u16) << 8 | (s.memory[pc + 1]) as u16;
        }
    }

    pub fn should_execute(&self) -> bool {
        !self.paused || self.step_requested
    }

}
