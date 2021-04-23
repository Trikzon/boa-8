/// The Chip-8 language is capable of accessing up to 4KB (4,096 bytes) of RAM,
/// from location 0x000 (0) to 0xFFF (4095). The first 512 bytes, from 0x000
/// to 0x1FF, are where the original interpreter was located, and should not be
/// used by programs.
///
/// Memory Map:
/// +---------------+= 0xFFF (4095) End of Chip-8 RAM
/// |               |
/// |               |
/// |               |
/// |               |
/// |               |
/// | 0x200 to 0xFFF|
/// |     Chip-8    |
/// | Program / Data|
/// |     Space     |
/// |               |
/// |               |
/// |               |
/// |               |
/// |               |
/// |               |
/// +---------------+= 0x200 (512) Start of most Chip-8 programs
/// | 0x000 to 0x1FF|
/// | Reserved for  |
/// |  interpreter  |
/// +---------------+= 0x000 (0) Start of Chip-8 RAM
pub struct Memory {
    ram: [u8; 4096],
}

impl Memory {
    pub fn new() -> Self {
        let mut memory = Self { ram: [0; 4096] };
        crate::emulator::sprites::load_default_sprites(&mut memory);
        memory
    }

    pub fn read(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value;
    }

    pub fn load_rom(&mut self, rom_data: &[u8]) {
        todo!()
    }
}
