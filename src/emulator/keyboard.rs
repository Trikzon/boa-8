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
    keys: [bool; 16],
}

impl Keyboard {
    pub fn new() -> Self {
        Self { keys: [false; 16] }
    }
}
