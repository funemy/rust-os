use x86_64::structures::paging::{MappedPageTable, MapperAllSizes, PageTable, PhysFrame};
use x86_64::{PhysAddr, VirtAddr};

pub unsafe fn init(physical_memory_offset: u64) -> impl MapperAllSizes {
    let level_4_table = active_level_4_table(physical_memory_offset);
    let phys_to_virt = move |frame: PhysFrame| -> *mut PageTable {
        let phys = frame.start_address().as_u64();
        let virt = VirtAddr::new(phys + physical_memory_offset);
        virt.as_mut_ptr()
    };
    MappedPageTable::new(level_4_table, phys_to_virt)
}

// simple helper function taken from the `init` function above
pub fn phys2virt(phys_addr: usize, physical_memory_offset: usize) -> usize {
    phys_addr + physical_memory_offset
}

// NOTE: returns a mutable reference to the active level 4 table
//
// This function is unsafe because the caller must guarantee that the
// complete physical memory is mapped to virtual memory at the passed
// `physical_memory_offset`. Also, this function must be only called once
// to avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn active_level_4_table(physical_memory_offset: u64) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let phys = level_4_table_frame.start_address();
    let virt = VirtAddr::new(phys.as_u64() + physical_memory_offset);
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr
}

use x86_64::structures::paging::{FrameAllocator, FrameDeallocator, Mapper, Page, Size4KiB};

pub fn create_example_mapping(
    page: Page,
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) {
    use x86_64::structures::paging::PageTableFlags as Flags;

    let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
    let flags = Flags::PRESENT | Flags::WRITABLE;

    let map_to_result = unsafe { mapper.map_to(page, frame, flags, frame_allocator) };
    map_to_result.expect("map_to failed").flush();
}


use bootloader::bootinfo::{MemoryMap, MemoryRegionType};


pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    // usable_frames always return the same result
    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);

        let addr_ranges = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));

        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        // self.free_list.append(&mut LinkedListNode::<usize>::new(,next))
        self.next += 1;
        frame
    }
}

impl FrameDeallocator<Size4KiB> for BootInfoFrameAllocator {
    fn deallocate_frame(&mut self, frame: PhysFrame<Size4KiB>) {
        // frame.start_address()
    }
}

use crate::println;

pub fn get_whole_memory(memory_map: &'static MemoryMap) {
    let regions = memory_map.iter();
    let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
    // let addr_ranges = usable_regions.map(|r| {
    //     println!("{:?}", r.range.start_addr());
    //     // println!("{:?}", FRAME_START_ADDRESS);
    //     if r.range.start_addr() == (FRAME_START_ADDRESS as u64) {
    //         r.range.start_addr()..r.range.end_addr()
    //     } else {
    //         0..0
    //     }
    // });
    // let frame_addresses = addr_ranges.flat_map(|r| r.step_by(4096));
    for (i, r) in usable_regions.enumerate() {
        println!("start{}:{:?}", i, r.range.start_addr());
    }
}
