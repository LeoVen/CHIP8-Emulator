use minifb::{Key, Window, WindowOptions};

/// For every pixel, there are SCALE real pixels
const SCALE: usize = 10;

/// Screen width and height without SCALE
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

pub struct Display {
    screen: Window,
    buffer: Vec<u32>,
}

impl Display {
    pub fn new() -> Self {
        Self {
            screen: Window::new(
                "CHIP-8 Emulator",
                WIDTH * SCALE,
                HEIGHT * SCALE,
                WindowOptions::default(),
            )
            .unwrap_or_else(|e| {
                panic!("Could not create the emulator screen {}", e);
            }),
            buffer: vec![0; WIDTH * SCALE * HEIGHT * SCALE],
        }
    }

    /// mem represents the start of memory indicated by the I register
    /// Displays a pixel at coordinate (x, y) with width 8 and height h
    /// Returns true if any bits where flipped
    pub fn display(&mut self, x: u16, y: u16, height: u16, mem: &[u8]) -> bool {
        let mut flipped = false;
        for w in 0..8 {
            for h in 0..height {
                let pixel = self.get_pixel(x + w, y + h);
                self.set_pixel(x + w, y + h, pixel ^ 0x00FFFFFF);
                flipped = flipped || pixel != self.get_pixel(x + w, y + h);
            }
        }
        self.screen
            .update_with_buffer(&self.buffer, WIDTH, HEIGHT)
            .unwrap();
        flipped
    }

    /// Clears the screen
    pub fn clear(&mut self) {
        for i in self.buffer.iter_mut() {
            *i = 0;
        }
    }

    pub fn is_open(&self) -> bool {
        self.screen.is_open() && !self.screen.is_key_down(Key::Escape)
    }

    /// Gets a pixel from the scaled up buffer
    fn get_pixel(&self, x: u16, y: u16) -> u32 {
        self.buffer[self.pixel_index(x, y)]
    }

    /// Sets the pixels at coord taking into account the scale
    fn set_pixel(&mut self, x: u16, y: u16, val: u32) {
        let start = self.pixel_index(x, y);
        for i in start..start + SCALE {
            for j in 0..SCALE {
                self.buffer[start + j * WIDTH] = val;
            }
        }
    }

    // Utility to get the value of a pixel from our flat buffer given SCALE
    fn pixel_index(&self, x: u16, y: u16) -> usize {
        Display::scaled(x) + Display::scaled(y) * WIDTH
    }

    fn scaled(n: u16) -> usize {
        n as usize * SCALE
    }
}
