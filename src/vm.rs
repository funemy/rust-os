use crate::data_structures::{LinkedList, LinkedListNode};
use crate::frame_allocator::SimpleFrameAllocator;

// NOTE: this definition is taken from linux design doc
const LEVEL_NUM: usize = 8;
// the unit of the size is "byte"
const SIZE_LEVEL: [usize; LEVEL_NUM] = [32, 64, 128, 256, 512, 1024, 2048, 4096];

#[derive(Default)]
#[repr(C)]
pub struct KernelHeapAllocator {
    pre_alloc_memory: [LinkedList<usize>; LEVEL_NUM],
    frame_allocator: SimpleFrameAllocator,
}

use core::alloc::Layout;
use core::ptr;
use x86_64::structures::paging::page::{PageSize, Size4KiB};

impl KernelHeapAllocator {
    // the pre_alloc_frame_num is the number of frames we spend on each level of obj
    pub fn new(frame_allocator: SimpleFrameAllocator, pre_alloc_frame_num: usize) -> Self {
        let mut kallocator = KernelHeapAllocator {
            pre_alloc_memory: Default::default(),
            frame_allocator: frame_allocator,
        };
        kallocator.init(pre_alloc_frame_num);
        kallocator
    }

    // TODO: here I make it much simpler
    // There's no mapping created on page table
    // I just used the complete map region
    fn init(&mut self, pre_alloc_frame_num: usize) {
        for level in 0..LEVEL_NUM {
            let page_size = Size4KiB::SIZE as usize;
            let pre_alloc_size = SIZE_LEVEL[level];
            let obj_num = (page_size * pre_alloc_frame_num) / pre_alloc_frame_num;
            if let Some(frame_info) = self.frame_allocator.alloc_frames(pre_alloc_frame_num) {
                let base_vir_addr = frame_info.get_direct_access();
                for i in 0..obj_num {
                    let virt_addr = base_vir_addr + i * pre_alloc_size;
                    let obj_node = virt_addr as *mut LinkedListNode<usize>;
                    unsafe { (*obj_node).init(virt_addr) };
                    self.pre_alloc_memory[level].append(obj_node);
                }
            }
        }
    }

    pub fn get_frame_allocator(&mut self) -> &mut SimpleFrameAllocator {
        &mut self.frame_allocator
    }

    pub fn malloc(&mut self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        // check all pre alloc sizes
        for level in 0..LEVEL_NUM {
            // if we need a larger space than current level obj
            // go into next iteration
            if size > SIZE_LEVEL[level] {
                continue;
            } else {
                let obj_node = self.pre_alloc_memory[level].pop();
                return unsafe { (*obj_node).content as *mut u8 };
            }
        }
        ptr::null_mut()
    }

    pub fn free(&mut self, ptr: *mut u8, layout: Layout) {
        let size = layout.size();

        for level in 0..LEVEL_NUM {
            if size > SIZE_LEVEL[level] {
                continue;
            } else {
                let obj_node = ptr as *mut LinkedListNode<usize>;
                unsafe { (*obj_node).init(ptr as usize) };
                self.pre_alloc_memory[level].append(obj_node);
                return;
            }
        }
    }
}