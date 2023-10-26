use core::fmt::{Arguments, Write};

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
    pub fn print(&mut self, string: &str) {
        for &byte in string.as_bytes() {
            match byte {
                0xA => unsafe { self.newline() },
                _ => unsafe { self.print_byte(byte) },
            }
        }
    }
    unsafe fn print_byte(&mut self, byte: u8) {
        let screen_char = (*self.buffer)
            .chars
            .get_mut(self.row)
            .unwrap()
            .get_mut(self.column)
            .unwrap();
        screen_char.0 = byte;
        screen_char.1 = 0xF;
        self.column += 1;
        if self.column >= 80 {
            self.newline()
        }
    }
    unsafe fn newline(&mut self) {
        self.column = 0;
        self.row += 1;
        if self.row >= 25 {
            self.row = 0;
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
