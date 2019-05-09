# CSCE 613 Final Project - Extended Rust Micro Kernel

**Name: Yanze Li 		UIN: 326006687**

## Project Description

This goal of this project is using Rust programming language to implement a micro-kernel prototype. The intended features includes **Frame Allocator**, **Heap Allocator**, **Thread**, and **Process**.

The starting point of the project is a minimal bootable kernel with interrupt and VGA text mode support, plus a very limited frame allocator.

Therefore, in my project, I re-implemented the frame allocator from scratch and build a kernel heap allocator based on it. For the thread/process implementation, it is only required to support single-threaded process, therefore, the key difference between the thread and process in my project whether having a separate memory space.

By the time this report was written, the frame allocator and heap allocator has been fully implemented, with some passed test cases available. The heap allocator can successfully replace the global heap allocator, so we can use `Box` and `vec!` in the rest implementation. The implementation of thread/process is only partly done. I'm working on implementing the rest, and hopefully this will be done by the review session.

## References

For frame allocation and heap allocation, following materials are referred:

1. [Writing an OS in Rust (First Edition)](https://os.phil-opp.com/first-edition/)
2. [OS Dev](https://wiki.osdev.org/Page_Frame_Allocation)
3. [Understanding the Linux Virtual Memory Manager](https://www.kernel.org/doc/gorman/html/understand/understand011.html)
4. CSCE 611 Machine Problem code

For thread/process:

1. [Redox Source code](https://gitlab.redox-os.org/redox-os/kernel/blob/master/src/context/arch/x86_64.rs) (a Unix-like OS written in Rust)
2. Some documentations about X64 registers, such as (this)[https://docs.microsoft.com/en-us/windows-hardware/drivers/debugger/x64-architecture] and (this)[https://stackoverflow.com/questions/18024672/what-registers-are-preserved-through-a-linux-x86-64-function-call]

In addition to the materials mentioned above, I also discussed the design and some implementation difficulties with my lambastes (who are also in the class).

## Design

#### Frame Allocation

My original design was directly taking my work from CSCE 611, which used a bitmap to manage the whole physical memory and scan the bitmap to allocate a continuous chunk of frames. The implementation ended up being ugly and and inefficient.

After some discussion with classmates, I decide to re-implement a buddy based frame allocator. To enable separate memory spaces, I manage each separate physical memory space as `Region` (similar to the concept of `frame_pool` in CSCE 611). This design is inspired by the `MemoryRegion` structure inside `BootInfo` crate. The definition of `Region` is as followed:

```rust
pub struct Region {
    // each node in the linked list denote a buddy
    // the content of each node is the start index of the frame
    // we can use this index to access FrameInfo object
    free_lists: [LinkedList<usize>; LEVEL_NUM],
    size: usize,
    free_frame_num: usize,
    // the base frame index of the region
    base_frame_idx: usize,
    // the start (usable) frame number of the region
    start_frame_idx: usize,
    frame_map: *mut FrameInfo,
}
```

The `free_lists` is a set of linked list that store buddies at different level. `size` denotes the total frame number of the region, `free_frame_num` denotes the number of frames that are free for allocation. We need to take some frames out from the region to store the meta-information of this region, which is stored in `frame_map`

#### Heap Allocation

#### Thread/Process

## Implementation



## Conclusion & Future Work































[link text][ref1]