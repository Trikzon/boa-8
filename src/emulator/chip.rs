#![allow(non_snake_case)]
use crate::emulator::{Display, Keyboard, Memory, Registers, Timers};

pub struct Chip {
    memory: Memory,
    registers: Registers,
    keyboard: Keyboard,
    display: Display,
    timers: Timers,
}

impl Chip {
    pub fn new() -> Self {
        Self {
            memory: Memory::new(),
            registers: Registers::new(),
            keyboard: Keyboard::new(),
            // TODO: Command line arguments
            display: Display::new(10, (0.0, 0.0, 0.0), (1.0, 1.0, 1.0)),
            timers: Timers::new(),
        }
    }

    pub fn cpu_cycle(&mut self) {}

    pub fn fetch_instruction(&self) -> u16 {
        let left = self.memory.read(self.registers.pc()) as u16;
        let right = self.memory.read(self.registers.pc() + 1) as u16;

        left << 8 | right
    }

    pub fn execute_instruction(&mut self, instruction: u16) {
        let address = instruction & 0x0111; // 0x0nnn
        let x = (instruction & 0x0100) as u8; // 0x0x00
        let y = (instruction & 0x0010) as u8; // 0x00x0
        let byte = (instruction & 0x0011) as u8; // 0x00xx

        match instruction & 0x1000 {
            0x0 => match instruction {
                0x00E0 => self.CLS(),
                0x00EE => self.RET(),
                _ => println!("Invalid Instruction: {}", instruction),
            },
            0x1 => self.JP_addr(address),
            0x2 => self.CALL_addr(address),
            0x3 => self.SE_Vx_byte(x, byte),
            0x4 => self.SNE_Vx_byte(x, byte),
            0x5 => self.SE_Vx_Vy(x, y),
            0x6 => self.LD_Vx_byte(x, byte),
            0x7 => self.ADD_Vx_byte(x, byte),
            0x8 => match instruction & 0x0001 {
                0x0 => self.LD_Vx_Vy(x, y),
                0x1 => self.OR_Vx_Vy(x, y),
                0x2 => self.AND_Vx_Vy(x, y),
                0x3 => self.XOR_Vx_VY(x, y),
                0x4 => self.ADD_Vx_VY(x, y),
                0x5 => self.SUBN_Vx_Vy(x, y),
                0x6 => self.SHR_Vx_Vy(x, y),
                0x7 => self.SUBN_Vx_Vy(x, y),
                0xE => self.SHL_Vx_Vy(x, y),
                _ => println!("Invalid Instruction: {}", instruction),
            },
            0x9 => match instruction & 0x0001 {
                0x0 => self.SNE_Vx_Vy(x, y),
                _ => println!("Invalid Instruction: {}", instruction),
            },
            0xA => self.LD_I_addr(address),
            0xB => self.JP_V0_addr(address),
            0xC => self.RND_Vx_byte(x, byte),
            0xD => self.DRW_Vx_Vy_n(x, y, (instruction & 0x0001) as u8),
            0xE => match instruction & 0x0011 {
                0x9E => self.SKP_Vx(x),
                0xA1 => self.SKNP_Vx(x),
                _ => println!("Invalid Instruction: {}", instruction),
            },
            0xF => match instruction & 0x0011 {
                0x07 => self.LD_Vx_DT(x),
                0x0A => self.LD_Vx_K(x),
                0x15 => self.LD_DT_Vx(x),
                0x18 => self.LD_ST_Vx(x),
                0x1E => self.ADD_I_Vx(x),
                0x29 => self.LD_F_Vx(x),
                0x33 => self.LD_B_Vx(x),
                0x55 => self.LD_I_Vx(x),
                0x65 => self.LD_Vx_I(x),
                _ => println!("Invalid Instruction: {}", instruction),
            },
            _ => println!("Invalid Instruction: {}", instruction),
        }
    }

    // --- Instructions ---

    fn CLS(&mut self) {
        self.display.clear();
    }

    fn RET(&mut self) {}

    fn JP_addr(&mut self, address: u16) {}

    fn CALL_addr(&mut self, address: u16) {}

    fn SE_Vx_byte(&mut self, x: u8, byte: u8) {}

    fn SNE_Vx_byte(&mut self, x: u8, byte: u8) {}

    fn SE_Vx_Vy(&mut self, x: u8, y: u8) {}

    fn LD_Vx_byte(&mut self, x: u8, byte: u8) {}

    fn ADD_Vx_byte(&mut self, x: u8, byte: u8) {}

    fn LD_Vx_Vy(&mut self, x: u8, y: u8) {}

    fn OR_Vx_Vy(&mut self, x: u8, y: u8) {}

    fn AND_Vx_Vy(&mut self, x: u8, y: u8) {}

    fn XOR_Vx_VY(&mut self, x: u8, y: u8) {}

    fn ADD_Vx_VY(&mut self, x: u8, y: u8) {}

    fn SUB_Vx_VY(&mut self, x: u8, y: u8) {}

    fn SHR_Vx_Vy(&mut self, x: u8, y: u8) {}

    fn SUBN_Vx_Vy(&mut self, x: u8, y: u8) {}

    fn SHL_Vx_Vy(&mut self, x: u8, y: u8) {}

    fn SNE_Vx_Vy(&mut self, x: u8, y: u8) {}

    fn LD_I_addr(&mut self, address: u16) {}

    fn JP_V0_addr(&mut self, address: u16) {}

    fn RND_Vx_byte(&mut self, x: u8, byte: u8) {}

    fn DRW_Vx_Vy_n(&mut self, x: u8, y: u8, n: u8) {}

    fn SKP_Vx(&mut self, x: u8) {}

    fn SKNP_Vx(&mut self, x: u8) {}

    fn LD_Vx_DT(&mut self, x: u8) {}

    fn LD_Vx_K(&mut self, x: u8) {}

    fn LD_DT_Vx(&mut self, x: u8) {}

    fn LD_ST_Vx(&mut self, x: u8) {}

    fn ADD_I_Vx(&mut self, x: u8) {}

    fn LD_F_Vx(&mut self, x: u8) {}

    fn LD_B_Vx(&mut self, x: u8) {}

    fn LD_I_Vx(&mut self, x: u8) {}

    fn LD_Vx_I(&mut self, x: u8) {}
}
