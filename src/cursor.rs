use x86_64::instructions::port::{Port, PortGeneric, ReadWriteAccess};
pub static mut CURSOR: Cursor = Cursor::new();
pub struct Cursor {
    ports: [PortGeneric<u8, ReadWriteAccess>; 2],
}
impl Cursor {
    pub fn disable_cursor(&mut self) {
        unsafe {
            self.ports[0].write(0x0Au8);
            self.ports[1].write(0x20);
        };
    }
    const fn new() -> Cursor {
        Cursor {
            ports: [Port::new(0x3D4), Port::new(0x3D5)],
        }
    }
}
