// #[derive(Debug, Clone, Default)]
// pub struct Context {
//     fx: usize,
//     cr3: usize,
//     rflags: usize,
//     rbx: usize,
//     r12: usize,
//     r13: usize,
//     r14: usize,
//     r15: usize,
//     rbp: usize,
//     rsp: usize,
//     stack: Vec<u8>,
//     fx_loc: Vec<u8>,
// }


// impl Context {
//     pub fn new() -> Self {

//     }

//     pub fn push_stack(&mut self) {

//     }

//     pub fn pop_stack(&mut self) {

//     }

//     pub fn write_CR3(&self) {

//     }

//     pub fn read_CR3(&self) {

//     }
// }

// pub struct Thread {
//     init: bool,
//     tid: u32,
//     content: Context,
// }

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