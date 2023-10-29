pub static mut SHELL: Shell = Shell::new();

pub struct Shell {
    pub command_input: bool,
    pub command_running: bool,
}
impl Shell {
    const fn new() -> Shell {
        Shell {
            command_input: false,
            command_running: false,
        }
    }
}

pub fn initialize_shell() {
    let font = psf_rs::Font::load(include_bytes!("./font.psfu"));
}
