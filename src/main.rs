#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use core::panic::PanicInfo;
use yzos::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    yzos::hlt_loop();
}

// linux start
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    use yzos::interrupts::PICS;
    // let x = "test";
    println!("Hello World{}", "!");
    yzos::gdt::init();
    yzos::interrupts::init_idt();
    unsafe { PICS.lock().initialize() }
    x86_64::instructions::interrupts::enable();

    // NOTE: trigger a breakpoint interrupt
    // x86_64::instructions::int3();

    // NOTE: trigger double fault
    // fn stack_overflow() {
    //     stack_overflow();
    // }
    // stack_overflow();

    //NOTE: trigger a deadlock between the main thread and interrupt due to the lock in print!
    // loop {
    //     use yzos::print;
    //     print!("-");
    //     for _ in 0..10000 {}
    // }

    println!("It did not crash!");

    yzos::hlt_loop();
}
