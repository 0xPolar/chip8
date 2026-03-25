use std::fmt::Error;

use rand::{Rng, RngExt};

use crate::internal::{display::Display, keypad::Keypad};

pub struct CPU {
    regs: [u8; 16],
    index: u16,
    PC: u16,
    SP: u8,
    stack: [u16; 16],
    DT: u8,
    ST: u8,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            regs: [0x00; 16],
            index: 0x00,
            PC: 0x200, // Program Counter
            SP: 0x00,  // Stack Pointer
            stack: [0x00; 16],
            DT: 0x00, // Delay Timer
            ST: 0x00, // Sound Timer
        }
    }

    pub fn fetch(&self, memory: &[u8; 4096]) -> u16 {
        let high_byte = memory[self.PC as usize];
        let low_byte = memory[self.PC as usize + 1];

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
        cpu.PC = cpu.stack[cpu.SP as usize];
        cpu.SP -= 1;
    }

    // Jump addr
    fn JP(cpu: &mut CPU, addr: u16) {
        cpu.PC = addr;
    }

    // Call addr
    fn CALL(cpu: &mut CPU, addr: u16) {
        cpu.SP += 1;
        cpu.stack[cpu.SP as usize] = cpu.PC;

        cpu.PC = addr;
    }

    // Skip next instruction if Vx = kk.
    fn SEVx(cpu: &mut CPU, Rx: usize, payload: u8) {
        if cpu.regs[Rx] == payload {
            cpu.PC += 2;
        }
    }

    // Skip next instruction if Vx != kk.
    fn SNEVx(cpu: &mut CPU, Rx: usize, payload: u8) {
        if !(cpu.regs[Rx] != payload) {
            cpu.PC += 2;
        }
    }

    // Skip next instruction if Vx = Vy
    fn SEVxVy(cpu: &mut CPU, Rx: usize, Ry: usize) {
        if cpu.regs[Rx] == cpu.regs[Ry] {
            cpu.PC += 2;
        }
    }

    // Set Vx to KK
    fn LDVx(cpu: &mut CPU, Rx: usize, payload: u8) {
        cpu.regs[Rx] = payload;
    }

    // Add kk to Vx and store in Vx
    fn ADDVx(cpu: &mut CPU, Rx: usize, payload: u8) {
        cpu.regs[Rx] += payload;
    }

    // Store the value of Rx Vy in Vx
    fn LDVxVy(cpu: &mut CPU, Rx: usize, Ry: usize) {
        cpu.regs[Rx] = cpu.regs[Ry]
    }

    // Set Vx to Vx bitwise or'ed Vy
    fn ORVxVy(cpu: &mut CPU, Rx: usize, Ry: usize) {
        cpu.regs[Rx] = cpu.regs[Rx] | cpu.regs[Ry];
    }

    // Set Vx to Vx btwise and'ed Vy
    fn ANDVxVy(cpu: &mut CPU, Rx: usize, Ry: usize) {
        cpu.regs[Rx] = cpu.regs[Rx] & cpu.regs[Ry];
    }

    // Set Vx to Vx bitwise xor'ed Vy
    fn XORVxVy(cpu: &mut CPU, Rx: usize, Ry: usize) {
        cpu.regs[Rx] = cpu.regs[Rx] ^ cpu.regs[Ry];
    }

    // Set Vx to Vx + Vy, Vf to carry
    fn CRRYADD(cpu: &mut CPU, Rx: usize, Ry: usize) {
        let sum: u16 = cpu.regs[Rx] as u16 + cpu.regs[Ry] as u16;

        if sum > 255 {
            cpu.regs[0xF] = 1;
        }

        cpu.regs[Rx] = sum as u8;
    }

    // If Vx > Vy, then VF is set to 1, otherwise 0.
    // Then Vy is subtracted from Vx, and the results stored in Vx.
    fn BRWSUB(cpu: &mut CPU, Rx: usize, Ry: usize) {
        if cpu.regs[Rx] > cpu.regs[Ry] {
            cpu.regs[0xF] = 1;
        } else {
            cpu.regs[0xF] = 0;
        }

        cpu.regs[Rx] = cpu.regs[Rx] - cpu.regs[Ry];
    }

    // Set Vx = Vx SHR 1.
    fn SHRVc(cpu: &mut CPU, Rx: usize) {
        if (cpu.regs[Rx] & 0x000F) == 0x01 {
            cpu.regs[0xF] = 1;
        } else {
            cpu.regs[0xF] = 0;
        }

        cpu.regs[Rx] /= 2;
    }

    // If Vy > Vx, then VF is set to 1, otherwise 0.
    // Then Vy is subtracted from Vx, and the results stored in Vx.
    fn BRWSUB2(cpu: &mut CPU, Rx: usize, Ry: usize) {
        if cpu.regs[Ry] > cpu.regs[Rx] {
            cpu.regs[0xF] = 1;
        } else {
            cpu.regs[0xF] = 0;
        }

        cpu.regs[Rx] = cpu.regs[Ry] - cpu.regs[Rx];
    }

    // Set Vx = Vx SHL 1.
    fn SHLVc(cpu: &mut CPU, Rx: usize) {
        if (cpu.regs[Rx] & 0xF0) == 0x10 {
            cpu.regs[0xF] = 1;
        } else {
            cpu.regs[0xF] = 0;
        }

        cpu.regs[Rx] *= 2;
    }

    // Skip next instruction if Vx != Vy
    fn SNEVxVy(cpu: &mut CPU, Rx: usize, Ry: usize) {
        if (cpu.regs[Rx] != cpu.regs[Ry]) {
            cpu.PC += 2;
        }
    }

    // Load value of nnn into index register
    fn LDI(cpu: &mut CPU, payload: u16) {
        cpu.index = payload;
    }

    // Jump to location nnn + V0
    fn JPV0(cpu: &mut CPU, payload: u16) {
        let addr = cpu.regs[0] as u16 + payload;

        cpu.PC = addr;
    }

    // Set Vx = random byte AND kk
    fn RNDVx(cpu: &mut CPU, Rx: usize, payload: u8) {
        let random_number: u8 = rand::rng().random();

        cpu.regs[Rx] = cpu.regs[Rx] & payload;
    }

    // DRW
    fn DRW(
        cpu: &mut CPU,
        memory: &[u8; 4096],
        display: &mut Display,
        Rx: usize,
        Ry: usize,
        n: usize,
    ) {
        let sprite = &memory[cpu.index as usize..(cpu.index as usize + n)];

        let collision = display.draw_sprite(cpu.regs[Rx] as usize, cpu.regs[Ry] as usize, sprite);

        cpu.regs[Rx] = collision as u8;
    }
}
