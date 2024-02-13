use super::tss::init_tss;
use crate::interrupt_crap::tss::TSS;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};

pub struct Selectors {
    pub code_selector: SegmentSelector,
    pub tss_selector: SegmentSelector,
}

pub static mut GDT: (GlobalDescriptorTable, Option<Selectors>) =
    (GlobalDescriptorTable::new(), None);
pub fn init_gdt() {
    init_tss();
    unsafe {
        let code_selector = GDT.0.add_entry(Descriptor::kernel_code_segment());

        #[allow(static_mut_ref)]
        let tss_selector = GDT.0.add_entry(Descriptor::tss_segment(&TSS));

        GDT.1 = Some(Selectors {
            code_selector,
            tss_selector,
        });
        GDT.0.load();
    }
}
