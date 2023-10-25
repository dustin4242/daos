use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();
use crate::print::print;
pub fn init_idt() {
    unsafe {
        IDT.breakpoint.set_handler_fn(breakpoint_exception);
        IDT.double_fault.set_handler_fn(double_fault_handler);
        IDT.load();
    }
}
extern "x86-interrupt" fn double_fault_handler(stack: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack);
}
extern "x86-interrupt" fn breakpoint_exception(_stack: InterruptStackFrame) {
    print(b"Breakpoint Exception Occured");
}
