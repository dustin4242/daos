#![no_std]
#![no_main]

mod print_string;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let lain = b"Let's All Love Lain";
    print_string!(lain);
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
