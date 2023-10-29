use crate::{
    interrupt_crap::idt::READ_KEYS,
    print, println,
    screen::{SCREEN, SCREEN_WIDTH},
};

pub static mut SHELL: Shell = Shell::new();

pub struct Shell {
    pub command_input: bool,
    pub command_running: bool,
}
impl Shell {
    pub fn run_command(&mut self) {
        let screen = unsafe { &SCREEN };
        self.command_input = false;
        self.command_running = true;
        let command = screen.chars[screen.row - 1]
            .get(2..SCREEN_WIDTH / 16 - 1)
            .unwrap();
        if command == crate::str_to_command!("lain") {
            println!("Let's All Love Lain");
        }
        print!("> ");
        self.command_running = false;
        self.command_input = true;
    }
    const fn new() -> Shell {
        Shell {
            command_input: false,
            command_running: false,
        }
    }
}
pub fn initialize_shell() {
    unsafe { SHELL.command_input = true };
    print!("> ");
    unsafe { READ_KEYS = true };
}

#[macro_export]
macro_rules! str_to_command {
    ($x:expr) => {{
        let mut command = [0; SCREEN_WIDTH / 16 - 3];
        for (i, c) in $x.chars().enumerate() {
            command[i] = c as u8;
        }
        command
    }};
}
