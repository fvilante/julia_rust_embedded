use core::{cell::{Cell, RefCell, RefMut, Ref, UnsafeCell, BorrowError, BorrowMutError}, marker::PhantomData, ops::{DerefMut, Deref}};
use heapless::Vec;

use crate::utils::common::usize_to_u8_clamper;


// Arena
// 
// Concepts used to understand the design of this module:
//     
//     - External concepts
//         - Typed Arenas (to avoid propagating lifetimes and make code simplier)
//     - Internal concepts
//         - Rust aliasing
//         - Interior mutability
//             - UnsafeCell
//             - RefCell
//                 - Ref
//                 - RefMut
// 

/// Areana Element Refewrence
#[derive(Copy,Clone)]
pub struct ArenaId<T> {
    index: u8,
    phantom: PhantomData<T>
}

pub struct Arena<T, const CAPACITY: usize> {
    // SAFETY: The aliasing of UnsafeCell is garanteed by the interior RefCell and to the fact that
    // UnsafeCell value is just accessed when the interior RefCell is accessed.
    data_base: UnsafeCell<Vec<RefCell<T>, CAPACITY>>, //TODO: Maybe this be converted to an array and cell.as_slice_of_cells should be used. (?!)
}

impl<T, const SIZE: usize> Arena<T, SIZE> {
    pub const fn new() -> Self {
        Self {
            data_base: UnsafeCell::new(Vec::new()),
        }
    }

    fn make_hash(index: usize) -> u8 {
        usize_to_u8_clamper(index)
    }

    /// Allocates a new arena_bucket of type T; returns None if data_base is out of capacity.
    pub fn alloc(&mut self, initial_value: T) -> Option<ArenaId<T>> {
        let new_value = RefCell::new(initial_value);
        let data_base = unsafe { &mut *self.data_base.get() };
        let index = data_base.len();
        let arena_index = Self::make_hash(index);
        let result = data_base.push(new_value);
        match result {
            Ok(_) => Some(ArenaId{ index: arena_index, phantom: PhantomData }),
            Err(_) => None,
        }
    }

    /// Returns error if aliasing rules violated at run-time otherwhise returns the mutable reference
    /// Note, see also: https://stackoverflow.com/a/51349578/8233039
    pub fn try_borrow_mut<'a>(&'a self, arena_id: ArenaId<T>) -> Result<impl DerefMut<Target = T> + 'a, BorrowMutError> {
        let index = arena_id.index as usize;
        let data_base = unsafe { &mut *self.data_base.get() };
        let element = data_base.get_mut(index).unwrap();
        let ref_mut = (*element).try_borrow_mut();
        ref_mut
    }

    /// Returns error if aliasing rules violated at run-time otherwhise returns the mutable reference
    /// Note, see also: https://stackoverflow.com/a/51349578/8233039
    pub fn try_borrow<'a>(&'a self, handler: ArenaId<T>) -> Result<impl Deref<Target = T> + 'a, BorrowError> {
        let data_base = unsafe { &mut *self.data_base.get() };
        let ref_ref_element = data_base.get(handler.index as usize).unwrap();
        let ref_element = ref_ref_element;
        let borrow = ref_element.try_borrow();
        borrow
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_allocate_once_immutable() {
        let mut arena: Arena<u8, 1> = Arena::new();
        let probe = 0;
        let handler = arena.alloc(probe).unwrap();
        let actual = *arena.try_borrow(handler).unwrap();
        assert_eq!(actual, probe); 
    }

    #[test]
    fn can_allocate_once_and_mutate() {
        let probe = 0;
        let mut arena: Arena<u8, 1> = Arena::new();
        let handler = arena.alloc(probe).unwrap();
        *arena.try_borrow_mut(handler).unwrap() += 1;
        let actual = *arena.try_borrow(handler).unwrap();
        let expected = 1;
        assert_eq!(actual, expected); 
    }

    #[test]
    fn can_allocate_twice_and_mutate() {
        let probe = 0;
        let mut arena: Arena<u8, 2> = Arena::new();
        let handler1 = arena.alloc(probe).unwrap();
        let handler2 = arena.alloc(probe).unwrap();
        *arena.try_borrow_mut(handler1).unwrap() += 1;
        *arena.try_borrow_mut(handler2).unwrap() += 2;
        let actual1 = *arena.try_borrow(handler1).unwrap();
        let actual2 = *arena.try_borrow(handler2).unwrap();
        let expected1 = 1;
        let expected2 = 2;
        assert_eq!(actual1, expected1); 
        assert_eq!(actual2, expected2); 
    }

    #[test]
    fn can_allocate_once_and_borrow_mut_once_at_a_time() {
        let probe = 0;
        let mut arena: Arena<u8, 2> = Arena::new();
        let handler = arena.alloc(probe).unwrap();
        {
            let mut borrow_mut1 = arena.try_borrow_mut(handler).unwrap();
            *borrow_mut1 += 1;
        }
        {
            let mut borrow_mut2 = arena.try_borrow_mut(handler).unwrap(); 
            *borrow_mut2 += 2;
        }
        let actual1 = *arena.try_borrow(handler).unwrap();
        let actual2 = *arena.try_borrow(handler).unwrap();
        let expected1 = 3;
        let expected2 = 3;
        assert_eq!(actual1, expected1); 
        assert_eq!(actual2, expected2); 
    }

    #[test]
    fn can_allocate_once_and_borrow_immutable_many_at_a_time() {
        let probe = 7;
        let mut arena: Arena<u8, 2> = Arena::new();
        let handler = arena.alloc(probe).unwrap();
        let borrow_1 = arena.try_borrow(handler).unwrap();
        let borrow_2 = arena.try_borrow(handler).unwrap();
        let actual1 = *borrow_1;
        let actual2 = *borrow_2;
        let expected1 = probe;
        let expected2 = probe;
        assert_eq!(actual1, expected1); 
        assert_eq!(actual2, expected2); 
    }

    #[test]
    fn cannot_allocate_once_and_reborrow_twice_at_same_time() {
        let probe = 0;
        let mut arena: Arena<u8, 2> = Arena::new();
        let handler = arena.alloc(probe).unwrap();
        let mut borrow_mut1 = arena.try_borrow_mut(handler).unwrap();
        let mut should_be_error = arena.try_borrow_mut(handler.clone()); 
        match should_be_error {
            Ok(_) => assert!(false),
            Err(_) => assert!(true), // happy path
        } 
    }

}
