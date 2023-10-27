#![no_std]
#![no_main]
#![feature(const_for)]
#![feature(const_mut_refs)]
#![feature(abi_x86_interrupt)]

mod interrupt_crap;
use interrupt_crap::idt::init_idt;

mod cursor;
mod print;

mod pic;
use pic::init_pics;

use crate::{cursor::CURSOR, print::SCREEN, shell::SHELL};

mod shell;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();

    println!("Welcome To Dustin's Awesome Operating System!");
    unsafe { SHELL.initialize_shell() };

    hlt_loop()
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
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
    unsafe { CURSOR.disable_cursor() };
    unsafe { SCREEN.fill_screen() };
}
