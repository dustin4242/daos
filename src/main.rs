#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let hello = b"Let's All Love Lain";
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in hello.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
