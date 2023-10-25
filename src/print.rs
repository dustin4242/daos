static mut COUNTER: isize = 0;

pub fn print(string: &[u8]) {
    for &byte in string {
        match byte {
            0xA => unsafe { newline() },
            _ => unsafe { print_byte(byte) },
        }
    }
}
unsafe fn print_byte(byte: u8) {
    let vga_buffer = 0xb8000 as *mut u8;
    *vga_buffer.offset(COUNTER * 2) = byte;
    *vga_buffer.offset(COUNTER * 2 + 1) = 0x0F;
    COUNTER += 1;
    if COUNTER >= 80 * 20 {
        COUNTER = 0;
    }
}
unsafe fn newline() {
    COUNTER += 80 - COUNTER % 80;
    if COUNTER >= 80 * 20 {
        COUNTER = 0;
    }
}
