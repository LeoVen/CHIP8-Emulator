use minifb::{Key, Window, WindowOptions};

/// For every pixel, there are SCALE real pixels
const SCALE: usize = 10;

/// Screen width and height without SCALE
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

/// A Display for the CHIP-8
pub trait Display {
    /// Displays a pixel at (x, y) with size (8, height)
    /// mem represents the start of memory indicated by the I register
    /// Returns true if any bits where flipped
    fn display(&mut self, x: u16, y: u16, height: u16, mem: &[u8]) -> bool;
    /// Clears the display
    fn clear(&mut self);
    /// If the display is still open
    fn is_open(&self) -> bool;
    /// Gets a pixel from the scaled up buffer
    fn get_pixel(&self, x: u16, y: u16) -> u32;
    /// Sets the pixels at coord taking into account the scale
    fn set_pixel(&mut self, x: u16, y: u16, val: u32);
}

/// A display using the minifb library
pub struct Chip8Display {
    screen: Window,
    buffer: Vec<u32>,
}

impl Chip8Display {
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

    // Utility to get the value of a pixel from our flat buffer given SCALE
    fn pixel_index(&self, x: u16, y: u16) -> usize {
        Chip8Display::scaled(x) + Chip8Display::scaled(y) * WIDTH
    }

    fn scaled(n: u16) -> usize {
        n as usize * SCALE
    }
}

impl Display for Chip8Display {
    fn display(&mut self, x: u16, y: u16, height: u16, mem: &[u8]) -> bool {
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

    fn clear(&mut self) {
        for i in self.buffer.iter_mut() {
            *i = 0;
        }
    }

    fn is_open(&self) -> bool {
        self.screen.is_open() && !self.screen.is_key_down(Key::Escape)
    }

    fn get_pixel(&self, x: u16, y: u16) -> u32 {
        self.buffer[self.pixel_index(x, y)]
    }

    fn set_pixel(&mut self, x: u16, y: u16, val: u32) {
        let start = self.pixel_index(x, y);
        for i in start..start + SCALE {
            for j in 0..SCALE {
                self.buffer[start + j * WIDTH] = val;
            }
        }
    }
}

impl Default for Chip8Display {
    fn default() -> Self {
        Self::new()
    }
}

/// A simple display, used for tests and debugging
pub struct DebugDisplay {
    buffer: [u32; WIDTH * HEIGHT],
}

impl DebugDisplay {
    pub fn new() -> Self {
        Self {
            buffer: [0; WIDTH * HEIGHT],
        }
    }

    #[allow(dead_code)]
    pub fn debug_string(&self) -> [char; WIDTH * HEIGHT] {
        let mut result: [char; WIDTH * HEIGHT] = [' '; WIDTH * HEIGHT];
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                result[i + j * WIDTH] = match self.get_pixel(i as u16, j as u16)
                {
                    0x0 => ' ',
                    _ => 'X',
                }
            }
        }
        result
    }
}

impl Display for DebugDisplay {
    fn display(&mut self, x: u16, y: u16, height: u16, mem: &[u8]) -> bool {
        let mut flipped = false;
        for w in 0..8 as u16 {
            let pixel = mem[w as usize] as u32;
            for h in 0..height {
                self.set_pixel(x + w, y + h, pixel ^ 1);
                flipped = flipped || pixel != self.get_pixel(x + w, y + h);
            }
        }
        flipped
    }

    fn clear(&mut self) {
        for i in self.buffer.iter_mut() {
            *i = 0;
        }
    }

    fn is_open(&self) -> bool {
        true
    }

    fn get_pixel(&self, x: u16, y: u16) -> u32 {
        self.buffer[x as usize + y as usize * WIDTH]
    }

    fn set_pixel(&mut self, x: u16, y: u16, val: u32) {
        self.buffer[x as usize + y as usize * WIDTH] = val;
    }
}

impl Default for DebugDisplay {
    fn default() -> Self {
        Self::new()
    }
}
