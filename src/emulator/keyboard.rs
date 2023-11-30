use glutin::event::{ElementState, KeyboardInput, VirtualKeyCode};

/// The computers which originally used the Chip-8 Language had a 16-key
/// hexadecimal keypad with the following layout:
///
/// |1|2|3|C|
/// |4|5|6|D|
/// |7|8|9|E|
/// |A|0|B|F|
///
/// This layout must be mapped into various other configurations to fit the
/// keyboards of today's platforms.
pub struct Keyboard {
    keys: [bool; 16]
}

impl Keyboard {
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }

    pub fn process_input(&mut self, input: KeyboardInput) {
        if let Some(keycode) = input.virtual_keycode {
            let key: Option<u8> = match keycode {
                VirtualKeyCode::X => Some(0x0),
                VirtualKeyCode::Key1 => Some(0x1),
                VirtualKeyCode::Key2 => Some(0x2),
                VirtualKeyCode::Key3 => Some(0x3),
                VirtualKeyCode::Q => Some(0x4),
                VirtualKeyCode::W => Some(0x5),
                VirtualKeyCode::E => Some(0x6),
                VirtualKeyCode::A => Some(0x7),
                VirtualKeyCode::S => Some(0x8),
                VirtualKeyCode::D => Some(0x9),
                VirtualKeyCode::Z => Some(0xA),
                VirtualKeyCode::C => Some(0xB),
                VirtualKeyCode::Key4 => Some(0xC),
                VirtualKeyCode::R => Some(0xD),
                VirtualKeyCode::F => Some(0xE),
                VirtualKeyCode::V => Some(0xF),
                _ => None
            };
            
            if let Some(key) = key {
                self.keys[key as usize] = input.state == ElementState::Pressed;
            }
        }
    }

    pub fn is_pressed(&self, key: u8) -> bool {
        if key > 0xF {
            println!("Attempted to query key out of range: {:#04x}", key);
            return false;
        }

        self.keys[key as usize]
    }

    pub fn find_pressed_key(&self) -> Option<u8> {
        for key in 0..16 {
            if self.is_pressed(key) {
                return Some(key);
            }
        }
        None
    }
}
