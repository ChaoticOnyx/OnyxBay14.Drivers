#![allow(clippy::missing_safety_doc)]
#![no_std]

use core::{alloc::GlobalAlloc, cell::RefCell};

use linked_list_allocator::Heap as LLHeap;

pub struct Heap {
    heap: RefCell<LLHeap>,
}

impl Heap {
    pub unsafe fn init(&self, start: *mut u8, size: usize) {
        self.heap.borrow_mut().init(start, size);
    }

    pub const fn empty() -> Self {
        let heap = LLHeap::empty();

        Self {
            heap: RefCell::new(heap),
        }
    }
}

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        self.heap
            .borrow_mut()
            .allocate_first_fit(layout)
            .ok()
            .map_or(core::ptr::null_mut(), |all| all.as_ptr())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        assert_ne!(ptr, core::ptr::null_mut());

        self.heap
            .borrow_mut()
            .deallocate(core::ptr::NonNull::new_unchecked(ptr), layout)
    }
}

unsafe impl Sync for Heap {}
