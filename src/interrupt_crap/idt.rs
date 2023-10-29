use x86_64::instructions::{port::Port, tables::load_tss};
use x86_64::registers::segmentation::{Segment, CS as CodeSegment};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::pic::{InterruptIndex, PICS};
use crate::print;

use super::gdt::init_gdt;
use super::gdt::GDT;
use super::scancodes;

pub static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();
pub fn init_idt() {
    unsafe {
        init_gdt();
        load_tss(GDT.1.as_ref().unwrap().tss_selector);
        CodeSegment::set_reg(GDT.1.as_ref().unwrap().code_selector);

        IDT.breakpoint.set_handler_fn(breakpoint_handler);
        IDT[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_handler);
        IDT[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_handler);
        IDT.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(0);
        IDT.load();
    }
}

extern "x86-interrupt" fn breakpoint_handler(stack: InterruptStackFrame) {
    panic!("Breakpoint Exception Occured:\n{:#?}", stack);
}
extern "x86-interrupt" fn timer_handler(_stack: InterruptStackFrame) {
    unsafe {
        PICS.notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}
pub static mut READ_KEYS: bool = false;
extern "x86-interrupt" fn keyboard_handler(_stack: InterruptStackFrame) {
    if unsafe { READ_KEYS } {
        let mut port = Port::new(0x60);
        let scancode: u8 = unsafe { port.read() };
        let key = scancodes::get_char(scancode);
        if let Some(key) = key {
            print!("{}", key);
        }
    }
    unsafe {
        PICS.notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
extern "x86-interrupt" fn double_fault_handler(stack: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("Double Fault Exception Occurred:\n{:#?}", stack);
}
