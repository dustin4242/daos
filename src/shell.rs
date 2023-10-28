use crate::interrupt_crap::idt::READ_KEYS;
use crate::print;

pub static mut SHELL: Shell = Shell::new();

pub struct Shell {
    pub command_input: bool,
    pub command_running: bool,
}

impl Shell {
    pub fn initialize_shell(&mut self) {
        print!(">{}", 0u8 as char);
        self.command_input = true;
        unsafe {
            READ_KEYS = true;
        }
    }
    pub fn run_command(&mut self, command: &[(u8, u8)]) {
        self.command_running = true;
        if command == crate::str_to_command!("lain") {
            print!("Let's All Love Lain.");
        } else {
            print!("Unknown Command.");
        }
        self.command_input = true;
        self.command_running = false;
    }
    const fn new() -> Shell {
        Shell {
            command_input: false,
            command_running: false,
        }
    }
}

#[macro_export]
macro_rules! str_to_command {
    ($x:expr) => {{
        let mut command = [(0, 0x0F); 77];
        for (i, c) in $x.chars().enumerate() {
            command[i].0 = c as u8;
        }
        command
    }};
}
