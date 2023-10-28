use core::fmt::{Arguments, Write};

use crate::{print, shell::SHELL};

pub static mut SCREEN: Screen = Screen::new();

struct Buffer {
    chars: [[(u8, u8); 80]; 25],
}
pub struct Screen {
    column: usize,
    row: usize,
    buffer: *mut Buffer,
}
impl Screen {
    fn print(&mut self, string: &str) {
        for &ascii in string.as_bytes() {
            self.handle_ascii(ascii);
        }
    }
    fn print_byte(&mut self, byte: u8) {
        let screen_char = unsafe {
            (*self.buffer)
                .chars
                .get_mut(self.row)
                .unwrap()
                .get_mut(self.column)
                .unwrap()
        };
        if !unsafe { SHELL.command_input } {
            screen_char.0 = byte;
            screen_char.1 = 0xF;
            self.inc_pos();
        } else {
            if self.column != 79 {
                screen_char.0 = byte;
                screen_char.1 = 0xF;
                self.inc_pos();
            }
        }
    }
    fn newline(&mut self) {
        self.column = 0;
        self.row += 1;
        if self.row >= 25 {
            let buffer = unsafe { self.buffer.as_mut().unwrap() };
            for (i, line) in buffer.chars.into_iter().enumerate() {
                if i == 0 {
                    continue;
                }
                buffer.chars[i - 1] = line;
            }
            buffer.chars[24] = [(0, 0xF); 80];
            self.row -= 1;
        }
    }
    fn backspace(&mut self) {
        self.dec_pos();
        let buffer = unsafe { self.buffer.as_mut().unwrap() };
        if unsafe { SHELL.command_input } {
            let prev_char = buffer
                .chars
                .get_mut(self.row)
                .unwrap()
                .get_mut(self.column)
                .unwrap()
                .0;
            if prev_char == 0u8 {
                self.inc_pos();
            }
        }
        buffer
            .chars
            .get_mut(self.row)
            .unwrap()
            .get_mut(self.column)
            .unwrap()
            .0 = 0;
    }
    fn dec_pos(&mut self) {
        if self.row != 0 {
            if self.column != 0 {
                self.column -= 1;
            } else {
                self.row -= 1;
                self.column = 79;
            }
        } else {
            if self.column != 0 {
                self.column -= 1;
            }
        }
    }
    fn inc_pos(&mut self) {
        self.column += 1;
        if self.column >= 80 {
            self.newline()
        }
    }
    fn handle_ascii(&mut self, ascii: u8) {
        match ascii {
            0x08 => self.backspace(),
            0x09 => self.print("    "),
            0x0a => {
                if unsafe { SHELL.command_input } {
                    self.newline();
                    unsafe { SHELL.command_input = false };
                    let buffer = unsafe { (*self.buffer).chars.get(self.row - 1).unwrap() };
                    unsafe { SHELL.run_command(buffer.get(2..79).unwrap()) }
                }
                self.newline();
                if unsafe { SHELL.command_input } {
                    print!(">{}", 0u8 as char);
                }
            }
            _ => self.print_byte(ascii),
        }
    }
    pub unsafe fn fill_screen(&self) {
        for row in 0..24 {
            for column in 0..79 {
                (*self.buffer)
                    .chars
                    .get_mut(row)
                    .unwrap()
                    .get_mut(column)
                    .unwrap()
                    .1 = 0x0F;
            }
        }
    }
    const fn new() -> Screen {
        Screen {
            column: 0,
            row: 0,
            buffer: 0xb8000 as *mut Buffer,
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
    ($($arg:tt)*) => (crate::print::_print(format_args!($($arg)*)));
}
