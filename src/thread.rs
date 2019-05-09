use crate::context::Context;
use alloc::vec::Vec;
use lazy_static::lazy_static;
use spin::Mutex;

// the tid of kernel thread is 0;
pub static mut next_tid: usize = 0;

pub struct Thread {
    init: bool,
    tid: usize,
    context: Context,
}

impl Thread {
    pub fn new(mut stack: Vec<u8>) -> Self {
        let stack_ptr = stack.as_mut_ptr();
        let rsp = unsafe { stack_ptr.offset(stack.len() as isize) as usize };
        unsafe { next_tid += 1 };
        let tid = unsafe { next_tid };
        let cr3 = Thread::init_page_table();
        let context = Context::new(cr3, rsp, stack);
        Thread {
            init: false,
            // FIXME: data race here
            tid: tid,
            context: context,
        }
    }

    // TODO:
    fn init_page_table() -> usize {
        0 as usize
    }
    //     pub fn get_tid(&self) -> u32 {
    //         self.tid
    //     }

    //     pub fn set_context(&mut self) {}

    //     // static methods
    //     pub fn get_active_thread() { }

    //     pub fn dispatch_to() {}
}