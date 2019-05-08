use crate::data_structures::{FrameFlags, FrameInfo, LinkedList, LinkedListNode};
use crate::memory::phys2virt;

use core::mem::size_of;
use core::ptr;
use libm::{ceil, floor, log2, log2f};
use x86_64::structures::paging::page::{PageSize, Size4KiB};
use x86_64::structures::paging::{frame::PhysFrame, FrameAllocator};
use x86_64::PhysAddr;

const MAX_LEVEL: usize = 11;
// const MAX_LEVEL: usize = 1;
// NOTE: very odd, the memory_map given by bootloader only has 2 usable memory regions
const MAX_REGION_NUM: usize = 2;

// memory region, corresponds to the MemoryRegion in memory_map
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Region {
    // each node in the linked list denote a buddy
    // the content of each node is the start index of the frame
    // we can use this index to access FrameInfo object
    free_lists: [LinkedList<usize>; MAX_LEVEL],
    size: usize,
    free_frame_num: usize,
    // the base frame index of the region
    base_frame_idx: usize,
    // the start (usable) frame number of the region
    start_frame_idx: usize,
    memory_map: *mut FrameInfo,
}

impl Default for Region {
    fn default() -> Self {
        Region {
            free_lists: Default::default(),
            size: 0,
            free_frame_num: 0,
            base_frame_idx: 0,
            start_frame_idx: 0,
            memory_map: ptr::null_mut(),
        }
    }
}


use crate::is_power2;
use crate::println;
impl Region {
    pub fn new(size: usize, base_frame_num: usize) -> Self {
        let mut region = Region {
            size: size,
            base_frame_idx: base_frame_num,
            ..Default::default()
        };
        let page_size: usize = Size4KiB::SIZE as usize;
        let info_frame_num: usize = required_frame_num(size_of::<FrameInfo>() * size, page_size);
        region.start_frame_idx = region.base_frame_idx + info_frame_num;
        region.size -= info_frame_num;
        region.free_frame_num = region.size;

        let physical_memory_offset = unsafe { crate::physical_memory_offset };
        region.memory_map =
            phys2virt(region.base_frame_idx * page_size, physical_memory_offset) as *mut FrameInfo;

        region.init_memory_map();
        // region.init_buddy_system();
        region.init_free_list();
        region
    }

    fn init_memory_map(&mut self) {
        let physical_memory_offset = unsafe { crate::physical_memory_offset };
        let page_size = Size4KiB::SIZE as usize;

        for idx in 0..self.size {
            // since it's complicated to pre-assign a array,
            // we use offset to access each FrameInfo object
            // similar to memroy_map[idx] in C.
            let frame_info: &mut FrameInfo = unsafe { &mut *self.memory_map.offset(idx as isize) };
            let global_idx = self.start_frame_idx + idx;
            let phys_addr = global_idx * page_size;
            let virt_addr = phys2virt(phys_addr, physical_memory_offset);
            frame_info.init(FrameFlags::FREE, virt_addr, global_idx);
        }
    }

    fn init_free_list(&mut self) {
        let largest_possible_level: usize = floor(log2(self.size as f64)) as usize;
        if largest_possible_level >= MAX_LEVEL - 1 {
            let power: u32 = (MAX_LEVEL as u32) - 1;
            let block_num = self.size / 2usize.pow(power);

            for block_idx in 0..block_num {
                let page_idx = block_idx * 2usize.pow(power);

                // FIXME: might be buggy on frame_info
                let frame_info =
                    unsafe { self.memory_map.offset(page_idx as isize) as *mut FrameInfo };
                let free_node: *mut LinkedListNode<usize> =
                    unsafe { (*frame_info).get_direct_access() as *mut LinkedListNode<usize> };

                unsafe { (*free_node).init(page_idx) };
                // here the index of free_lists is usize
                self.free_lists[MAX_LEVEL - 1].append(free_node);
                unsafe { (*frame_info).set_level(power) };
            }

            let rest_frame_idx = block_num * 2usize.pow(power);
            let rest_size = self.size - rest_frame_idx;
            self.process_rest(rest_frame_idx, rest_size);
        } else {
            self.process_rest(0, self.size)
        }
    }

    // take care of 2 situations:
    // 1. when the whole physical memory is smaller than MAX_LEVEL
    // 2. when the physical memory is not a multiple of MAX_LEVEL
    // either way, we downgrade the piece of memory recursively
    // NOTE: could leads to bug
    // The allocator may give out insufficient frames
    fn process_rest(&mut self, frame_idx: usize, size: usize) {
        // println!("be careful about infinite recursion");
        if size > 0 {
            let level: usize = floor(log2(size as f64)) as usize;
            let frame_info = unsafe { self.memory_map.offset(frame_idx as isize) };
            let free_node: *mut LinkedListNode<usize> =
                unsafe { (*frame_info).get_direct_access() as *mut LinkedListNode<usize> };
            unsafe { (*free_node).init(frame_idx) };
            self.free_lists[level].append(free_node);
            unsafe { (*frame_info).set_level(level as u32) };

            let buddy_size = 2usize.pow(level as u32);
            if buddy_size < size {
                let rest_frame_idx = frame_idx + buddy_size;
                let rest_size = size - buddy_size;
                self.process_rest(rest_frame_idx, rest_size);
            }
        } else {
            return;
        }
    }

    pub fn split(&mut self, target_level: usize) -> bool {
        if target_level >= MAX_LEVEL {
            return false;
        }

        let cur_level = target_level;
        if self.free_lists[cur_level].size() > 0 {
            return true;
        }

        if self.split(cur_level + 1) {
            let free_node: *mut LinkedListNode<usize> = self.free_lists[cur_level + 1].pop();
            if !free_node.is_null() {
                let mut frame_idx = unsafe { (*free_node).content };
                self.free_lists[cur_level].append(free_node);

                // change the first half to lower level
                let frame_info = unsafe { &mut *self.memory_map.offset(frame_idx as isize) };
                frame_info.set_level(cur_level as u32);

                // find the second half
                let half_frame_idx = frame_idx + 2usize.pow(cur_level as u32);
                let half_frame_info = unsafe { self.memory_map.offset(half_frame_idx as isize) };
                let split_free_node: *mut LinkedListNode<usize> =
                    unsafe { (*half_frame_info).get_direct_access() as *mut LinkedListNode<usize> };
                unsafe { (*split_free_node).init(half_frame_idx) };
                self.free_lists[cur_level].append(split_free_node);
                unsafe { (*half_frame_info).set_level(cur_level as u32) };
                return true;
            }
        }

        false
    }

    pub fn request_frames(&mut self, frame_num: usize) -> Option<&'static mut FrameInfo> {
        if frame_num == 0 || frame_num > self.size {
            return None;
        } else {

            // simplified implementation
            // always return one level buddy frame
            if is_power2(frame_num) {
                // FIXME:
                // might have bug here
                let level = log2f(frame_num as f32) as usize;
                if level > (MAX_LEVEL - 1) {
                    return None;
                }

                // handle the case where we first need to do a split
                if self.free_lists[level].size() == 0 {
                    if !self.split(level) {
                        return None;
                    }
                }

                let free_node: *mut LinkedListNode<usize> = self.free_lists[level].pop();
                if free_node.is_null() {
                    return None;
                }
                let frame_idx = unsafe { (*free_node).content };
                let requested_frame = unsafe { &mut *self.memory_map.offset(frame_idx as isize) };
                // TODO: do I need to mark the rest of frame as "TAKEN"?
                requested_frame.add_flgs(FrameFlags::HEAD);
                self.free_frame_num -= 1 << level;
                return Some(requested_frame);
            }
        }
        None
    }

    pub fn retrieve_frame(&mut self, frame_info: &mut FrameInfo) {
        let frame_idx = frame_info.get_index();
        // should not happen
        if frame_idx < self.start_frame_idx || frame_idx > (self.start_frame_idx + self.size) {
            return;
        }

        let rel_frame_idx = frame_idx - self.start_frame_idx;
        let mut level = frame_info.get_level() as usize;
        self.free_frame_num -= 1 << level;
        // NOTE: this is the merge process
        // NOTE: this merge has a problem. Should I only consider merging only the frames that
        // `used` to be a whole chunk?
        // FIXME: may be buggy
        while level < (MAX_LEVEL - 1) {
            let half_frame_idx = rel_frame_idx + 2usize.pow(level as u32);
            if half_frame_idx > self.size {
                break;
            }
            let half_frame = unsafe { &mut *self.memory_map.offset(half_frame_idx as isize) };
            if !is_free_buddy_frame(half_frame, level as u32) {
                break;
            }
            // reset the level of this frame chunk
            // meaning it is merged
            half_frame.set_level(0);
            self.free_lists[level].remove(half_frame_idx);
            // the level is upgraded
            level += 1;
        }
        // put the merged frame back into the corresponding
        let merged_frame = unsafe { &mut *self.memory_map.offset(frame_idx as isize) };
        merged_frame.set_level(level as u32);
        let node = merged_frame.get_direct_access() as *mut LinkedListNode<usize>;
        unsafe { (*node).init(frame_idx) };
        self.free_lists[level].append(node);
    }
}

#[derive(Default)]
#[repr(C)]
pub struct SimpleFrameAllocator {
    regions: [Region; MAX_REGION_NUM],
    region_num: usize,
}

impl SimpleFrameAllocator {
    pub fn register_region(&mut self, region: Region) {
        self.regions[self.region_num] = region;
        self.region_num += 1;
    }

    pub fn alloc_frames(&mut self, frame_num: usize) -> Option<&'static mut FrameInfo> {
        // I do reverse order because the second region is larger
        for region_idx in (0..MAX_REGION_NUM).rev() {
            if let Some(frame_info) = self.regions[region_idx].request_frames(frame_num) {
                return Some(frame_info);
            }
        }
        None
    }

    pub fn dealloc_frame(&mut self, frame_info: &mut FrameInfo) {
        for region_idx in (0..MAX_REGION_NUM).rev() {
            self.regions[region_idx].retrieve_frame(frame_info);
        }
    }

    pub fn region_num(&self) -> usize {
        self.region_num
    }
}

// align a size number to a multiple of the unit
// unit must be power of 2
fn align_to(size: usize, page_size: usize) -> usize {
    if size % page_size == 0 {
        size
    } else {
        // TODO: can be optimize to
        // (size + unit - 1) & (unit - 1)
        let unit_num = size / page_size;
        page_size * (unit_num + 1)
    }
}

fn required_frame_num(size: usize, page_size: usize) -> usize {
    align_to(size, page_size) / page_size
}

fn is_free_buddy_frame(frame_info: &mut FrameInfo, level: u32) -> bool {
    let flgs = frame_info.get_flgs();
    // NOTE:
    // 1. the frame is at the same level
    // 2. the frame is not mapped
    // 3. the frame is not TAKEN, this probably is not used in this implementation
    // 4. the frame is not HEAD, meaning it is not allocated
    if (frame_info.get_level() == level)
        && (frame_info.get_count() == 0)
        && ((flgs.bits() & FrameFlags::TAKEN.bits()) == 0)
        && ((flgs.bits() & FrameFlags::HEAD.bits()) == 0)
    {
        return true;
    } else {
        return false;
    }
}

unsafe impl FrameAllocator<Size4KiB> for SimpleFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        if let Some(frame_info) = self.alloc_frames(1) {
            let frame_size = Size4KiB::SIZE as usize;
            return Some(PhysFrame::containing_address(PhysAddr::new((frame_info.get_index() * frame_size) as u64)));
        }
        None
    }
}