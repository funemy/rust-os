#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use core::panic::PanicInfo;
use yzos::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

// linux start
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let x = "test";
    println!("Hello World{}", "!");
    yzos::interrupts::init_idt();

    x86_64::instructions::int3();
    println!("It did not crash!");
    loop {}
}
