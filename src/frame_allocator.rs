use core::default::Default;
use core::ptr;
use core::mem::size_of;
use libm::{log2, log2f, ceil, floor};
use x86_64::PhysAddr;
use x86_64::structures::paging::page::{PageSize, Size4KiB};
use x86_64::structures::paging::{FrameAllocator, frame::PhysFrame};
use crate::mm_types::{Page, PageFlags};
use crate::data_structures::{LinkedList, LinkedListNode};
use crate::memory::phys_to_virt;