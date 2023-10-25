use crate::print::print;
use x86_64::instructions::tables::load_tss;
use x86_64::registers::segmentation::{Segment, CS};
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

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
        IDT.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        IDT.load();
    }
}

extern "x86-interrupt" fn double_fault_handler(_stack: InterruptStackFrame, _error_code: u64) -> ! {
    print(b"Double Fault Has Occured");
    loop {}
}
extern "x86-interrupt" fn breakpoint_exception(_stack: InterruptStackFrame) {
    print(b"Breakpoint Exception Occured");
}
