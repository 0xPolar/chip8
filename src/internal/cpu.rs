#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
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

    waiting: Option<usize>,
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
            waiting: None,
        }
    }

    pub fn decrement_timers(&mut self) {
        if self.DT > 0 {
            self.DT -= 1;
        }
        if self.ST > 0 {
            self.ST -= 1;
        }
    }

    pub fn registers(&self) -> &[u8; 16] {
        &self.regs
    }

    pub fn index(&self) -> u16 {
        self.index
    }

    pub fn program_counter(&self) -> u16 {
        self.PC
    }

    pub fn stack_pointer(&self) -> u8 {
        self.SP
    }

    pub fn stack(&self) -> &[u16; 16] {
        &self.stack
    }

    pub fn delay_timer(&self) -> u8 {
        self.DT
    }

    pub fn sound_timer(&self) -> u8 {
        self.ST
    }

    pub fn waiting(&self) -> bool {
        self.waiting.is_some()
    }

    // pub fn fetch(&mut self, memory: &[u8; 4096]) -> u16 {
    //     let high_byte = memory[self.PC as usize];
    //     let low_byte = memory[self.PC as usize + 1];
    //
    //     self.PC += 2;
    //
    //     return (high_byte as u16) << 8 | (low_byte as u16);
    // }

    pub fn fetch(&mut self, memory: &[u8; 4096]) -> u16 {
        let addr = self.PC as usize & 0xFFF;
        let high_byte = memory[addr];
        let low_byte = memory[(addr + 1) & 0xFFF];

        self.PC = (self.PC + 2) & 0xFFF;

        return (high_byte as u16) << 8 | (low_byte as u16);
    }

    pub fn execute(
        &mut self,
        opcode: u16,
        memory: &mut [u8; 4096],
        display: &mut Display,
        keypad: &Keypad,
    ) -> Result<(), String> {
        // If waiting for a key press, check the keypad and don't execute anything else
        if let Some(rx) = self.waiting {
            CPU::LDVxK(self, keypad, rx);
            return Ok(());
        }

        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF) as u8;
        let n = (opcode & 0x000F) as u8;

        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => CPU::CLS(display),
                0x00EE => CPU::RET(self),
                _ => return Err(format!("Unknown Opcode: {:#06x}", opcode)),
            },
            0x1000 => CPU::JP(self, nnn),
            0x2000 => CPU::CALL(self, nnn),
            0x3000 => CPU::SEVx(self, x, nn),
            0x4000 => CPU::SNEVx(self, x, nn),
            0x5000 => CPU::SEVxVy(self, x, y),
            0x6000 => CPU::LDVx(self, x, nn),
            0x7000 => CPU::ADDVx(self, x, nn),
            0x8000 => match opcode & 0x000F {
                0x0 => CPU::LDVxVy(self, x, y),
                0x1 => CPU::ORVxVy(self, x, y),
                0x2 => CPU::ANDVxVy(self, x, y),
                0x3 => CPU::XORVxVy(self, x, y),
                0x4 => CPU::CRRYADD(self, x, y),
                0x5 => CPU::BRWSUB(self, x, y),
                0x6 => CPU::SHRVc(self, x),
                0x7 => CPU::BRWSUB2(self, x, y),
                0xE => CPU::SHLVc(self, x),
                _ => return Err(format!("Unknown Opcode: {:#06x}", opcode)),
            },
            0x9000 => CPU::SNEVxVy(self, x, y),
            0xA000 => CPU::LDI(self, nnn),
            0xB000 => CPU::JPV0(self, nnn),
            0xC000 => CPU::RNDVx(self, x, nn),
            0xD000 => CPU::DRW(self, memory, display, x, y, n as usize),
            0xE000 => match opcode & 0x00FF {
                0x9E => CPU::SKP(self, keypad, x),
                0xA1 => CPU::SKNP(self, keypad, x),
                _ => return Err(format!("Unknown Opcode: {:#06x}", opcode)),
            },
            0xF000 => match opcode & 0x00FF {
                0x07 => CPU::LDVxDT(self, x),
                0x0A => CPU::LDVxK(self, keypad, x),
                0x15 => CPU::LDDTVx(self, x),
                0x18 => CPU::LDSTVx(self, x),
                0x1E => CPU::ADDIVx(self, x),
                0x29 => CPU::LDFVx(self, x),
                0x33 => CPU::LDBVx(self, memory, x),
                0x55 => CPU::LDIVx(self, memory, x),
                0x65 => CPU::LDVxI(self, memory, x),
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
        if cpu.regs[Rx] != payload {
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
        cpu.regs[Rx] = cpu.regs[Rx].wrapping_add(payload);
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

        cpu.regs[Rx] = sum as u8;
        cpu.regs[0xF] = if sum > 255 { 1 } else { 0 };
    }

    // If Vx > Vy, then VF is set to 1, otherwise 0.
    // Then Vy is subtracted from Vx, and the results stored in Vx.
    fn BRWSUB(cpu: &mut CPU, Rx: usize, Ry: usize) {
        if cpu.regs[Rx] > cpu.regs[Ry] {
            cpu.regs[0xF] = 1;
        } else {
            cpu.regs[0xF] = 0;
        }

        cpu.regs[Rx] = cpu.regs[Rx].wrapping_sub(cpu.regs[Ry]);
    }

    // Set Vx = Vx SHR 1.
    fn SHRVc(cpu: &mut CPU, Rx: usize) {
        if (cpu.regs[Rx] & 0x01) == 0x01 {
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

        cpu.regs[Rx] = cpu.regs[Ry].wrapping_sub(cpu.regs[Rx]);
    }

    // Set Vx = Vx SHL 1.
    fn SHLVc(cpu: &mut CPU, Rx: usize) {
        if (cpu.regs[Rx] & 0x80) != 0 {
            cpu.regs[0xF] = 1;
        } else {
            cpu.regs[0xF] = 0;
        }

        cpu.regs[Rx] = cpu.regs[Rx].wrapping_mul(2);
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

        cpu.regs[Rx] = random_number & payload;
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

        cpu.regs[0xF] = collision as u8;
    }

    // Skip next instruction if key with value of Vx is pressed.
    fn SKP(cpu: &mut CPU, keypad: &Keypad, Rx: usize) {
        if keypad.is_pressed(cpu.regs[Rx] as usize) {
            cpu.PC += 2;
        }
    }

    // Skip next instruction if key with value of Vx is not pressed.
    fn SKNP(cpu: &mut CPU, keypad: &Keypad, Rx: usize) {
        if !keypad.is_pressed(cpu.regs[Rx] as usize) {
            cpu.PC += 2;
        }
    }

    // Set Vx = delay timer value.
    fn LDVxDT(cpu: &mut CPU, Rx: usize) {
        cpu.regs[Rx] = cpu.DT;
    }

    // Wait for a key press, store the value of the key in Vx.
    // All execution stops until a key is pressed.
    fn LDVxK(cpu: &mut CPU, keypad: &Keypad, Rx: usize) {
        for i in 0..16 {
            if keypad.is_pressed(i) {
                cpu.regs[Rx] = i as u8;
                cpu.waiting = None;
                return;
            }
        }
        cpu.waiting = Some(Rx);
    }

    // Set delay timer = Vx.
    fn LDDTVx(cpu: &mut CPU, Rx: usize) {
        cpu.DT = cpu.regs[Rx];
    }

    // Set sound timer = Vx.
    fn LDSTVx(cpu: &mut CPU, Rx: usize) {
        cpu.ST = cpu.regs[Rx];
    }

    // Set I = I + Vx.
    fn ADDIVx(cpu: &mut CPU, Rx: usize) {
        cpu.index += cpu.regs[Rx] as u16;
    }

    // Set I = location of sprite for digit Vx.
    // Font data starts at 0x050, each character is 5 bytes.
    fn LDFVx(cpu: &mut CPU, Rx: usize) {
        cpu.index = 0x050 + (cpu.regs[Rx] as u16 * 5);
    }

    // Store BCD representation of Vx in memory locations I, I+1, and I+2.
    fn LDBVx(cpu: &CPU, memory: &mut [u8; 4096], Rx: usize) {
        let value = cpu.regs[Rx];
        memory[cpu.index as usize] = value / 100;
        memory[cpu.index as usize + 1] = (value / 10) % 10;
        memory[cpu.index as usize + 2] = value % 10;
    }

    // Store registers V0 through Vx in memory starting at location I.
    fn LDIVx(cpu: &CPU, memory: &mut [u8; 4096], Rx: usize) {
        for i in 0..=Rx {
            memory[cpu.index as usize + i] = cpu.regs[i];
        }
    }

    // Read registers V0 through Vx from memory starting at location I.
    fn LDVxI(cpu: &mut CPU, memory: &[u8; 4096], Rx: usize) {
        for i in 0..=Rx {
            cpu.regs[i] = memory[cpu.index as usize + i];
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn new_cpu() -> CPU {
        CPU::new()
    }

    fn new_display() -> Display {
        Display::new()
    }

    fn new_keypad() -> Keypad {
        Keypad::new()
    }

    #[test]
    fn initial_cpu_state() {
        let cpu = new_cpu();
        assert_eq!(cpu.PC, 0x200);
        assert_eq!(cpu.regs, [0u8; 16]);
        assert_eq!(cpu.index, 0);
        assert_eq!(cpu.SP, 0);
        assert_eq!(cpu.DT, 0);
        assert_eq!(cpu.ST, 0);
    }

    // -- CLS (00E0) --
    #[test]
    fn cls_clears_display() {
        let mut display = new_display();
        display.draw_sprite(0, 0, &[0xFF]);
        CPU::CLS(&mut display);
        for x in 0..64 {
            assert!(!display.get_pixel(x, 0));
        }
    }

    // -- RET (00EE) --
    #[test]
    fn ret_pops_stack() {
        let mut cpu = new_cpu();
        cpu.SP = 1;
        cpu.stack[1] = 0x400;
        CPU::RET(&mut cpu);
        assert_eq!(cpu.PC, 0x400);
        assert_eq!(cpu.SP, 0);
    }

    // -- JP (1nnn) --
    #[test]
    fn jp_sets_pc() {
        let mut cpu = new_cpu();
        CPU::JP(&mut cpu, 0x300);
        assert_eq!(cpu.PC, 0x300);
    }

    // -- CALL (2nnn) --
    #[test]
    fn call_pushes_stack_and_jumps() {
        let mut cpu = new_cpu();
        let old_pc = cpu.PC;
        CPU::CALL(&mut cpu, 0x400);
        assert_eq!(cpu.SP, 1);
        assert_eq!(cpu.stack[1], old_pc);
        assert_eq!(cpu.PC, 0x400);
    }

    // -- SE Vx, kk (3xkk) --
    #[test]
    fn sevx_skips_when_equal() {
        let mut cpu = new_cpu();
        let old_pc = cpu.PC;
        cpu.regs[0] = 0x42;
        CPU::SEVx(&mut cpu, 0, 0x42);
        assert_eq!(cpu.PC, old_pc + 2);
    }

    #[test]
    fn sevx_no_skip_when_not_equal() {
        let mut cpu = new_cpu();
        let old_pc = cpu.PC;
        cpu.regs[0] = 0x42;
        CPU::SEVx(&mut cpu, 0, 0x43);
        assert_eq!(cpu.PC, old_pc);
    }

    // -- SNE Vx, kk (4xkk) --
    #[test]
    fn snevx_skips_when_not_equal() {
        let mut cpu = new_cpu();
        let old_pc = cpu.PC;
        cpu.regs[0] = 0x42;
        CPU::SNEVx(&mut cpu, 0, 0x43);
        assert_eq!(cpu.PC, old_pc + 2);
    }

    #[test]
    fn snevx_no_skip_when_equal() {
        let mut cpu = new_cpu();
        let old_pc = cpu.PC;
        cpu.regs[0] = 0x42;
        CPU::SNEVx(&mut cpu, 0, 0x42);
        assert_eq!(cpu.PC, old_pc);
    }

    // -- SE Vx, Vy (5xy0) --
    #[test]
    fn sevxvy_skips_when_equal() {
        let mut cpu = new_cpu();
        let old_pc = cpu.PC;
        cpu.regs[0] = 0x10;
        cpu.regs[1] = 0x10;
        CPU::SEVxVy(&mut cpu, 0, 1);
        assert_eq!(cpu.PC, old_pc + 2);
    }

    #[test]
    fn sevxvy_no_skip_when_not_equal() {
        let mut cpu = new_cpu();
        let old_pc = cpu.PC;
        cpu.regs[0] = 0x10;
        cpu.regs[1] = 0x20;
        CPU::SEVxVy(&mut cpu, 0, 1);
        assert_eq!(cpu.PC, old_pc);
    }

    // -- LD Vx, kk (6xkk) --
    #[test]
    fn ldvx_sets_register() {
        let mut cpu = new_cpu();
        CPU::LDVx(&mut cpu, 0x5, 0xAB);
        assert_eq!(cpu.regs[0x5], 0xAB);
    }

    // -- ADD Vx, kk (7xkk) --
    #[test]
    fn addvx_adds_to_register() {
        let mut cpu = new_cpu();
        cpu.regs[0] = 0x10;
        CPU::ADDVx(&mut cpu, 0, 0x05);
        assert_eq!(cpu.regs[0], 0x15);
    }

    #[test]
    fn addvx_wraps_on_overflow() {
        let mut cpu = new_cpu();
        cpu.regs[0] = 0xFF;
        CPU::ADDVx(&mut cpu, 0, 0x02);
        assert_eq!(cpu.regs[0], 0x01);
    }

    // -- LD Vx, Vy (8xy0) --
    #[test]
    fn ldvxvy_copies_register() {
        let mut cpu = new_cpu();
        cpu.regs[1] = 0xBB;
        CPU::LDVxVy(&mut cpu, 0, 1);
        assert_eq!(cpu.regs[0], 0xBB);
    }

    // -- OR Vx, Vy (8xy1) --
    #[test]
    fn orvxvy_bitwise_or() {
        let mut cpu = new_cpu();
        cpu.regs[0] = 0x0F;
        cpu.regs[1] = 0xF0;
        CPU::ORVxVy(&mut cpu, 0, 1);
        assert_eq!(cpu.regs[0], 0xFF);
    }

    // -- AND Vx, Vy (8xy2) --
    #[test]
    fn andvxvy_bitwise_and() {
        let mut cpu = new_cpu();
        cpu.regs[0] = 0x0F;
        cpu.regs[1] = 0xFF;
        CPU::ANDVxVy(&mut cpu, 0, 1);
        assert_eq!(cpu.regs[0], 0x0F);
    }

    // -- XOR Vx, Vy (8xy3) --
    #[test]
    fn xorvxvy_bitwise_xor() {
        let mut cpu = new_cpu();
        cpu.regs[0] = 0xFF;
        cpu.regs[1] = 0x0F;
        CPU::XORVxVy(&mut cpu, 0, 1);
        assert_eq!(cpu.regs[0], 0xF0);
    }

    // -- ADD Vx, Vy with carry (8xy4) --
    #[test]
    fn crryadd_no_carry() {
        let mut cpu = new_cpu();
        cpu.regs[0] = 0x10;
        cpu.regs[1] = 0x20;
        CPU::CRRYADD(&mut cpu, 0, 1);
        assert_eq!(cpu.regs[0], 0x30);
        assert_eq!(cpu.regs[0xF], 0);
    }

    #[test]
    fn crryadd_with_carry() {
        let mut cpu = new_cpu();
        cpu.regs[0] = 0xFF;
        cpu.regs[1] = 0x02;
        CPU::CRRYADD(&mut cpu, 0, 1);
        assert_eq!(cpu.regs[0], 0x01);
        assert_eq!(cpu.regs[0xF], 1);
    }

    // -- SUB Vx, Vy (8xy5) --
    #[test]
    fn brwsub_no_borrow() {
        let mut cpu = new_cpu();
        cpu.regs[0] = 0x10;
        cpu.regs[1] = 0x05;
        CPU::BRWSUB(&mut cpu, 0, 1);
        assert_eq!(cpu.regs[0], 0x0B);
        assert_eq!(cpu.regs[0xF], 1);
    }

    #[test]
    fn brwsub_with_borrow() {
        let mut cpu = new_cpu();
        cpu.regs[0] = 0x05;
        cpu.regs[1] = 0x10;
        CPU::BRWSUB(&mut cpu, 0, 1);
        assert_eq!(cpu.regs[0], 0xF5); // wrapping subtraction
        assert_eq!(cpu.regs[0xF], 0);
    }

    // -- SHR Vx (8xy6) --
    #[test]
    fn shrvc_lsb_set() {
        let mut cpu = new_cpu();
        cpu.regs[0] = 0x03; // binary 0000_0011, LSB = 1
        CPU::SHRVc(&mut cpu, 0);
        assert_eq!(cpu.regs[0], 0x01);
        assert_eq!(cpu.regs[0xF], 1);
    }

    #[test]
    fn shrvc_lsb_clear() {
        let mut cpu = new_cpu();
        cpu.regs[0] = 0x02; // binary 0000_0010, LSB = 0
        CPU::SHRVc(&mut cpu, 0);
        assert_eq!(cpu.regs[0], 0x01);
        assert_eq!(cpu.regs[0xF], 0);
    }

    // -- SUBN Vx, Vy (8xy7) --
    #[test]
    fn brwsub2_no_borrow() {
        let mut cpu = new_cpu();
        cpu.regs[0] = 0x05;
        cpu.regs[1] = 0x10;
        CPU::BRWSUB2(&mut cpu, 0, 1);
        assert_eq!(cpu.regs[0], 0x0B);
        assert_eq!(cpu.regs[0xF], 1);
    }

    #[test]
    fn brwsub2_with_borrow() {
        let mut cpu = new_cpu();
        cpu.regs[0] = 0x10;
        cpu.regs[1] = 0x05;
        CPU::BRWSUB2(&mut cpu, 0, 1);
        assert_eq!(cpu.regs[0], 0xF5); // wrapping subtraction
        assert_eq!(cpu.regs[0xF], 0);
    }

    // -- SHL Vx (8xyE) --
    #[test]
    fn shlvc_msb_set() {
        let mut cpu = new_cpu();
        cpu.regs[0] = 0x80; // binary 1000_0000, MSB = 1
        CPU::SHLVc(&mut cpu, 0);
        assert_eq!(cpu.regs[0], 0x00); // 0x80 * 2 overflows to 0
        assert_eq!(cpu.regs[0xF], 1);
    }

    #[test]
    fn shlvc_msb_clear() {
        let mut cpu = new_cpu();
        cpu.regs[0] = 0x02; // binary 0000_0010, MSB = 0
        CPU::SHLVc(&mut cpu, 0);
        assert_eq!(cpu.regs[0], 0x04);
        assert_eq!(cpu.regs[0xF], 0);
    }

    // -- SNE Vx, Vy (9xy0) --
    #[test]
    fn snevxvy_skips_when_not_equal() {
        let mut cpu = new_cpu();
        let old_pc = cpu.PC;
        cpu.regs[0] = 0x10;
        cpu.regs[1] = 0x20;
        CPU::SNEVxVy(&mut cpu, 0, 1);
        assert_eq!(cpu.PC, old_pc + 2);
    }

    #[test]
    fn snevxvy_no_skip_when_equal() {
        let mut cpu = new_cpu();
        let old_pc = cpu.PC;
        cpu.regs[0] = 0x10;
        cpu.regs[1] = 0x10;
        CPU::SNEVxVy(&mut cpu, 0, 1);
        assert_eq!(cpu.PC, old_pc);
    }

    // -- LD I, nnn (Annn) --
    #[test]
    fn ldi_sets_index() {
        let mut cpu = new_cpu();
        CPU::LDI(&mut cpu, 0x500);
        assert_eq!(cpu.index, 0x500);
    }

    // -- JP V0, nnn (Bnnn) --
    #[test]
    fn jpv0_jumps_with_offset() {
        let mut cpu = new_cpu();
        cpu.regs[0] = 0x10;
        CPU::JPV0(&mut cpu, 0x300);
        assert_eq!(cpu.PC, 0x310);
    }

    // -- RND Vx, kk (Cxkk) --
    #[test]
    fn rndvx_masks_value() {
        let mut cpu = new_cpu();
        cpu.regs[0] = 0xFF;
        CPU::RNDVx(&mut cpu, 0, 0x0F);
        // Result should be masked to lower nibble at most
        assert_eq!(cpu.regs[0] & 0xF0, 0x00);
    }

    // -- DRW Vx, Vy, n (Dxyn) --
    #[test]
    fn drw_draws_sprite_no_collision() {
        let mut cpu = new_cpu();
        let mut memory = [0u8; 4096];
        let mut display = new_display();
        cpu.index = 0x300;
        memory[0x300] = 0x80; // single pixel top-left
        cpu.regs[0] = 0; // x
        cpu.regs[1] = 0; // y
        CPU::DRW(&mut cpu, &memory, &mut display, 0, 1, 1);
        assert!(display.get_pixel(0, 0));
    }

    #[test]
    fn drw_detects_collision() {
        let mut cpu = new_cpu();
        let mut memory = [0u8; 4096];
        let mut display = new_display();
        cpu.index = 0x300;
        memory[0x300] = 0x80;
        cpu.regs[0] = 0;
        cpu.regs[1] = 0;
        // Draw once — no collision
        CPU::DRW(&mut cpu, &memory, &mut display, 0, 1, 1);
        // Draw again at same spot — collision
        CPU::DRW(&mut cpu, &memory, &mut display, 0, 1, 1);
        assert_eq!(cpu.regs[0xF], 1); // VF set to collision flag
    }

    // -- SKP Vx (Ex9E) --
    #[test]
    fn skp_skips_when_pressed() {
        let mut cpu = new_cpu();
        let mut keypad = new_keypad();
        let old_pc = cpu.PC;
        cpu.regs[0] = 5;
        keypad.press(5);
        CPU::SKP(&mut cpu, &keypad, 0);
        assert_eq!(cpu.PC, old_pc + 2);
    }

    #[test]
    fn skp_no_skip_when_not_pressed() {
        let mut cpu = new_cpu();
        let keypad = new_keypad();
        let old_pc = cpu.PC;
        cpu.regs[0] = 5;
        CPU::SKP(&mut cpu, &keypad, 0);
        assert_eq!(cpu.PC, old_pc);
    }

    // -- SKNP Vx (ExA1) --
    #[test]
    fn sknp_skips_when_not_pressed() {
        let mut cpu = new_cpu();
        let keypad = new_keypad();
        let old_pc = cpu.PC;
        cpu.regs[0] = 5;
        CPU::SKNP(&mut cpu, &keypad, 0);
        assert_eq!(cpu.PC, old_pc + 2);
    }

    #[test]
    fn sknp_no_skip_when_pressed() {
        let mut cpu = new_cpu();
        let mut keypad = new_keypad();
        let old_pc = cpu.PC;
        cpu.regs[0] = 5;
        keypad.press(5);
        CPU::SKNP(&mut cpu, &keypad, 0);
        assert_eq!(cpu.PC, old_pc);
    }

    // -- LD Vx, DT (Fx07) --
    #[test]
    fn ldvxdt_reads_delay_timer() {
        let mut cpu = new_cpu();
        cpu.DT = 0x30;
        CPU::LDVxDT(&mut cpu, 0);
        assert_eq!(cpu.regs[0], 0x30);
    }

    // -- LD Vx, K (Fx0A) --
    #[test]
    fn ldvxk_stores_key_when_pressed() {
        let mut cpu = new_cpu();
        let mut keypad = new_keypad();
        keypad.press(7);
        CPU::LDVxK(&mut cpu, &keypad, 0);
        assert_eq!(cpu.regs[0], 7);
        assert!(cpu.waiting.is_none());
    }

    #[test]
    fn ldvxk_waits_when_no_key_pressed() {
        let mut cpu = new_cpu();
        let keypad = new_keypad();
        CPU::LDVxK(&mut cpu, &keypad, 3);
        assert_eq!(cpu.waiting, Some(3));
    }

    #[test]
    fn ldvxk_execute_halts_while_waiting() {
        let mut cpu = new_cpu();
        let mut memory = [0u8; 4096];
        let mut display = new_display();
        let keypad = new_keypad();
        cpu.waiting = Some(2);
        // Execute should just poll the keypad and return early
        let result = cpu.execute(0x00E0, &mut memory, &mut display, &keypad);
        assert!(result.is_ok());
        // Still waiting since no key pressed
        assert_eq!(cpu.waiting, Some(2));
    }

    #[test]
    fn ldvxk_execute_resumes_on_key_press() {
        let mut cpu = new_cpu();
        let mut memory = [0u8; 4096];
        let mut display = new_display();
        let mut keypad = new_keypad();
        cpu.waiting = Some(2);
        keypad.press(0xA);
        let result = cpu.execute(0x00E0, &mut memory, &mut display, &keypad);
        assert!(result.is_ok());
        assert!(cpu.waiting.is_none());
        assert_eq!(cpu.regs[2], 0xA);
    }

    // -- LD DT, Vx (Fx15) --
    #[test]
    fn lddtvx_sets_delay_timer() {
        let mut cpu = new_cpu();
        cpu.regs[0] = 0x45;
        CPU::LDDTVx(&mut cpu, 0);
        assert_eq!(cpu.DT, 0x45);
    }

    // -- LD ST, Vx (Fx18) --
    #[test]
    fn ldstvx_sets_sound_timer() {
        let mut cpu = new_cpu();
        cpu.regs[0] = 0x20;
        CPU::LDSTVx(&mut cpu, 0);
        assert_eq!(cpu.ST, 0x20);
    }

    // -- ADD I, Vx (Fx1E) --
    #[test]
    fn addivx_adds_to_index() {
        let mut cpu = new_cpu();
        cpu.index = 0x100;
        cpu.regs[0] = 0x10;
        CPU::ADDIVx(&mut cpu, 0);
        assert_eq!(cpu.index, 0x110);
    }

    // -- LD F, Vx (Fx29) --
    #[test]
    fn ldfvx_sets_font_address() {
        let mut cpu = new_cpu();
        cpu.regs[0] = 0; // digit 0
        CPU::LDFVx(&mut cpu, 0);
        assert_eq!(cpu.index, 0x050);

        cpu.regs[0] = 1; // digit 1
        CPU::LDFVx(&mut cpu, 0);
        assert_eq!(cpu.index, 0x055);

        cpu.regs[0] = 0xF; // digit F
        CPU::LDFVx(&mut cpu, 0);
        assert_eq!(cpu.index, 0x050 + 0xF * 5);
    }

    // -- LD B, Vx (Fx33) --
    #[test]
    fn ldbvx_stores_bcd() {
        let mut cpu = new_cpu();
        let mut memory = [0u8; 4096];
        cpu.index = 0x300;
        cpu.regs[0] = 254;
        CPU::LDBVx(&cpu, &mut memory, 0);
        assert_eq!(memory[0x300], 2); // hundreds
        assert_eq!(memory[0x301], 5); // tens
        assert_eq!(memory[0x302], 4); // ones
    }

    #[test]
    fn ldbvx_stores_bcd_zero() {
        let mut cpu = new_cpu();
        let mut memory = [0u8; 4096];
        cpu.index = 0x300;
        cpu.regs[0] = 0;
        CPU::LDBVx(&cpu, &mut memory, 0);
        assert_eq!(memory[0x300], 0);
        assert_eq!(memory[0x301], 0);
        assert_eq!(memory[0x302], 0);
    }

    // -- LD [I], Vx (Fx55) --
    #[test]
    fn ldivx_stores_registers_to_memory() {
        let mut cpu = new_cpu();
        let mut memory = [0u8; 4096];
        cpu.index = 0x300;
        cpu.regs[0] = 0xAA;
        cpu.regs[1] = 0xBB;
        cpu.regs[2] = 0xCC;
        CPU::LDIVx(&cpu, &mut memory, 2);
        assert_eq!(memory[0x300], 0xAA);
        assert_eq!(memory[0x301], 0xBB);
        assert_eq!(memory[0x302], 0xCC);
        assert_eq!(memory[0x303], 0x00); // untouched
    }

    // -- LD Vx, [I] (Fx65) --
    #[test]
    fn ldvxi_loads_registers_from_memory() {
        let mut cpu = new_cpu();
        let mut memory = [0u8; 4096];
        cpu.index = 0x300;
        memory[0x300] = 0x11;
        memory[0x301] = 0x22;
        memory[0x302] = 0x33;
        CPU::LDVxI(&mut cpu, &memory, 2);
        assert_eq!(cpu.regs[0], 0x11);
        assert_eq!(cpu.regs[1], 0x22);
        assert_eq!(cpu.regs[2], 0x33);
        assert_eq!(cpu.regs[3], 0x00); // untouched
    }

    // -- fetch --
    #[test]
    fn fetch_reads_opcode() {
        let mut cpu = new_cpu();
        let mut memory = [0u8; 4096];
        memory[0x200] = 0x12;
        memory[0x201] = 0x34;
        assert_eq!(cpu.fetch(&memory), 0x1234);
    }

    // -- new --
    #[test]
    fn new_cpu_initial_state() {
        let cpu = new_cpu();
        assert_eq!(cpu.PC, 0x200);
        assert_eq!(cpu.SP, 0);
        assert_eq!(cpu.index, 0);
        assert_eq!(cpu.DT, 0);
        assert_eq!(cpu.ST, 0);
        assert!(cpu.waiting.is_none());
        assert_eq!(cpu.regs, [0u8; 16]);
        assert_eq!(cpu.stack, [0u16; 16]);
    }
}
