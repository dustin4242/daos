#![no_std]
#![no_main]

mod screen;
use screen::Screen;

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let mut screen = Screen::new();
    let lain = b"Let's All Love Lain\n";
    screen.print(lain);
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
