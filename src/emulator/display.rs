const WIDTH: usize = 64;
const HEIGHT: usize = 32;

/// The original implementation of the Chip-8 language used a 64x32-pixel
/// monochrome display with this format:
///
/// -------------------
/// |(0,0)	    (63,0)|
/// |                 |
/// |(0,31)	   (63,31)|
/// -------------------
///
/// Chip-8 draws graphics on screen through the use of sprites. A sprite is a
/// group of bytes which are a binary representation of the desired picture.
/// Chip-8 sprites may be up to 15 bytes, for a possible sprite size of 8x15.
pub struct Display {
    pixels: [u32; HEIGHT * WIDTH / 32],
    scale: u32,
    background_color: (f32, f32, f32),
    foreground_color: (f32, f32, f32),
}

impl Display {
    pub fn new(
        scale: u32,
        background_color: (f32, f32, f32),
        foreground_color: (f32, f32, f32),
    ) -> Self {
        Self {
            pixels: [0; HEIGHT * WIDTH / 32],
            scale,
            background_color,
            foreground_color,
        }
    }

    pub fn clear(&mut self) {
        self.pixels = [0; HEIGHT * WIDTH / 32];
    }

    /// Draws a list of bytes onto the screen. Each byte being one row.
    /// Returns true if drawing collides with already drawn pixel.
    pub fn draw_sprite(&mut self, mut x: u8, mut y: u8, sprite: &[u8]) -> bool {
        let mut collided = false;

        let origin_x = x;
        let origin_y = y;
        for byte in sprite {
            for i in (0..8).rev() {
                let bit = (byte >> i) & 0b00000001;
                if bit == 1 {
                    if self.draw_pixel(x, y) {
                        collided = true;
                    }
                }
                x += 1;

                // If the sprite starts at the right and gets cut off, break.
                if origin_x < WIDTH as u8 && x > WIDTH as u8 {
                    break;
                }
            }
            y += 1;
            x = origin_x;

            // If the sprite starts at the bottom and gets cut off, break.
            if origin_y < HEIGHT as u8 && y > HEIGHT as u8 {
                break;
            }
        }
        collided
    }

    /// Draws a pixel onto the screen.
    /// Returns true if drawing collides with already drawn pixel.
    pub fn draw_pixel(&mut self, mut x: u8, mut y: u8) -> bool {
        // Loop x and y if they go out of the display's bounds.
        while x >= WIDTH as u8 {
            x -= WIDTH as u8;
        }
        while y >= HEIGHT as u8 {
            y -= HEIGHT as u8;
        }

        self.pixels[x as usize] ^= 0b10000000000000000000000000000000 >> y;

        // If the pixel is off, then it collided and this returns true.
        (self.pixels[x as usize] << y) >> HEIGHT - 1 == 0
    }

    pub fn pixels(&self) -> [u32; HEIGHT * WIDTH / 32] {
        self.pixels
    }

    pub fn scale(&self) -> u32 {
        self.scale
    }

    pub fn background_color(&self) -> (f32, f32, f32) {
        self.background_color
    }

    pub fn foreground_color(&self) -> (f32, f32, f32) {
        self.foreground_color
    }
}
