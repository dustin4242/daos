use x86_64::{structures::tss::TaskStateSegment, VirtAddr};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub static mut TSS: TaskStateSegment = TaskStateSegment::new();
pub fn init_tss() {
    unsafe {
        TSS.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(&STACK);
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
    }
}
