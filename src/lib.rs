#![cfg_attr(not(test), no_std)]
#![feature(abi_x86_interrupt)]

pub mod gdt;
pub mod interrupts;
pub mod vga_buffer;
