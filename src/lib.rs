#![cfg_attr(not(test), no_std)]
#![feature(abi_x86_interrupt)]
// #![feature(alloc)]

// #[macro_use]
// extern crate alloc;
pub mod vm;
pub mod gdt;
pub mod memory;
pub mod thread;
pub mod interrupts;
pub mod vga_buffer;
pub mod data_structures;
pub mod frame_allocator;

pub static mut physical_memory_offset: usize = 0;

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn is_power2(n:usize) -> bool {
    n & n -1 == 0
}
