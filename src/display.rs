use piston_window::*;

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
    fn is_open(&mut self) -> bool;
    /// Updates the screen
    fn update(&mut self);
    /// Set if the display should keep updating
    fn should_update(&mut self, update: bool);
    /// Gets a pixel from the scaled up buffer
    fn get_pixel(&self, x: u16, y: u16) -> u8;
    /// Sets the pixels at coord taking into account the scale
    fn set_pixel(&mut self, x: u16, y: u16, val: u8);
}

/// A display using the piston library
pub struct Chip8Display {
    screen: PistonWindow,
    buffer: [[u8; WIDTH]; HEIGHT],
    event: Option<Event>,
    on: bool,
}

impl Chip8Display {
    pub fn new() -> Self {
        Self {
            screen: WindowSettings::new(
                "CHIP-8 Emulator",
                Size::from(((WIDTH * SCALE) as u32, (HEIGHT * SCALE) as u32)),
            )
            .resizable(false)
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| {
                panic!("Could not create the emulator screen {}", e);
            }),
            buffer: [[0; WIDTH]; HEIGHT],
            event: None,
            on: true,
        }
    }
}

impl Display for Chip8Display {
    fn display(&mut self, x: u16, y: u16, height: u16, mem: &[u8]) -> bool {
        let mut flipped = false;
        for cy in 0..height {
            let y_line = mem[cy as usize];
            for cx in 0..8 {
                let pixel = y_line & (0x80 >> cx);
                if pixel != 0 {
                    if self.buffer[x as usize][y as usize] == 1 {
                        flipped = true
                    }
                    self.buffer[x as usize][y as usize] ^= 1;
                }
            }
        }
        flipped
    }

    fn clear(&mut self) {
        for row in self.buffer.iter_mut() {
            for col in row.iter_mut() {
                *col = 0;
            }
        }
    }

    fn is_open(&mut self) -> bool {
        self.event = self.screen.next();
        self.event.is_some()
    }

    fn update(&mut self) {
        if self.on {
            // todo
        }
    }

    fn should_update(&mut self, update: bool) {
        self.on = update;
    }

    fn get_pixel(&self, x: u16, y: u16) -> u8 {
        self.buffer[x as usize][y as usize]
    }

    fn set_pixel(&mut self, x: u16, y: u16, val: u8) {
        self.buffer[x as usize][y as usize] = val;
    }
}

impl Default for Chip8Display {
    fn default() -> Self {
        Self::new()
    }
}

/// A simple display, used for tests and debugging
pub struct TextDisplay {
    buffer: [[u8; WIDTH]; HEIGHT],
    on: bool,
}

impl TextDisplay {
    pub fn new() -> Self {
        Self {
            buffer: [[0; WIDTH]; HEIGHT],
            on: true,
        }
    }

    #[allow(dead_code)]
    pub fn debug_string(&self) -> [char; WIDTH * HEIGHT] {
        let mut result: [char; WIDTH * HEIGHT] = [' '; WIDTH * HEIGHT];
        for i in 0..WIDTH {
            for j in 0..HEIGHT {
                result[i + j * WIDTH] = match self.get_pixel(i as u16, j as u16) {
                    0x0 => ' ',
                    _ => 'X',
                }
            }
        }
        result
    }
}

impl Display for TextDisplay {
    fn display(&mut self, x: u16, y: u16, height: u16, mem: &[u8]) -> bool {
        let mut flipped = false;
        for cy in 0..height {
            let y_line = mem[cy as usize];
            for cx in 0..8 {
                let pixel = y_line & (0x80 >> cx);
                if pixel != 0 {
                    if self.buffer[x as usize][y as usize] == 1 {
                        flipped = true
                    }
                    self.buffer[x as usize][y as usize] ^= 1;
                }
            }
        }
        flipped
    }

    fn clear(&mut self) {
        for row in self.buffer.iter_mut() {
            for col in row.iter_mut() {
                *col = 0;
            }
        }
    }

    fn is_open(&mut self) -> bool {
        true
    }

    fn update(&mut self) {
        // todo fix
        if self.on {
            // Position the cursor at row 1, col 1
            print!("\x1B[2J");
            for row in self.buffer.iter() {
                for col in row.iter() {
                    match col {
                        0x0 => print!("."),
                        _ => print!("#"),
                    }
                }
                println!("");
            }
        }
    }

    fn should_update(&mut self, update: bool) {
        self.on = update;
    }

    fn get_pixel(&self, x: u16, y: u16) -> u8 {
        self.buffer[x as usize][y as usize]
    }

    fn set_pixel(&mut self, x: u16, y: u16, val: u8) {
        self.buffer[x as usize][y as usize] = val;
    }
}

impl Default for TextDisplay {
    fn default() -> Self {
        Self::new()
    }
}
