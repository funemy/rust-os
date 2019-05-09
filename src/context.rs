use alloc::vec::Vec;
use core::mem;

#[derive(Debug, Clone)]
// NOTE: only store non-volatile registers
// `fx` and `fx_loc` are in the redox code, but I don't know what they will affect
pub struct Context {
    // fx: usize,
    cr3: usize,
    rflags: usize,
    rbx: usize,
    r12: usize,
    r13: usize,
    r14: usize,
    r15: usize,
    rbp: usize,
    // stack pointer
    rsp: usize,
    // stack content
    stack: Vec<u8>,
    // fx_loc: Vec<u8>,
}

impl Default for Context {
    fn default() -> Self {
        Context {
            stack: vec![0_u8; 4096],
            ..Default::default()
        }
    }
}

impl Context {
    pub fn new(cr3: usize, rsp: usize, stack: Vec<u8>) -> Self {
        Context {
            cr3: cr3,
            rflags: 0,
            rbx: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rbp: rsp,
            rsp: rsp,
            stack: stack,
        }
    }

    pub fn set_stack(&mut self, addr: usize) {
        self.rsp = addr;
    }

    pub unsafe fn push_stack(&mut self, value: usize) {
        self.rsp -= mem::size_of::<usize>();
        *(self.rsp as *mut usize) = value;
    }

    pub unsafe fn pop_stack(&mut self) -> usize {
        let value = *(self.rsp as *const usize);
        self.rsp += mem::size_of::<usize>();
        value
    }

    pub fn set_cr3(&mut self, addr: usize) {
        self.cr3 = addr;
    }

    pub fn get_cr3(&self) -> usize {
        self.cr3
    }

    // NOTE: this is basically what `save_register` and `restore_register` did in `threads_low.asm`
    #[cold]
    #[inline(never)]
    #[naked]
    pub unsafe fn switch_to(&mut self, next: &mut Context) {
        asm!("mov $0, cr3" : "=r"(self.cr3) : : "memory" : "intel", "volatile");
        if next.cr3 != self.cr3 {
            asm!("mov cr3, $0" : : "r"(next.cr3) : "memory" : "intel", "volatile");
        }
        asm!("pushfq ; pop $0" : "=r"(self.rflags) : : "memory" : "intel", "volatile");
        asm!("push $0 ; popfq" : : "r"(next.rflags) : "memory" : "intel", "volatile");

        asm!("mov $0, rbx" : "=r"(self.rbx) : : "memory" : "intel", "volatile");
        asm!("mov rbx, $0" : : "r"(next.rbx) : "memory" : "intel", "volatile");

        asm!("mov $0, r12" : "=r"(self.r12) : : "memory" : "intel", "volatile");
        asm!("mov r12, $0" : : "r"(next.r12) : "memory" : "intel", "volatile");

        asm!("mov $0, r13" : "=r"(self.r13) : : "memory" : "intel", "volatile");
        asm!("mov r13, $0" : : "r"(next.r13) : "memory" : "intel", "volatile");

        asm!("mov $0, r14" : "=r"(self.r14) : : "memory" : "intel", "volatile");
        asm!("mov r14, $0" : : "r"(next.r14) : "memory" : "intel", "volatile");

        asm!("mov $0, r15" : "=r"(self.r15) : : "memory" : "intel", "volatile");
        asm!("mov r15, $0" : : "r"(next.r15) : "memory" : "intel", "volatile");

        asm!("mov $0, rsp" : "=r"(self.rsp) : : "memory" : "intel", "volatile");
        asm!("mov rsp, $0" : : "r"(next.rsp) : "memory" : "intel", "volatile");

        asm!("mov $0, rbp" : "=r"(self.rbp) : : "memory" : "intel", "volatile");
        asm!("mov rbp, $0" : : "r"(next.rbp) : "memory" : "intel", "volatile");
    }

    // static function
    // save the current context into a context object
    // use for kernel process
    pub fn save_current_context() -> Self {
        let mut context: Context = Default::default();
        unsafe {
            asm!("mov $0, cr3" : "=r"(context.cr3) : : "memory" : "intel", "volatile");
            asm!("pushfq ; pop $0 " : "=r"(context.rflags) : : "memory" : "intel", "volatile");
            asm!("mov $0, rbx" : "=r"(context.rbx) : : "memory" : "intel", "volatile");
            asm!("mov $0, r12" : "=r"(context.r12) : : "memory" : "intel", "volatile");
            asm!("mov $0, r13" : "=r"(context.r13) : : "memory" : "intel", "volatile");
            asm!("mov $0, r14" : "=r"(context.r14) : : "memory" : "intel", "volatile");
            asm!("mov $0, r15" : "=r"(context.r15) : : "memory" : "intel", "volatile");
            asm!("mov $0, rsp" : "=r"(context.rsp) : : "memory" : "intel", "volatile");
            asm!("mov $0, rbp" : "=r"(context.rbp) : : "memory" : "intel", "volatile");
        }
        context
    }
}
