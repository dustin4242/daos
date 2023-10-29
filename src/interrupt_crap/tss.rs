use x86_64::{structures::tss::TaskStateSegment, VirtAddr as Address};

pub static mut TSS: TaskStateSegment = TaskStateSegment::new();
pub fn init_tss() {
    unsafe {
        TSS.interrupt_stack_table[0] = {
            const STACK_SIZE: usize = 1024 * 20;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = Address::from_ptr(&STACK);
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
    }
}
