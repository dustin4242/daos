#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod interrupt;
use interrupt::init_idt;

mod print;
use print::print;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    init_idt();

    let lain = b"Let's All Love Lain\n";
    print(lain);
    print(lain);
    print(lain);

    unsafe {
        *(0xdeadbeef as *mut u8) = 42;
    };

    x86_64::instructions::interrupts::int3();

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
