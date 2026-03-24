use std::fmt::Error;

use crate::internal::{cpu, display::Display, keypad::Keypad};

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

    pub fn fetch(&self, memory: &[u8; 4096]) -> u16 {
        let high_byte = memory[self.program_counter as usize];
        let low_byte = memory[self.program_counter as usize + 1];

        return (high_byte as u16) << 8 | (low_byte as u16);
    }

    pub fn execute(
        &mut self,
        opcode: u16,
        memory: &mut [u8; 4096],
        display: &mut Display,
        keypad: &Keypad,
    ) -> Result<(), String> {
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;

        let x = ((opcode * 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        match (opcode & 0xF000) {
            0x0000 => match opcode {
                0x00E0 => display.clear(),
                0x00EE => CPU::RET(self),

                _ => return Err(format!("Unknown Opcode: {:#06x}", opcode)),
            },

            _ => return Err(format!("Unknown Opcode: {:#06x}", opcode)),
        }
        Ok(())
    }

    // Clear Display Screen
    fn CLS(display: &mut Display) {
        display.clear();
    }

    // Return
    fn RET(cpu: &mut CPU) {
        cpu.program_counter = cpu.stack[cpu.stack_pointer as usize];
        cpu.stack_pointer -= 1;
    }

    // Jump addr
    fn JP(cpu: &mut CPU, addr: u16) {
        cpu.program_counter = addr;
    }

    // Call addr
    fn CALL(cpu: &mut CPU, addr: u16) {
        cpu.stack_pointer += 1;
        cpu.stack[cpu.stack_pointer as usize] = cpu.program_counter;

        cpu.program_counter = addr;
    }

    // Skip next instruction if Vx = kk.
    fn SEVx(cpu: &mut CPU, register: usize, payload: u8) {
        if cpu.regs[register] == payload {
            cpu.program_counter += 2;
        }
    }

    // Skip next instruction if Vx != kk.
    fn SNEVx(cpu: &mut CPU, register: usize, payload: u8) {
        if !(cpu.regs[register] != payload) {
            cpu.program_counter += 2;
        }
    }

    // Skip next instruction if Vx = Vy
    fn SEVxVy(cpu: &mut CPU, register_x: usize, register_y: usize) {
        if cpu.regs[register_x] == cpu.regs[register_y] {
            cpu.program_counter += 2;
        }
    }

    // Set Vx to KK
    fn LDVx(cpu: &mut CPU, register: usize, payload: u8) {
        cpu.regs[register] = payload;
    }

    // Add kk to Vx and store in Vx
    fn ADDVx(cpu: &mut CPU, register: usize, payload: u8) {
        cpu.regs[register] += payload;
    }

    // Store the value of register Vy in Vx
    fn LDVxVy(cpu: &mut CPU, register_x: usize, register_y: usize) {
        cpu.regs[register_x] = cpu.regs[register_y]
    }

    // Set Vx to Vx bitwise or'ed Vy
    fn ORVxVy(cpu: &mut CPU, register_x: usize, register_y: usize) {
        cpu.regs[register_x] = cpu.regs[register_x] | cpu.regs[register_y];
    }

    // Set Vx to Vx btwise and'ed Vy
    fn ANDVxVy(cpu: &mut CPU, register_x: usize, register_y: usize) {
        cpu.regs[register_x] = cpu.regs[register_x] & cpu.regs[register_y];
    }

    // Set Vx to Vx bitwise xor'ed Vy
    fn XORVxVy(cpu: &mut CPU, register_x: usize, register_y: usize) {
        cpu.regs[register_x] = cpu.regs[register_x] ^ cpu.regs[register_y];
    }
}
