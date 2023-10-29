use crate::{
    graphics, interrupt_crap::idt::READ_KEYS, print, print_graphic, println, screen::SCREEN_WIDTH,
};

pub static mut SHELL: Shell = Shell::new();

pub struct Shell {
    pub command_input: bool,
    pub command_running: bool,
}
impl Shell {
    pub fn initialize_shell(&mut self) {
        print!("> ");
        self.command_input = true;
        unsafe { READ_KEYS = true };
    }
    pub fn run_command(&mut self, command: [u8; SCREEN_WIDTH / 8 - 3]) {
        self.command_running = true;
        if command == crate::str_to_command!("lain") {
            println!("Let's All Love Lain");
        } else if command == crate::str_to_command!("cat") {
            let graphic = graphics::cat_graphic();
            print_graphic!(graphic);
            println!(":3");
        } else {
            println!("Unknown Command");
        }
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

#[macro_export]
macro_rules! str_to_command {
    ($x:expr) => {{
        let mut command = [0; SCREEN_WIDTH / 8 - 3];
        for (i, c) in $x.chars().enumerate() {
            command[i] = c as u8;
        }
        command
    }};
}
