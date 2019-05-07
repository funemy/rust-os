use crate::println;
use core::fmt::Debug;

#[derive(Copy, Clone)]
pub struct LinkedListNode<T>
where
    T: Default + Eq + Copy + Debug,
{
    next: *mut LinkedListNode<T>,
    content: T,
}

impl<T> LinkedListNode<T>
where
    T: Default + Eq + Copy + Debug,
{
    pub fn new(next: *mut LinkedListNode<T>, content: T) -> Self {
        LinkedListNode {
            next: next,
            content: content,
        }
    }

    pub fn init(&mut self, content: T) {
        self.next = core::ptr::null_mut();
        self.content = content;
    }
}

impl<T> Default for LinkedListNode<T>
where
    T: Default + Eq + Copy + Debug,
{
    fn default() -> Self {
        LinkedListNode {
            next: core::ptr::null_mut(),
            content: Default::default(),
        }
    }
}

pub struct LinkedList<T>
where
    T: Default + Eq + Copy + Debug,
{
    head: LinkedListNode<T>,
    size: usize,
}


impl<T> LinkedList<T>
where
    T: Default + Eq + Copy + Debug,
{
    pub fn new() -> Self {
        let list = LinkedList {
            head: Default::default(),
            size: 0,
        };
        list
    }

    // append a new node at the head of the linked list
    pub fn append(&mut self, node: *mut LinkedListNode<T>) {
        if self.head.next.is_null() {
            self.head.next = node;
        } else {
            unsafe { (*node).next = self.head.next };
            self.head.next = node;
        }
        self.size += 1;
    }

    // pub fn append_val(&mut self, content: T) {
    //     self.append(&mut LinkedListNode::<T>::new(
    //         core::ptr::null_mut(),
    //         content,
    //     ));
    // }

    // remove the first node (starting from the head)
    // whose content is the same as the given `content`
    pub fn remove(&mut self, content: T) {
        // pointer that points to the current node
        let mut pt = &mut self.head;
        while !pt.next.is_null() {
            let node_content = unsafe { (*pt.next).content };
            if node_content == content {
                pt.next = unsafe { (*pt.next).next };
                self.size -= 1;
                // TODO: thr raw pointer have no gc?
                return;
            }
            pt = unsafe { &mut (*pt.next) };
        }
    }

    pub fn size(&mut self) -> usize {
        self.size
    }

    // for testing purpose
    pub fn show_complete_list(&mut self) {
        println!("{:?}", self.size());
        let mut pt = &mut self.head;
        while !pt.next.is_null() {
            println!("{:?}", unsafe { (*pt.next).content });
            pt = unsafe { &mut (*pt.next) };
        }
    }
}

use bitflags::bitflags;
bitflags! {
    pub struct PageFlags: u32 {
        const FRESH = 0x0;
        const DIRTY = 0x1;
        const RESERVED = 0x2;
        const HEAD = 0x4;
    }
}

pub struct Page {
    flgs: PageFlags,
    count: u16,
    direct: usize,
    level: u32,
    index: usize,
}

impl Page {
    pub fn init(&mut self, flags: PageFlags, direct: usize, index: usize) {
        self.flgs = flags;
        self.count = 0;
        self.direct = direct;
        self.level = 0;
        self.index = index;
    }

    pub fn get_flgs(&self) -> PageFlags {
        self.flgs
    }

    pub fn get_count(&self) -> u16 {
        self.count
    }

    pub fn get_direct(&self) -> usize {
        self.direct
    }

    pub fn get_level(&self) -> u32 {
        self.level
    }
    
    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn set_flgs(&mut self, flgs: PageFlags) {
        self.flgs = flgs;
    }

    pub fn add_flgs(&mut self, flgs: PageFlags) {
        self.flgs = self.flgs | flgs;
    }

    pub fn reset_flgs(&mut self) {
        self.flgs = self.flgs ^ self.flgs;
    }

    pub fn set_level(&mut self, level: u32) {
        self.level = level;
    }
}
