use crate::memory::BootInfoFrameAllocator;

pub struct HeapAllocator {
    frame_allocator: BootInfoFrameAllocator,
}