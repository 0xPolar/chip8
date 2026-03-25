use std::fmt::Error;

use crate::internal::{display::Display, keypad::Keypad};

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
                0x00E0 => CPU::CLS(display),
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
    fn SEVx(cpu: &mut CPU, rX: usize, payload: u8) {
        if cpu.regs[rX] == payload {
            cpu.program_counter += 2;
        }
    }

    // Skip next instruction if Vx != kk.
    fn SNEVx(cpu: &mut CPU, rX: usize, payload: u8) {
        if !(cpu.regs[rX] != payload) {
            cpu.program_counter += 2;
        }
    }

    // Skip next instruction if Vx = Vy
    fn SEVxVy(cpu: &mut CPU, rX: usize, rY: usize) {
        if cpu.regs[rX] == cpu.regs[rY] {
            cpu.program_counter += 2;
        }
    }

    // Set Vx to KK
    fn LDVx(cpu: &mut CPU, rX: usize, payload: u8) {
        cpu.regs[rX] = payload;
    }

    // Add kk to Vx and store in Vx
    fn ADDVx(cpu: &mut CPU, rX: usize, payload: u8) {
        cpu.regs[rX] += payload;
    }

    // Store the value of rX Vy in Vx
    fn LDVxVy(cpu: &mut CPU, rX: usize, rY: usize) {
        cpu.regs[rX] = cpu.regs[rY]
    }

    // Set Vx to Vx bitwise or'ed Vy
    fn ORVxVy(cpu: &mut CPU, rX: usize, rY: usize) {
        cpu.regs[rX] = cpu.regs[rX] | cpu.regs[rY];
    }

    // Set Vx to Vx btwise and'ed Vy
    fn ANDVxVy(cpu: &mut CPU, rX: usize, rY: usize) {
        cpu.regs[rX] = cpu.regs[rX] & cpu.regs[rY];
    }

    // Set Vx to Vx bitwise xor'ed Vy
    fn XORVxVy(cpu: &mut CPU, rX: usize, rY: usize) {
        cpu.regs[rX] = cpu.regs[rX] ^ cpu.regs[rY];
    }

    // Set Vx to Vx + Vy, Vf to carry
    fn CRRYADD(cpu: &mut CPU, rX: usize, rY: usize) {
        let sum: u16 = cpu.regs[rX] as u16 + cpu.regs[rY] as u16;

        if sum > 255 {
            cpu.regs[0xF] = 1;
        }

        cpu.regs[rX] = sum as u8;
    }

    // If Vx > Vy, then VF is set to 1, otherwise 0.
    // Then Vy is subtracted from Vx, and the results stored in Vx.
    fn BRWSUB(cpu: &mut CPU, rX: usize, rY: usize) {
        if cpu.regs[rX] > cpu.regs[rY] {
            cpu.regs[0xF] = 1;
        } else {
            cpu.regs[0xF] = 0;
        }

        cpu.regs[rX] = cpu.regs[rX] - cpu.regs[rY];
    }

    // Set Vx = Vx SHR 1.
    fn SHRVc(cpu: &mut CPU, rX: usize) {
        if (cpu.regs[rX] & 0x000F) == 0x01 {
            cpu.regs[0xF] = 1;
        } else {
            cpu.regs[0xF] = 0;
        }

        cpu.regs[rX] /= 2;
    }
}
