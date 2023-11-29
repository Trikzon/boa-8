/// Chip-8 provides 2 timers, a delay timer and a sound timer.
//
// The delay timer is active whenever the delay timer register (DT) is
// non-zero. This timer does nothing more than subtract 1 from the value of DT
// at a rate of 60Hz. When DT reaches 0, it deactivates.
//
// The sound timer is active whenever the sound timer register (ST) is
// non-zero. This timer also decrements at a rate of 60Hz, however, as long as
// ST's value is greater than zero, the Chip-8 buzzer will sound. When ST
// reaches zero, the sound timer deactivates.
//
// The sound produced by the Chip-8 interpreter has only one tone. The
// frequency of this tone is decided by the author of the interpreter.
pub struct Timers {
    delay: u8,
    sound: u8,
}

impl Timers {
    pub fn new() -> Self {
        Self { delay: 0, sound: 0 }
    }

    pub fn update(&mut self) {
        if self.delay > 0 {
            self.delay -= 1;
        }
        if self.sound > 0 {
            self.sound -= 1;
        }
    }

    pub fn delay(&self) -> u8 {
        self.delay
    }

    pub fn set_delay(&mut self, value: u8) {
        self.delay = value;
    }

    pub fn sound(&self) -> u8 {
        self.sound
    }

    pub fn set_sound(&mut self, value: u8) {
        self.sound = value;
    }
}
