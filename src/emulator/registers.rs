/// Chip-8 has 16 general purpose 8-bit registers, usually referred to as Vx,
/// where x is a hexadecimal digit (0 through F). There is also a 16-bit
/// register called I. This register is generally used to store memory
/// addresses, so only the lowest (rightmost) 12 bits are usually used.
///
/// The VF register should not be used by any program, as it is used as a flag
/// by some instructions. See section 3.0, Instructions for details.
///
/// There are also some "pseudo-registers" which are not accessable from Chip-8
/// programs. The program counter (PC) should be 16-bit, and is used to store
/// the currently executing address. The stack pointer (SP) can be 8-bit, it is
/// used to point to the topmost level of the stack.
pub struct Registers {
    v: [u8; 16],
    i: u16,
    pc: u16, // program counter
    sp: u8,  // stack pointer
    stack: [u16; 16],
}

impl Registers {
    pub fn new() -> Self {
        Self {
            v: [0; 16],
            i: 0,
            pc: 0,
            sp: 0,
            stack: [0; 16],
        }
    }

    pub fn v(&self, x: u8) -> u8 {
        self.v[x as usize]
    }

    pub fn set_v(&mut self, x: u8, value: u8) {
        self.v[x as usize] = value;
    }

    pub fn vf(&self) -> u8 {
        self.v(0x0F)
    }

    pub fn set_vf(&mut self, value: u8) {
        self.set_v(0x0F, value);
    }

    pub fn i(&self) -> u16 {
        self.i
    }

    pub fn set_i(&mut self, value: u16) {
        self.i = value;
    }

    pub fn pc(&self) -> u16 {
        self.pc
    }

    pub fn increment_pc(&mut self) {
        self.pc += 2;
    }

    // TODO: Implement the stack.
}
