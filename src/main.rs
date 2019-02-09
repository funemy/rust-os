#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

// static HELLO : &[u8] = b"Hello World!";

// linux start
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let x = "test";
    println!("Hello World{}", "!");
    println!("{}", x);
    panic!("this is a panic");
    loop {}
}
