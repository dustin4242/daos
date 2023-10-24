#![no_std]
#![no_main]

#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let lain = b"Let's All Love Lain";
    print(0xb8000, lain);
    loop {}
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

unsafe fn print(addr: usize, string: &[u8]) {
    let vga_buffer = addr as *mut u8;
    for (i, &byte) in string.iter().enumerate() {
        *vga_buffer.offset(i as isize * 2) = byte;
        *vga_buffer.offset(i as isize * 2 + 1) = 0x0F;
    }
}
