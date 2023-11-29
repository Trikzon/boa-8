#![allow(non_snake_case)]
use crate::emulator::{Display, Keyboard, Memory, Registers, Timers};
use rand::Rng;

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
        let addr = instruction & 0x0111; // 0x0nnn
        let x = (instruction & 0x0100) as u8; // 0x0x00
        let y = (instruction & 0x0010) as u8; // 0x00x0
        let byte = (instruction & 0x0011) as u8; // 0x00xx
        let nibble = (instruction & 0x0001) as u8; // 0x000x

        match instruction & 0x1000 {
            0x0 => match instruction {
                0x00E0 => self.CLS(),
                0x00EE => self.RET(),
                _ => println!("Invalid Instruction: {}", instruction),
            },
            0x1 => self.JP_addr(addr),
            0x2 => self.CALL_addr(addr),
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
            0xA => self.LD_I_addr(addr),
            0xB => self.JP_V0_addr(addr),
            0xC => self.RND_Vx_byte(x, byte),
            0xD => self.DRW_Vx_Vy_n(x, y, nibble),
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

    /// 00E0 - CLS
    /// Clear the display.
    fn CLS(&mut self) {
        self.display.clear();
    }

    /// 00EE - RET
    /// Return from a subroutine.
    /// 
    /// The interpreter sets the program counter to the address at the top of
    /// the stack, then subtracts 1 from the stack pointer.
    fn RET(&mut self) {
        let pc = self.registers.pop_stack();
        self.registers.set_pc(pc);
    }

    /// 1nnn - JP addr
    /// Jump to location nnn.
    /// 
    /// The interpreter sets the program counter to nnn.
    fn JP_addr(&mut self, addr: u16) {
        self.registers.set_pc(addr);
    }

    /// 2nnn - CALL addr
    /// Call subroutine at nnn.
    /// 
    /// The interpreter increments the stack pointer, then puts the current PC
    /// on the top of the stack. The PC is then set to nnn.
    fn CALL_addr(&mut self, addr: u16) {
        self.registers.push_stack(self.registers.pc());
        self.registers.set_pc(addr);
    }

    /// 3xkk - SE Vx, byte
    /// Skip next instruction if Vx = kk.
    /// The interpreter compares register Vx to kk, and if they are equal,
    /// increments the program counter by 2.
    fn SE_Vx_byte(&mut self, x: u8, byte: u8) {
        if self.registers.v(x) == byte {
            self.registers.increment_pc();
        }
    }

    /// 4xkk - SNE Vx, byte
    /// Skip next instruction if Vx != kk.
    ///
    /// The interpreter compares register Vx to kk, and if they are not equal,
    /// increments the program counter by 2.
    fn SNE_Vx_byte(&mut self, x: u8, byte: u8) {
        if self.registers.v(x) != byte {
            self.registers.increment_pc();
        }
    }

    /// 5xy0 - SE Vx, Vy
    /// Skip next instruction if Vx = Vy.
    /// 
    /// The interpreter compares register Vx to register Vy, and if they are
    /// equal, increments the program counter by 2.
    fn SE_Vx_Vy(&mut self, x: u8, y: u8) {
        if self.registers.v(x) == self.registers.v(y) {
            self.registers.increment_pc();
        }
    }

    /// 6xkk - LD Vx, byte
    /// Set Vx = kk.
    /// 
    /// The interpreter puts the value kk into register Vx.
    fn LD_Vx_byte(&mut self, x: u8, byte: u8) {
        self.registers.set_v(x, byte);
    }

    /// 7xkk - ADD Vx, byte
    /// Set Vx = Vx + kk.
    /// 
    /// Adds the value kk to the value of register Vx, then stores the result
    /// in Vx.
    fn ADD_Vx_byte(&mut self, x: u8, byte: u8) {
        self.registers.set_v(x, self.registers.v(x) + byte);
    }

    /// 8xy0 - LD Vx, Vy
    /// Set Vx = Vy.
    /// 
    /// Stores the value of register Vy in register Vx.
    fn LD_Vx_Vy(&mut self, x: u8, y: u8) {
        self.registers.set_v(x, self.registers.v(y));
    }

    /// 8xy1 - OR Vx, Vy
    /// Set Vx = Vx OR Vy.
    /// 
    /// Performs a bitwise OR on the values of Vx and Vy, then stores the result
    /// in Vx. A bitwise OR compares the corrseponding bits from two values, and
    /// if either bit is 1, then the same bit in the result is also 1.
    /// Otherwise, it is 0.
    fn OR_Vx_Vy(&mut self, x: u8, y: u8) {
        self.registers.set_v(x, self.registers.v(x) | self.registers.v(y));
    }

    /// 8xy2 - AND Vx, Vy
    /// Set Vx = Vx AND Vy.
    /// 
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the
    /// result in Vx. A bitwise AND compares the corrseponding bits from two
    /// values, and if both bits are 1, then the same bit in the result is also
    /// 1. Otherwise, it is 0.
    fn AND_Vx_Vy(&mut self, x: u8, y: u8) {
        self.registers.set_v(x, self.registers.v(x) & self.registers.v(y));
    }

    /// 8xy3 - XOR Vx, Vy
    /// Set Vx = Vx XOR Vy.
    /// 
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores
    /// the result in Vx. An exclusive OR compares the corrseponding bits from
    /// two values, and if the bits are not both the same, then the
    /// corresponding bit in the result is set to 1. Otherwise, it is 0.
    fn XOR_Vx_VY(&mut self, x: u8, y: u8) {
        self.registers.set_v(x, self.registers.v(x) ^ self.registers.v(y));
    }

    /// 8xy4 - ADD Vx, Vy
    /// Set Vx = Vx + Vy, set VF = carry.
    /// 
    /// The values of Vx and Vy are added together. If the result is greater
    /// than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest
    /// 8 bits of the result are kept, and stored in Vx.
    fn ADD_Vx_VY(&mut self, x: u8, y: u8) {
        let sum = self.registers.v(x) as u16 + self.registers.v(y) as u16;

        self.registers.set_vf((sum > 255) as u8);

        self.registers.set_v(x, (sum & 0x00FF) as u8);
    }

    /// 8xy5 - SUB Vx, Vy
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    /// 
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from
    /// Vx, and the results stored in Vx.
    fn SUB_Vx_VY(&mut self, x: u8, y: u8) {
        self.registers.set_vf((self.registers.v(x) > self.registers.v(y)) as u8);

        self.registers.set_v(x, self.registers.v(x) - self.registers.v(y));
    }

    /// 8xy6 - SHR Vx {, Vy}
    /// Set Vx = Vx SHR 1.
    /// 
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise
    /// 0. Then Vx is divided by 2.
    fn SHR_Vx_Vy(&mut self, x: u8, _y: u8) {
        self.registers.set_vf(self.registers.v(x) & 0b00000001);

        self.registers.set_v(x, self.registers.v(x) >> 1);
    }

    /// 8xy7 - SUBN Vx, Vy
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    /// 
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from
    /// Vy, and the results stored in Vx.
    fn SUBN_Vx_Vy(&mut self, x: u8, y: u8) {
        self.registers.set_vf((self.registers.v(y) > self.registers.v(x)) as u8);

        self.registers.set_v(x, self.registers.v(y) - self.registers.v(x));
    }

    /// 8xyE - SHL Vx {, Vy}
    /// Set Vx = Vx SHL 1.
    /// 
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise
    /// to 0. Then Vx is multiplied by 2.
    fn SHL_Vx_Vy(&mut self, x: u8, _y: u8) {
        self.registers.set_vf((self.registers.v(x) & 0b10000000 == 0b10000000) as u8);

        self.registers.set_v(x, self.registers.v(x) << 1);
    }

    /// 9xy0 - SNE Vx, Vy
    /// Skip next instruction if Vx != Vy.
    /// 
    /// The values of Vx and Vy are compared, and if they are not equal, the
    /// program counter is increased by 2.
    fn SNE_Vx_Vy(&mut self, x: u8, y: u8) {
        if self.registers.v(x) != self.registers.v(y) {
            self.registers.increment_pc();
        }
    }

    /// Annn - LD I, addr
    /// Set I = nnn.
    /// 
    /// The value of register I is set to nnn.
    fn LD_I_addr(&mut self, addr: u16) {
        self.registers.set_i(addr);
    }

    /// Bnnn - JP V0, addr
    /// Jump to location nnn + V0.
    /// 
    /// The program counter is set to nnn plus the value of V0.
    fn JP_V0_addr(&mut self, addr: u16) {
        self.registers.set_pc(self.registers.v(0) as u16 + addr);
    }

    /// Cxkk - RND Vx, byte
    /// Set Vx = random byte AND kk.
    /// 
    /// The interpreter generates a random number from 0 to 255, which is then
    /// ANDed with the value kk. The results are stored in Vx. See instruction
    /// 8xy2 for more information on AND.
    fn RND_Vx_byte(&mut self, x: u8, byte: u8) {
        let rand_u8: u8 = rand::thread_rng().gen();

        self.registers.set_v(x, rand_u8 & byte);
    }

    /// Dxyn - DRW Vx, Vy, nibble
    /// Display n-byte sprite starting at memory location I at (Vx, Vy),
    /// set VF = collision.
    /// 
    /// The interpreter reads n bytes from memory, starting at the address
    /// stored in I. These bytes are then displayed as sprites on screen at
    /// coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If
    /// this causes any pixels to be erased, VF is set to 1, otherwise it is
    /// set to 0. If the sprite is positioned so part of it is outside the
    /// coordinates of the display, it wraps around to the opposite side of
    /// the screen. See instruction 8xy3 for more information on XOR, and
    /// section 2.4, Display, for more information on the Chip-8 screen and
    /// sprites.
    fn DRW_Vx_Vy_n(&mut self, x: u8, y: u8, nibble: u8) {
        let mut sprite: Vec<u8> = Vec::new();

        for i in 0..nibble {
            sprite.push(self.memory.read(self.registers.i() + i as u16));
        }

        let collision = self.display.draw_sprite(self.registers.v(x), self.registers.v(y), sprite.as_slice());

        self.registers.set_vf(collision as u8);
    }

    /// Ex9E - SKP Vx
    /// Skip next instruction if key with the value of Vx is pressed.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is
    /// currently in the down position, PC is increased by 2.
    fn SKP_Vx(&mut self, _x: u8) {
        println!("SKP_Vx is unimplemented.");
    }

    /// ExA1 - SKNP Vx
    /// Skip next instruction if key with the value of Vx is not pressed.
    /// 
    /// Checks the keyboard, and if the key corresponding to the value of Vx is
    /// currently in the up position, PC is increased by 2.
    fn SKNP_Vx(&mut self, _x: u8) {
        println!("SKP_Vx is unimplemented.");
    }

    /// Fx07 - LD Vx, DT
    /// Set Vx = delay timer value.
    /// 
    /// The value of DT is placed into Vx.
    fn LD_Vx_DT(&mut self, x: u8) {
        self.registers.set_v(x, self.timers.delay());
    }

    /// Fx0A - LD Vx, K
    /// Wait for a key press, store the value of the key in Vx.
    /// 
    /// All execution stops until a key is pressed, then the value of that key
    /// is stored in Vx.
    fn LD_Vx_K(&mut self, _x: u8) {
        println!("LD_Vx_K is unimplemented.");
    }

    /// Fx15 - LD DT, Vx
    /// Set delay timer = Vx.
    /// 
    /// DT is set equal to the value of Vx.
    fn LD_DT_Vx(&mut self, x: u8) {
        self.timers.set_delay(self.registers.v(x));
    }

    /// Fx18 - LD ST, Vx
    /// Set sound timer = Vx.
    /// 
    /// ST is set equal to the value of Vx.
    fn LD_ST_Vx(&mut self, x: u8) {
        self.timers.set_sound(self.registers.v(x));
    }

    /// Fx1E - ADD I, Vx
    /// Set I = I + Vx.
    /// 
    /// The values of I and Vx are added, and the results are stored in I.
    fn ADD_I_Vx(&mut self, x: u8) {
        self.registers.set_i(self.registers.i() + self.registers.v(x) as u16);
    }

    /// Fx29 - LD F, Vx
    /// Set I = location of sprite for digit Vx.
    /// 
    /// The value of I is set to the location for the hexadecimal sprite
    /// corresponding to the value of Vx. See section 2.4, Display, for more
    /// information on the Chip-8 hexadecimal font.
    fn LD_F_Vx(&mut self, x: u8) {
        self.registers.set_i(self.registers.v(x) as u16 * 5);
    }

    /// Fx33 - LD B, Vx
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    /// 
    /// The interpreter takes the decimal value of Vx, and places the hundreds
    /// digit in memory at location in I, the tens digit at location I+1, and
    /// the ones digit at location I+2.
    fn LD_B_Vx(&mut self, x: u8) {
        let value = self.registers.v(x);
        let (hundreds, value) = (value / 100, value % 100);
        let (tens, ones) = (value / 10, value % 10);

        self.memory.write(self.registers.i() + 0, hundreds);
        self.memory.write(self.registers.i() + 1, tens);
        self.memory.write(self.registers.i() + 2, ones);
    }

    /// Fx55 - LD [I], Vx
    /// Store registers V0 through Vx in memory starting at location I.
    ///
    /// The interpreter copies the values of registers V0 through Vx into
    /// memory, starting at the address in I.
    fn LD_I_Vx(&mut self, x: u8) {
        for i in 0..=x {
            self.memory.write(self.registers.i() + i as u16, self.registers.v(i));
        }
    }

    /// Fx65 - LD Vx, [I]
    /// Read registers V0 through Vx from memory starting at location I.
    /// 
    /// The interpreter reads values from memory starting at location I into
    /// registers V0 through Vx.
    fn LD_Vx_I(&mut self, x: u8) {
        for i in 0..=x {
            self.registers.set_v(i, self.memory.read(self.registers.i() + i as u16));
        }
    }
}
