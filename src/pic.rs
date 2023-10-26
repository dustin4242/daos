//PIC stands for Peripheral Interface Controller
//The more you know
use pic8259::ChainedPics;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static mut PICS: ChainedPics = unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) };

pub fn init_pics() {
    unsafe { PICS.initialize() };
}

pub enum InterruptIndex {
    Timer = PIC_1_OFFSET as isize,
    Keyboard,
}
impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}
