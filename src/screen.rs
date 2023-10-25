pub struct Screen {
    pos: isize,
}
impl Screen {
    pub unsafe fn print(&mut self, string: &[u8]) {
        let vga_buffer = 0xb8000 as *mut u8;
        for &byte in string {
            match byte {
                0xA => self.pos += 80 - self.pos % 80,
                _ => {
                    *vga_buffer.offset(self.pos * 2) = byte;
                    *vga_buffer.offset(self.pos * 2 + 1) = 0x0F;
                    self.pos += 1;
                }
            }
        }
    }
    pub fn new() -> Screen {
        Screen { pos: 0 }
    }
}
