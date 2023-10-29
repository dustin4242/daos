#![no_std]
#![no_main]
#![feature(const_for)]
#![feature(const_mut_refs)]
#![feature(abi_x86_interrupt)]

mod interrupt_crap;
use interrupt_crap::idt::init_idt;

mod screen;

mod shell;

mod pic;
use pic::init_pics;

use crate::{
    interrupt_crap::idt::READ_KEYS,
    screen::SCREEN,
    shell::{initialize_shell, SHELL},
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();

    println!("Welcome To Dustin's Awesome Operating\nSystem!");
    println!("Talwat Is The Goat For The Font Loader!");
    initialize_shell();

    hlt_loop()
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        READ_KEYS = false;
        SHELL.command_input = false;
        SHELL.command_running = false;
        SCREEN.fill_screen();
        SCREEN.row = 0
    };
    print!("{}", info);
    hlt_loop()
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

fn init() {
    init_idt();
    init_pics();
    x86_64::instructions::interrupts::enable();
}
