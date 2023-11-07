#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod interrupt_crap;
use interrupt_crap::idt::init_idt;

use daos_lib::print;
use daos_lib::println;
use daos_lib::screen::{self, SCREEN};
use daos_lib::shell::SHELL;

mod pic;
use pic::init_pics;
use x86_64::instructions::port::Port;
use x86_64::instructions::port::PortGeneric;
use x86_64::instructions::port::ReadWriteAccess;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();

    unsafe { SCREEN.font = Some(psf_rs::Font::load(include_bytes!("./font.psfu"))) };

    let mut misc1: PortGeneric<u8, ReadWriteAccess> = Port::new(0x3C4);
    let mut misc2: PortGeneric<u8, ReadWriteAccess> = Port::new(0x3C5);

    unsafe {
        println!("{}", misc2.read());
    };

    println!("Welcome To Dustin's Awesome Operating\nSystem!");
    println!("Talwat Is The Goat For The Font Loader!");
    unsafe { SHELL.initialize_shell() };

    hlt_loop()
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        SHELL.read_keys = false;
        SHELL.command_input = false;
        SHELL.command_running = false;
        SCREEN.clear_screen();
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
