use core::fmt::{Arguments, Write};

use crate::shell::SHELL;

pub static mut SCREEN: Screen = Screen::new();
pub const SCREEN_WIDTH: usize = 320;
const SCREEN_HEIGHT: usize = 192;

struct Buffer {
    chars: [[u8; SCREEN_WIDTH]; SCREEN_HEIGHT],
}
pub struct Screen {
    pub chars: [[u8; SCREEN_WIDTH / 8]; SCREEN_HEIGHT / 16],
    pub column: usize,
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
        if unsafe { SHELL.command_input } && self.column == SCREEN_WIDTH / 8 - 1 {
            return;
        }
        let font = psf_rs::Font::load(include_bytes!("./font.psfu"));
        let buffer = unsafe { self.buffer.as_mut().unwrap() };
        font.get_char(byte as char, |bit, x, y| {
            buffer.chars[self.row * 16 + y as usize][self.column * 8 + x as usize] =
                bit * self.color;
        });
        self.chars[self.row][self.column] = byte;
        self.inc_pos();
    }
    pub fn print_graphic(&mut self, bytes: [u8; 16]) {
        let buffer = unsafe { self.buffer.as_mut().unwrap() };
        for y in 0..16 {
            for x in 0..8 {
                buffer.chars[self.row * 16 + y][self.column * 8 + x] =
                    ((bytes[y] & 0x80 >> x) << x) / 0x80 * self.color;
            }
        }
        self.inc_pos();
    }
    fn newline(&mut self) {
        self.column = 0;
        self.row += 1;
        if self.row >= SCREEN_HEIGHT / 16 {
            let buffer = unsafe { self.buffer.as_mut().unwrap() };
            for y in 0..SCREEN_HEIGHT / 16 - 1 {
                self.chars[y] = self.chars[y + 1];
                for y2 in 0..16 {
                    for x in 0..SCREEN_WIDTH {
                        buffer.chars[y * 16 + y2][x] = buffer.chars[(y + 1) * 16 + y2][x];
                    }
                }
            }
            self.chars[SCREEN_HEIGHT / 16 - 1] = [0; SCREEN_WIDTH / 8];
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
        let shell = unsafe { &SHELL };
        if shell.command_input {
            if self.column != 2 {
                self.dec_pos();
                self.chars[self.row][self.column] = 0;
                for y in 0..15 {
                    for x in 0..8 {
                        buffer.chars[self.row * 16 + y][self.column * 8 + x] = 0x00;
                    }
                }
            }
        } else {
            self.dec_pos();
            self.chars[self.row][self.column] = 0;
            for y in 0..15 {
                for x in 0..8 {
                    buffer.chars[self.row * 16 + y][self.column * 8 + x] = 0x00;
                }
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
                if unsafe { SHELL.command_input } {
                    let command = self.chars[self.row - 1]
                        .get(2..SCREEN_WIDTH / 8 - 1)
                        .unwrap();
                    unsafe {
                        SHELL.command_input = false;
                        SHELL.run_command(command.try_into().unwrap());
                    }
                    crate::print!("> ");
                }
            }
            _ => self.print_byte(ascii),
        }
    }
    pub fn fill_screen(&mut self) {
        let buffer = unsafe { self.buffer.as_mut().unwrap() };
        for y in 0..SCREEN_HEIGHT - 1 {
            for x in 0..SCREEN_WIDTH - 1 {
                buffer.chars[y][x] = 0x00;
            }
        }
    }
    const fn new() -> Screen {
        Screen {
            chars: [[0; SCREEN_WIDTH / 8]; SCREEN_HEIGHT / 16],
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
#[macro_export]
macro_rules! print_graphic {
    ($arg:expr) => {
        unsafe { crate::screen::SCREEN.print_graphic($arg) }
    };
}
