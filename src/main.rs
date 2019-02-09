#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// static HELLO : &[u8] = b"Hello World!";

// linux start
#[no_mangle]
pub extern "C" fn _start() -> ! {

    println!("Hello World{}", "!");

    loop {}
}
