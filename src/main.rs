#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod interrupt_crap;
use interrupt_crap::idt::init_idt;

mod print;
use print::print;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init_idt();

    let lain = b"Let's All Love Lain\n";
    print(lain);
    print(lain);
    print(lain);

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
