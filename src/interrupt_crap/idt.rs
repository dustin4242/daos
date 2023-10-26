use x86_64::instructions::tables::load_tss;
use x86_64::registers::segmentation::{Segment, CS};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::pic::{InterruptIndex, PICS};
use crate::println;

use super::gdt::init_gdt;
use super::gdt::GDT;
use super::tss::DOUBLE_FAULT_IST_INDEX;

pub static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();
pub fn init_idt() {
    unsafe {
        init_gdt();
        CS::set_reg(GDT.1.as_ref().unwrap().code_selector);
        load_tss(GDT.1.as_ref().unwrap().tss_selector);

        IDT.breakpoint.set_handler_fn(breakpoint_exception);
        IDT[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_handler);
        IDT[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_handler);
        IDT.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        IDT.load();
    }
}

extern "x86-interrupt" fn breakpoint_exception(stack: InterruptStackFrame) {
    panic!("Breakpoint Exception Occured:\n{:#?}", stack);
}
extern "x86-interrupt" fn timer_handler(_stack: InterruptStackFrame) {
    unsafe {
        PICS.notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}
extern "x86-interrupt" fn keyboard_handler(_stack: InterruptStackFrame) {
    println!("k");
    unsafe {
        PICS.notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
extern "x86-interrupt" fn double_fault_handler(stack: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack);
}
