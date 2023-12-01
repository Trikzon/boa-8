#![allow(non_snake_case)]
use std::borrow::BorrowMut;

use crate::emulator::{Display, Keyboard, Memory, Registers, Timers};
use ears::AudioController;
use rand::Rng;

const INSTRUCTIONS_PER_CYCLE: usize = 10;

pub struct Chip {
    memory: Memory,
    registers: Registers,
    keyboard: Keyboard,
    display: Display,
    timers: Timers,
    sound: Option<ears::Sound>,
    paused: bool,
    waiting_for_key: bool,
    first_instruction: bool,
}

impl Chip {
    pub fn new() -> Self {
        let sound = match ears::Sound::new("./sound/440hz.wav") {
            Ok(mut sound) => {
                sound.set_looping(true);
                Some(sound)
            },
            Err(err) => {
                println!("{}", err);
                None
            }
        };

        Self {
            memory: Memory::new(),
            registers: Registers::new(),
            keyboard: Keyboard::new(),
            // TODO: Command line arguments
            display: Display::new(10, (0.0, 0.0, 0.0), (1.0, 1.0, 1.0)),
            timers: Timers::new(),
            sound,
            paused: false,
            waiting_for_key: false,
            first_instruction: true,
        }
    }

    pub fn cpu_cycle(&mut self) {
        self.first_instruction = true;

        for _ in 0..INSTRUCTIONS_PER_CYCLE {
            if !self.paused() {
                let instruction = self.fetch_instruction();
                self.execute_instruction(instruction);
            }
            self.first_instruction = false;
        }

        if let Some(sound) = self.sound.borrow_mut() {
            if self.timers.sound() > 0  && !sound.is_playing() {
                sound.play();
            } else if self.timers.sound() == 0 && sound.is_playing() {
                sound.stop();
            }
        }

        if !self.paused() {
            self.timers.update();
            self.keyboard.update();
        }

    }

    pub fn load_rom(&mut self, rom_data: &[u8]) {
        self.memory.load_rom(rom_data);
    }

    pub fn load_rom_from_path(&mut self, path: &std::path::Path) -> std::io::Result<()> {
        let mut file = std::fs::File::open(path)?;
        let mut rom_data = Vec::new();

        std::io::Read::read_to_end(&mut file, &mut rom_data)?;

        self.load_rom(&rom_data);

        Ok(())
    }

    pub fn process_input(&mut self, input: glutin::event::KeyboardInput) {
        self.keyboard.process_input(input);
    }

    pub fn display(&self) -> &Display {
        &self.display
    }

    pub fn paused(&self) -> bool {
        self.paused
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.paused = paused;
    }

    fn fetch_instruction(&mut self) -> u16 {
        let left = self.memory.read(self.registers.pc()) as u16;
        let right = self.memory.read(self.registers.pc() + 1) as u16;

        self.registers.increment_pc();

        left << 8 | right
    }

    fn execute_instruction(&mut self, instruction: u16) {
        let addr = instruction & 0x0FFF; // 0x0nnn
        let x = ((instruction & 0x0F00) >> 8) as u8; // 0x0x00
        let y = ((instruction & 0x00F0) >> 4) as u8; // 0x00x0
        let byte = (instruction & 0x00FF) as u8; // 0x00xx
        let nibble = (instruction & 0x000F) as u8; // 0x000x

        match instruction & 0xF000 {
            0x0000 => match instruction {
                0x00E0 => self.CLS(),
                0x00EE => self.RET(),
                _ => self.debug_println_instruction("INVD", format!("{:<#06x}", instruction))
            },
            0x1000 => self.JP_addr(addr),
            0x2000 => self.CALL_addr(addr),
            0x3000 => self.SE_Vx_byte(x, byte),
            0x4000 => self.SNE_Vx_byte(x, byte),
            0x5000 => self.SE_Vx_Vy(x, y),
            0x6000 => self.LD_Vx_byte(x, byte),
            0x7000 => self.ADD_Vx_byte(x, byte),
            0x8000 => match instruction & 0x000F {
                0x0 => self.LD_Vx_Vy(x, y),
                0x1 => self.OR_Vx_Vy(x, y),
                0x2 => self.AND_Vx_Vy(x, y),
                0x3 => self.XOR_Vx_Vy(x, y),
                0x4 => self.ADD_Vx_Vy(x, y),
                0x5 => self.SUB_Vx_Vy(x, y),
                0x6 => self.SHR_Vx_Vy(x, y),
                0x7 => self.SUBN_Vx_Vy(x, y),
                0xE => self.SHL_Vx_Vy(x, y),
                _ => self.debug_println_instruction("INVD", format!("{:<#06x}", instruction))
            },
            0x9000 => match instruction & 0x000F {
                0x0 => self.SNE_Vx_Vy(x, y),
                _ => self.debug_println_instruction("INVD", format!("{:<#06x}", instruction))
            },
            0xA000 => self.LD_I_addr(addr),
            0xB000 => self.JP_V0_addr(addr),
            0xC000 => self.RND_Vx_byte(x, byte),
            0xD000 => self.DRW_Vx_Vy_n(x, y, nibble),
            0xE000 => match instruction & 0x00FF {
                0x9E => self.SKP_Vx(x),
                0xA1 => self.SKNP_Vx(x),
                _ => self.debug_println_instruction("INVD", format!("{:<#06x}", instruction))
            },
            0xF000 => match instruction & 0x00FF {
                0x07 => self.LD_Vx_DT(x),
                0x0A => self.LD_Vx_K(x),
                0x15 => self.LD_DT_Vx(x),
                0x18 => self.LD_ST_Vx(x),
                0x1E => self.ADD_I_Vx(x),
                0x29 => self.LD_F_Vx(x),
                0x33 => self.LD_B_Vx(x),
                0x55 => self.LD_I_Vx(x),
                0x65 => self.LD_Vx_I(x),
                _ => self.debug_println_instruction("INVD", format!("{:<#06x}", instruction))
            },
            _ => self.debug_println_instruction("INVD", format!("{:<#06x}", instruction))
        }
    }

    fn debug_println_instruction(&self, instruction: impl Into<String>, description: impl Into<String>) {
        println!("{:<#05x}: {:<16} # {}", self.registers.pc() - 2, instruction.into(), description.into());
    }

    // --- Instructions ---

    /// 00E0 - CLS
    /// Clear the display.
    fn CLS(&mut self) {
        self.debug_println_instruction("CLS", "Clear the display.");
        self.display.clear();
    }

    /// 00EE - RET
    /// Return from a subroutine.
    /// 
    /// The interpreter sets the program counter to the address at the top of
    /// the stack, then subtracts 1 from the stack pointer.
    fn RET(&mut self) {
        self.debug_println_instruction("RET", "Return from a subroutine.");

        let pc = self.registers.pop_stack();
        self.registers.set_pc(pc);
    }

    /// 1nnn - JP addr
    /// Jump to location nnn.
    /// 
    /// The interpreter sets the program counter to nnn.
    fn JP_addr(&mut self, addr: u16) {
        self.debug_println_instruction(format!("JP   {:#05x}", addr), "The interpreter sets the program counter to addr.");

        if addr == self.registers.pc() - 2 {
            self.set_paused(true);
            self.debug_println_instruction("PAUS", "The previous instruction jumped to its own address.")
        }

        self.registers.set_pc(addr);
    }

    /// 2nnn - CALL addr
    /// Call subroutine at nnn.
    /// 
    /// The interpreter increments the stack pointer, then puts the current PC
    /// on the top of the stack. The PC is then set to nnn.
    fn CALL_addr(&mut self, addr: u16) {
        self.debug_println_instruction(format!("CALL {:#05x}", addr), "Call subroutine at addr.");

        self.registers.push_stack(self.registers.pc());
        self.registers.set_pc(addr);
    }

    /// 3xkk - SE Vx, byte
    /// Skip next instruction if Vx = kk.
    /// The interpreter compares register Vx to kk, and if they are equal,
    /// increments the program counter by 2.
    fn SE_Vx_byte(&mut self, x: u8, byte: u8) {
        self.debug_println_instruction(format!("SE   V{:01x}, {:#04x}", x, byte), "Skip next instruction if Vx = byte.");

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
        self.debug_println_instruction(format!("SNE  V{:01x}, {:#04x}", x, byte), "Skip next instruction if Vx != byte.");

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
        self.debug_println_instruction(format!("SE   V{:01x}, V{:01x}", x, y), "Skip next instruction if Vx = Vy.");

        if self.registers.v(x) == self.registers.v(y) {
            self.registers.increment_pc();
        }
    }

    /// 6xkk - LD Vx, byte
    /// Set Vx = kk.
    /// 
    /// The interpreter puts the value kk into register Vx.
    fn LD_Vx_byte(&mut self, x: u8, byte: u8) {
        self.debug_println_instruction(format!("LD   V{:01x}, {:#04x}", x, byte), "Set Vx = byte.");

        self.registers.set_v(x, byte);
    }

    /// 7xkk - ADD Vx, byte
    /// Set Vx = Vx + kk.
    /// 
    /// Adds the value kk to the value of register Vx, then stores the result
    /// in Vx.
    fn ADD_Vx_byte(&mut self, x: u8, byte: u8) {
        self.debug_println_instruction(format!("ADD  V{:01x}, {:#04x}", x, byte), "Set Vx = Vx + byte.");

        self.registers.set_v(x, self.registers.v(x).wrapping_add(byte));
    }

    /// 8xy0 - LD Vx, Vy
    /// Set Vx = Vy.
    /// 
    /// Stores the value of register Vy in register Vx.
    fn LD_Vx_Vy(&mut self, x: u8, y: u8) {
        self.debug_println_instruction(format!("LD   V{:01x}, V{:01x}", x, y), "Set Vx = Vy.");

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
        self.debug_println_instruction(format!("OR   V{:01x}, V{:01x}", x, y), "Set Vx = Vx OR Vy.");

        self.registers.set_v(x, self.registers.v(x) | self.registers.v(y));
        self.registers.set_vf(0);   // According to the chip-8-test-suite: AND, OR, and XOR should set the flag register to 0.
    }

    /// 8xy2 - AND Vx, Vy
    /// Set Vx = Vx AND Vy.
    /// 
    /// Performs a bitwise AND on the values of Vx and Vy, then stores the
    /// result in Vx. A bitwise AND compares the corrseponding bits from two
    /// values, and if both bits are 1, then the same bit in the result is also
    /// 1. Otherwise, it is 0.
    fn AND_Vx_Vy(&mut self, x: u8, y: u8) {
        self.debug_println_instruction(format!("AND  V{:01x}, V{:01x}", x, y), "Set Vx = Vx AND Vy.");

        self.registers.set_v(x, self.registers.v(x) & self.registers.v(y));
        self.registers.set_vf(0);   // According to the chip-8-test-suite: AND, OR, and XOR should set the flag register to 0.
    }

    /// 8xy3 - XOR Vx, Vy
    /// Set Vx = Vx XOR Vy.
    /// 
    /// Performs a bitwise exclusive OR on the values of Vx and Vy, then stores
    /// the result in Vx. An exclusive OR compares the corrseponding bits from
    /// two values, and if the bits are not both the same, then the
    /// corresponding bit in the result is set to 1. Otherwise, it is 0.
    fn XOR_Vx_Vy(&mut self, x: u8, y: u8) {
        self.debug_println_instruction(format!("XOR  V{:01x}, V{:01x}", x, y), "Set Vx = Vx XOR Vy.");

        self.registers.set_v(x, self.registers.v(x) ^ self.registers.v(y));
        self.registers.set_vf(0);   // According to the chip-8-test-suite: AND, OR, and XOR should set the flag register to 0.
    }

    /// 8xy4 - ADD Vx, Vy
    /// Set Vx = Vx + Vy, set VF = carry.
    /// 
    /// The values of Vx and Vy are added together. If the result is greater
    /// than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest
    /// 8 bits of the result are kept, and stored in Vx.
    fn ADD_Vx_Vy(&mut self, x: u8, y: u8) {
        self.debug_println_instruction(format!("ADD  V{:01x}, V{:01x}", x, y), "Set Vx = Vx + Vy, set VF = carry.");

        let sum = self.registers.v(x) as u16 + self.registers.v(y) as u16;

        self.registers.set_v(x, (sum & 0x00FF) as u8);

        self.registers.set_vf((sum > 255) as u8);
    }

    /// 8xy5 - SUB Vx, Vy
    /// Set Vx = Vx - Vy, set VF = NOT borrow.
    /// 
    /// If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from
    /// Vx, and the results stored in Vx.
    fn SUB_Vx_Vy(&mut self, x: u8, y: u8) {
        self.debug_println_instruction(format!("SUB  V{:01x}, V{:01x}", x, y), "Set Vx = Vx - Vy, set VF = NOT borrow.");

        let not_borrow = self.registers.v(x) >= self.registers.v(y);

        self.registers.set_v(x, self.registers.v(x).wrapping_sub(self.registers.v(y)));

        self.registers.set_vf(not_borrow as u8);
    }

    /// 8xy6 - SHR Vx {, Vy}
    /// Set Vx = Vx SHR 1.
    /// 
    /// If the least-significant bit of Vx is 1, then VF is set to 1, otherwise
    /// 0. Then Vx is divided by 2.
    fn SHR_Vx_Vy(&mut self, x: u8, y: u8) {
        self.debug_println_instruction(format!("SHR  V{:01x} {{,V{:01x}}}", x, y), "Set Vx = Vy SHR 1, VF = lost bit.");

        let v_y = self.registers.v(y);
        self.registers.set_v(x, v_y >> 1);

        self.registers.set_vf(v_y & 0b00000001);
    }

    /// 8xy7 - SUBN Vx, Vy
    /// Set Vx = Vy - Vx, set VF = NOT borrow.
    /// 
    /// If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from
    /// Vy, and the results stored in Vx.
    fn SUBN_Vx_Vy(&mut self, x: u8, y: u8) {
        self.debug_println_instruction(format!("SUBN V{:01x}, V{:01x}", x, y), "Set Vx = Vy - Vx, set VF = NOT borrow.");

        let not_borrow = self.registers.v(y) >= self.registers.v(x);

        self.registers.set_v(x, self.registers.v(y).wrapping_sub(self.registers.v(x)));
        self.registers.set_vf(not_borrow as u8);
    }

    /// 8xyE - SHL Vx {, Vy}
    /// Set Vx = Vx SHL 1.
    /// 
    /// If the most-significant bit of Vx is 1, then VF is set to 1, otherwise
    /// to 0. Then Vx is multiplied by 2.
    fn SHL_Vx_Vy(&mut self, x: u8, y: u8) {
        self.debug_println_instruction(format!("SHL  V{:01x} {{, V{:01x}}}", x, y), "Set Vy = Vx SHL 1, VF = lost bit.");

        let v_y = self.registers.v(y);
        self.registers.set_v(x, v_y << 1);

        self.registers.set_vf((v_y & 0b10000000 == 0b10000000) as u8);
    }

    /// 9xy0 - SNE Vx, Vy
    /// Skip next instruction if Vx != Vy.
    /// 
    /// The values of Vx and Vy are compared, and if they are not equal, the
    /// program counter is increased by 2.
    fn SNE_Vx_Vy(&mut self, x: u8, y: u8) {
        self.debug_println_instruction(format!("SNE  V{:01x}, V{:01x}", x, y), "Skip next instruction if Vx != Vy.");

        if self.registers.v(x) != self.registers.v(y) {
            self.registers.increment_pc();
        }
    }

    /// Annn - LD I, addr
    /// Set I = nnn.
    /// 
    /// The value of register I is set to nnn.
    fn LD_I_addr(&mut self, addr: u16) {
        self.debug_println_instruction(format!("LD   I, {:#05x}", addr), "Set I = addr.");

        self.registers.set_i(addr);
    }

    /// Bnnn - JP V0, addr
    /// Jump to location nnn + V0.
    /// 
    /// The program counter is set to nnn plus the value of V0.
    fn JP_V0_addr(&mut self, addr: u16) {
        self.debug_println_instruction(format!("JP   V0, {:#05x}", addr), "Jump to the location addr + V0.");

        self.registers.set_pc((self.registers.v(0) as u16).wrapping_add(addr));
    }

    /// Cxkk - RND Vx, byte
    /// Set Vx = random byte AND kk.
    /// 
    /// The interpreter generates a random number from 0 to 255, which is then
    /// ANDed with the value kk. The results are stored in Vx. See instruction
    /// 8xy2 for more information on AND.
    fn RND_Vx_byte(&mut self, x: u8, byte: u8) {
        self.debug_println_instruction(format!("RND  V{:01x}, {:#04x}", x, byte), "Set Vx = random byte AND byte.");

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
        if !self.first_instruction {
            self.debug_println_instruction("WAIT", "Wait for the start of the cycle to draw a sprite.");
            self.registers.set_pc(self.registers.pc() - 2);
            return;
        }

        self.debug_println_instruction(format!("DRW  V{:01x}, V{:01x}, {:#03x}", x, y, nibble), "Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.");

        let mut sprite: Vec<u8> = Vec::new();

        for i in 0..nibble {
            sprite.push(self.memory.read(self.registers.i().wrapping_add(i as u16)));
        }

        let collision = self.display.draw_sprite(self.registers.v(x), self.registers.v(y), sprite.as_slice());

        self.registers.set_vf(collision as u8);
    }

    /// Ex9E - SKP Vx
    /// Skip next instruction if key with the value of Vx is pressed.
    ///
    /// Checks the keyboard, and if the key corresponding to the value of Vx is
    /// currently in the down position, PC is increased by 2.
    fn SKP_Vx(&mut self, x: u8) {
        self.debug_println_instruction(format!("SKP  V{:01x}, K", x), "Skip next instruction if key with the value of Vx is pressed.");

        if self.keyboard.is_pressed(self.registers.v(x)) {
            self.registers.increment_pc();
        }
    }

    /// ExA1 - SKNP Vx
    /// Skip next instruction if key with the value of Vx is not pressed.
    /// 
    /// Checks the keyboard, and if the key corresponding to the value of Vx is
    /// currently in the up position, PC is increased by 2.
    fn SKNP_Vx(&mut self, x: u8) {
        self.debug_println_instruction(format!("SKNP V{:01x}, K", x), "Skip next instruction if key with the value of Vx is not pressed.");

        if !self.keyboard.is_pressed(self.registers.v(x)) {
            self.registers.increment_pc();
        }
    }

    /// Fx07 - LD Vx, DT
    /// Set Vx = delay timer value.
    /// 
    /// The value of DT is placed into Vx.
    fn LD_Vx_DT(&mut self, x: u8) {
        self.debug_println_instruction(format!("LD   V{:01x}, DT", x), "Set Vx = delay timer value.");

        self.registers.set_v(x, self.timers.delay());
    }

    /// Fx0A - LD Vx, K
    /// Wait for a key press, store the value of the key in Vx.
    /// 
    /// All execution stops until a key is pressed, then the value of that key
    /// is stored in Vx.
    fn LD_Vx_K(&mut self, x: u8) {
        // Only print this once per instruction.
        if !self.waiting_for_key {
            self.debug_println_instruction(format!("LD   V{:01x}, K", x), "Wait for a key press, store the value of the key in Vx.");
        }

        match self.keyboard.just_released() {
            Some(key) => {
                self.waiting_for_key = false;
                self.registers.set_v(x, key)
            },
            None => {
                self.waiting_for_key = true;
                // If no key is pressed, jump back to this instruction.
                self.registers.set_pc(self.registers.pc() - 2);
            }
        }
    }

    /// Fx15 - LD DT, Vx
    /// Set delay timer = Vx.
    /// 
    /// DT is set equal to the value of Vx.
    fn LD_DT_Vx(&mut self, x: u8) {
        self.debug_println_instruction(format!("LD   DT, V{:01x}", x), "Set delay timer = Vx.");

        self.timers.set_delay(self.registers.v(x));
    }

    /// Fx18 - LD ST, Vx
    /// Set sound timer = Vx.
    /// 
    /// ST is set equal to the value of Vx.
    fn LD_ST_Vx(&mut self, x: u8) {
        self.debug_println_instruction(format!("LD   ST, V{:01x}", x), "Set sound timer = Vx.");

        self.timers.set_sound(self.registers.v(x));
    }

    /// Fx1E - ADD I, Vx
    /// Set I = I + Vx.
    /// 
    /// The values of I and Vx are added, and the results are stored in I.
    fn ADD_I_Vx(&mut self, x: u8) {
        self.debug_println_instruction(format!("ADD  I, V{:01x}", x), "Set I = I + Vx.");
        self.registers.set_i(self.registers.i().wrapping_add(self.registers.v(x) as u16));
    }

    /// Fx29 - LD F, Vx
    /// Set I = location of sprite for digit Vx.
    /// 
    /// The value of I is set to the location for the hexadecimal sprite
    /// corresponding to the value of Vx. See section 2.4, Display, for more
    /// information on the Chip-8 hexadecimal font.
    fn LD_F_Vx(&mut self, x: u8) {
        self.debug_println_instruction(format!("LD   F, V{:01x}", x), "Set I = location of sprite for digit Vx.");

        self.registers.set_i(self.registers.v(x) as u16 * 5);
    }

    /// Fx33 - LD B, Vx
    /// Store BCD representation of Vx in memory locations I, I+1, and I+2.
    /// 
    /// The interpreter takes the decimal value of Vx, and places the hundreds
    /// digit in memory at location in I, the tens digit at location I+1, and
    /// the ones digit at location I+2.
    fn LD_B_Vx(&mut self, x: u8) {
        self.debug_println_instruction(format!("LD   B, V{:01x}", x), "Store BCD representation of Vx in memory locations I, I+1, and I+2.");

        let value = self.registers.v(x);
        let (hundreds, value) = (value / 100, value % 100);
        let (tens, ones) = (value / 10, value % 10);

        self.memory.write(self.registers.i().wrapping_add(0), hundreds);
        self.memory.write(self.registers.i().wrapping_add(1), tens);
        self.memory.write(self.registers.i().wrapping_add(2), ones);
    }

    /// Fx55 - LD [I], Vx
    /// Store registers V0 through Vx in memory starting at location I.
    ///
    /// The interpreter copies the values of registers V0 through Vx into
    /// memory, starting at the address in I.
    fn LD_I_Vx(&mut self, x: u8) {
        self.debug_println_instruction(format!("LD   I, V{:01x}", x), "Store registers V0 through Vx in memory starting at location I.");

        for i in 0..=x {
            self.memory.write(self.registers.i().wrapping_add(i as u16), self.registers.v(i));
        }
        // According to the chip-8-test-suite: The i register should be set to I + x + 1.
        // TODO: Some modern emulators did not do this, so some games break with this. Make it toggle-able.
        self.registers.set_i(self.registers.i().wrapping_add(x as u16 + 1));
    }

    /// Fx65 - LD Vx, [I]
    /// Read registers V0 through Vx from memory starting at location I.
    /// 
    /// The interpreter reads values from memory starting at location I into
    /// registers V0 through Vx.
    fn LD_Vx_I(&mut self, x: u8) {
        self.debug_println_instruction(format!("LD   V{:01x}, I", x), "Read registers V0 through Vx from memory starting at location I.");

        for i in 0..=x {
            self.registers.set_v(i, self.memory.read(self.registers.i().wrapping_add(i as u16)));
        }
        // According to the chip-8-test-suite: The i register should be set to I + x + 1.
        // TODO: Some modern emulators did not do this, so some games break with this. Make it toggle-able.
        self.registers.set_i(self.registers.i().wrapping_add(x as u16 + 1));
    }
}
