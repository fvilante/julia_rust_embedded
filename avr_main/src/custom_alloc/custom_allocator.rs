// ==============================================================================
// For more see: 
//
//          https://os.phil-opp.com/heap-allocation/#a-dummyallocator
//
// ==============================================================================


use crate::board::lcd;

use alloc::alloc::{
    GlobalAlloc,
    Layout,
};

use core::ptr::null_mut;


pub struct Dummy {
    val: u8,
}

//ATTENTION: I don't know why but this Handler is necessary in code but is not being executed
unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        lcd::lcd_initialize(); 
        lcd::print("Inside Allocator. Lets halt it");
        loop { };
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        unreachable!();
    }
}

#[global_allocator]
static ALLOCATOR: Dummy = Dummy{val: 0x00};
