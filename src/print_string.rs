#[macro_export]
macro_rules! print_string {
    ($a: expr, $b: expr) => {
        print_string::print($a, $b)
    };
    ($a: expr) => {
        print_string::print($a, None)
    };
}

pub unsafe fn print(string: &[u8], addr: Option<usize>) {
    let vga_buffer = match addr {
        Some(a) => a as *mut u8,
        None => 0xb8000 as *mut u8,
    };
    for (i, &byte) in string.iter().enumerate() {
        *vga_buffer.offset(i as isize * 2) = byte;
        *vga_buffer.offset(i as isize * 2 + 1) = 0x0F;
    }
}
