// ==============================================================================
// For more see:
//
//          https://os.phil-opp.com/heap-allocation/#a-dummyallocator
//
// ==============================================================================

//use crate::board::lcd;

use alloc::alloc::{GlobalAlloc, Layout};

pub struct Dummy {
    val: u8,
}

struct Reservation {
    start_position: u8,
    size: u8,
}

//static mut INDEX: Option<[Reservation; 5]> = None;
static mut HEAP_MEMORY: [u8; 200] = [0x00; 200]; //heap memory

unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        //lcd::lcd_initialize();
        //lcd::print("Inside Allocator. Size/Alignment");
        //lcd::print_u16_in_hex(layout.size().try_into().unwrap());
        //lcd::print("/");
        //lcd::print_u16_in_hex(layout.align().try_into().unwrap());
        //loop {};
        //
        //null_mut()
        let a = HEAP_MEMORY.as_mut_ptr();
        a
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        //unreachable!();
    }
}

#[global_allocator]
static ALLOCATOR: Dummy = Dummy { val: 0x00 };
