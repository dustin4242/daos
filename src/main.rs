#![no_std]
#![no_main]
#![feature(const_for)]
#![feature(const_mut_refs)]
#![feature(abi_x86_interrupt)]

mod interrupt_crap;
use interrupt_crap::idt::init_idt;

mod print;

mod pic;
use pic::init_pics;

use crate::print::SCREEN;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();

    println!("Let's All Love Lain");
    println!("Let's All Love Lain aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa");
    println!("Let's All Love Lain");
    println!("Let's All Love Lain");
    println!("Let's All Love Lain");
    println!("Let's All Love Lain");
    println!("Let's All Love Lain");
    println!("Let's All Love Lain");
    println!("Let's All Love Lain");
    println!("Let's All Love Lain");
    println!("Let's All Love Lain");
    println!("Let's All Love Lain");

    hlt_loop()
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        SCREEN.row = 0;
        SCREEN.fill_screen();
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
