use crate::context::Context;

pub struct Thread {
    init: bool,
    tid: u32,
    content: Context,
}

// impl Thread {
//     pub fn new() -> Self { }


//     pub fn init_page_table(&self) { }

//     pub fn get_tid(&self) -> u32 {
//         self.tid
//     }

//     pub fn set_context(&mut self) {}

//     // static methods
//     pub fn get_active_thread() { }

//     pub fn dispatch_to() {}

// }