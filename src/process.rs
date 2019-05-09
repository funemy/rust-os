use crate::context::Context;

use alloc::boxed::Box;
use alloc::vec::Vec;
// use lazy_static::lazy_static;
// use spin::Mutex;

// the tid of kernel thread is 0;
pub static mut next_pid: usize = 0;

pub struct Process {
    init: bool,
    pid: usize,
    context: Context,
}

impl Process {
    pub fn new(mut stack: Vec<u8>) -> Self {
        let stack_ptr = stack.as_mut_ptr();
        let rsp = unsafe { stack_ptr.offset(stack.len() as isize) as usize };

        // FIXME: data race here?
        unsafe { next_pid += 1 };
        let pid = unsafe { next_pid };

        let cr3 = Process::init_page_table();
        let context = Context::new(cr3, rsp, stack);
        Process {
            init: false,
            pid: pid,
            context: context,
        }
    }

    // TODO:
    // The process are created from the kernel process
    // therefore when we construct the new page table
    // we should reserve the content in the kernel page
    fn init_page_table() -> usize {
        use crate::PHYSICAL_MEMORY_OFFSET;
        use crate::memory::virt2phys;
        use x86_64::{registers::control::Cr3, structures::paging::PageTable, VirtAddr};
        // read kernel CR3
        // since we should always create a process from kernel space
        // this should be fine?
        let (level_4_table_frame, _) = Cr3::read();
        let cr3_phys = level_4_table_frame.start_address();
        let cr3_virt = unsafe { VirtAddr::new(cr3_phys.as_u64() + PHYSICAL_MEMORY_OFFSET as u64) };

        let page_table_ptr: *mut PageTable = unsafe { cr3_virt.as_mut_ptr() };
        let page_table: &mut PageTable = unsafe { &mut *page_table_ptr };

        let mut new_page_table = Box::new(PageTable::new());
        for (i, entry) in page_table.iter().enumerate() {
            if !entry.is_unused() {
                new_page_table[i] = entry.clone();
            }
        }

        let new_page_table_addr = Box::into_raw(new_page_table) as usize;
        unsafe { virt2phys(new_page_table_addr, PHYSICAL_MEMORY_OFFSET) }
    }
    //     pub fn get_tid(&self) -> u32 {
    //         self.tid
    //     }

    //     pub fn set_context(&mut self) {}

    //     // static methods
    //     pub fn get_active_thread() { }

    //     pub fn dispatch_to() {}
}