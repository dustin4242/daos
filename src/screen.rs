use core::fmt::{Arguments, Write};

pub static mut SCREEN: Screen = Screen::new();
pub const SCREEN_WIDTH: usize = 320;
const SCREEN_HEIGHT: usize = 192;

struct Buffer {
    chars: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
}
pub struct Screen {
    column: usize,
    pub row: usize,
    buffer: *mut Buffer,
    color: u8,
}
impl Screen {
    fn print(&mut self, string: &str) {
        for &ascii in string.as_bytes() {
            self.handle_ascii(ascii);
        }
    }
    fn print_byte(&mut self, byte: u8) {
        let font = psf_rs::Font::load(include_bytes!("./font.psfu"));
        let buffer = unsafe { self.buffer.as_mut().unwrap() };
        font.get_char(byte as char, |bit, x, y| {
            buffer.chars[self.row * 16 + y as usize][self.column * 8 + x as usize] =
                bit * self.color;
        });
        self.inc_pos();
    }
    fn newline(&mut self) {
        self.column = 0;
        self.row += 1;
        if self.row >= SCREEN_HEIGHT / 16 {
            let buffer = unsafe { self.buffer.as_mut().unwrap() };
            for y in 0..SCREEN_HEIGHT / 16 - 1 {
                for y2 in 0..15 {
                    for x in 0..SCREEN_WIDTH - 1 {
                        buffer.chars[(y * 16 + y2) * SCREEN_WIDTH + x] =
                            buffer.chars[((y + 1) * 16 + y2) * SCREEN_WIDTH + x];
                    }
                }
            }
            for y in 0..15 {
                for x in 0..SCREEN_WIDTH {
                    buffer.chars[(SCREEN_HEIGHT - 16) + y][x] = 0x00;
                }
            }
            self.row -= 1;
        }
    }
    fn backspace(&mut self) {
        let buffer = unsafe { self.buffer.as_mut().unwrap() };
        self.dec_pos();
        for y in 0..15 {
            for x in 0..7 {
                buffer.chars[self.row * 16 + y][(self.column * 8) + x] = 0x00;
            }
        }
    }
    fn dec_pos(&mut self) {
        if self.row != 0 {
            if self.column != 0 {
                self.column -= 1;
            } else {
                self.row -= 1;
                self.column = (SCREEN_WIDTH / 8) - 1;
            }
        } else {
            if self.column != 0 {
                self.column -= 1;
            }
        }
    }
    fn inc_pos(&mut self) {
        self.column += 1;
        if self.column >= SCREEN_WIDTH / 8 {
            self.newline()
        }
    }
    fn handle_ascii(&mut self, ascii: u8) {
        match ascii {
            0x08 => self.backspace(),
            0x09 => self.print("    "),
            0x0a => {
                self.newline();
            }
            _ => self.print_byte(ascii),
        }
    }
    pub fn _fill_screen(&mut self) {
        let buffer = unsafe { self.buffer.as_mut().unwrap() };
        for y in 0..SCREEN_HEIGHT - 1 {
            for x in 0..SCREEN_WIDTH - 1 {
                buffer.chars[y][x] = 0x00;
            }
        }
    }
    const fn new() -> Screen {
        Screen {
            column: 0,
            row: 0,
            buffer: 0xa0000 as *mut Buffer,
            color: 0b00001111,
        }
    }
}
impl core::fmt::Write for Screen {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.print(s);
        Ok(())
    }
}

pub fn _print(args: Arguments) {
    unsafe {
        SCREEN.write_fmt(args).unwrap();
    }
}

#[macro_export]
macro_rules! println {
    () => (print!("\n"));
    ($($arg:tt)*) => (crate::print!("{}\n", format_args!($($arg)*)));
}
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (crate::screen::_print(format_args!($($arg)*)));
}
