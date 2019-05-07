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
use bootloader::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    // let x = "test";
    println!("Hello World{}", "!");
    yzos::init();

    // test_linked_list();

    // NOTE: trigger a breakpoint interrupt
    // x86_64::instructions::int3();

    // NOTE: trigger double fault
    // fn stack_overflow() {
    //     stack_overflow();
    // }
    // stack_overflow();

    // NOTE: trigger a deadlock between the main thread and interrupt due to the lock in print!
    // loop {
    //     use yzos::print;
    //     print!("-");
    //     for _ in 0..10000 {}
    // }

    // NOTE: access some memory outside kernel
    // let ptr = 0xdeadbeaf as *mut u32;
    // unsafe {
    //     *ptr = 42;
    // }

    // NOTE: accessing the physical address of level 4 page table
    // use x86_64::registers::control::Cr3;

    // let (level_4_page_table, _) = Cr3::read();
    // println!(
    //     "Level 4 page table at {:?}",
    //     level_4_page_table.start_address()
    // );

    // NOTE: iterate over active level 4 and 3 page tables
    // use yzos::memory::active_level_4_table;
    // let l4_table = unsafe { active_level_4_table(boot_info.physical_memory_offset) };

    // use x86_64::{structures::paging::PageTable, VirtAddr};
    // for (i, entry) in l4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         println!("L4 Entry {}: {:?}", i, entry);

    //         let phys = entry.frame().unwrap().start_address();
    //         let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //         let ptr = VirtAddr::new(virt).as_mut_ptr();
    //         let l3_table: &PageTable = unsafe { &*ptr };

    //         for (i, entry) in l3_table.iter().enumerate() {
    //             if !entry.is_unused() {
    //                 println!("L3 Entry {}: {:?}", i, entry);
    //             }
    //         }
    //     }
    // }

    //NOTE: for testing translating virtual address into physical address
    // use x86_64::{structures::paging::MapperAllSizes, VirtAddr};
    // use yzos::memory;

    // let mapper = unsafe { memory::init(boot_info.physical_memory_offset) };

    // let addresses = [
    //     // identity-mapped vga buffer page
    //     0xb8000,
    //     // some code page
    //     0x20010a,
    //     // some stack page
    //     0x57ac_001f_fe48,
    //     boot_info.physical_memory_offset,
    // ];

    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     let phys = mapper.translate_addr(virt);
    //     println!("{:?} -> {:?}", virt, phys);
    // }

    // NOTE: map a page to vga buffer and write to it
    // use x86_64::{structures::paging::Page, VirtAddr};
    // use yzos::memory;

    // let mut mapper = unsafe { memory::init(boot_info.physical_memory_offset) };
    // let mut frame_allocator =
    //     unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };

    // memory::get_whole_memory(&boot_info.memory_map);

    // let page = Page::containing_address(VirtAddr::new(0x1000));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    // println!("{:?}", boot_info.memory_map.len());
    println!("It did not crash!");

    yzos::hlt_loop();
}

use yzos::data_structures::{LinkedList, LinkedListNode};
fn test_linked_list() {
    let mut linked_list = LinkedList::<usize>::new();
    linked_list.append(&mut LinkedListNode::<usize>::new(core::ptr::null_mut(), 5));
    linked_list.append(&mut LinkedListNode::<usize>::new(core::ptr::null_mut(), 6));
    linked_list.append(&mut LinkedListNode::<usize>::new(core::ptr::null_mut(), 7));
    linked_list.append(&mut LinkedListNode::<usize>::new(core::ptr::null_mut(), 8));
    linked_list.append(&mut LinkedListNode::<usize>::new(
        core::ptr::null_mut(),
        100,
    ));
    // linked_list.append_val(8);
    linked_list.show_complete_list();
    println!("Now testing remove");
    linked_list.remove(6);
    linked_list.remove(7);
    linked_list.show_complete_list();
}
