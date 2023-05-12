use core::{
    cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut, UnsafeCell},
    marker::PhantomData,
    ops::{Deref, DerefMut},
};
use heapless::Vec;

use crate::utils::numerical::usize_to_u8_clamper;

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
#[derive(Copy, Clone)]
pub struct ArenaId<T> {
    index: u8,
    phantom: PhantomData<T>,
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
    pub fn alloc(&self, initial_value: T) -> Option<ArenaId<T>> {
        let new_value = RefCell::new(initial_value);
        let data_base = unsafe { &mut *self.data_base.get() };
        let index = data_base.len();
        let arena_index = Self::make_hash(index);
        let result = data_base.push(new_value);
        match result {
            Ok(_) => Some(ArenaId {
                index: arena_index,
                phantom: PhantomData,
            }),
            Err(_) => None,
        }
    }

    /// Returns error if aliasing rules violated at run-time otherwhise returns the mutable reference
    /// Equivalent to RefCell::try_borrow_mut
    /// Note, see also: https://stackoverflow.com/a/51349578/8233039
    pub fn try_borrow_mut<'a>(
        &'a self,
        arena_id: ArenaId<T>,
    ) -> Result<RefMut<'a, T>, BorrowMutError> {
        let index = arena_id.index as usize;
        let data_base = unsafe { &mut *self.data_base.get() };
        let element = data_base.get_mut(index).unwrap();
        let ref_mut = (*element).try_borrow_mut();
        ref_mut
    }

    /// Returns error if aliasing rules violated at run-time otherwhise returns the imutable reference
    /// Equivalent to RefCell::try_borrow
    /// Note, see also: https://stackoverflow.com/a/51349578/8233039
    pub fn try_borrow<'a>(&'a self, handler: ArenaId<T>) -> Result<Ref<'a, T>, BorrowError> {
        let data_base = unsafe { &mut *self.data_base.get() };
        let ref_ref_element = data_base.get(handler.index as usize).unwrap();
        let ref_element = ref_ref_element;
        let borrow = ref_element.try_borrow();
        borrow
    }

    /// Attention may fail at run-time, prefer try_borrow to a infalible action
    pub fn borrow<'a>(&'a self, handler: ArenaId<T>) -> Ref<'a, T> {
        self.try_borrow(handler).unwrap()
    }

    /// Attention may fail at run-time, prefer try_borrow_mut to a infalible action
    pub fn borrow_mut<'a>(&'a self, handler: ArenaId<T>) -> RefMut<'a, T> {
        self.try_borrow_mut(handler).unwrap()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_allocate_once_immutable() {
        let arena: Arena<u8, 1> = Arena::new();
        let probe = 0;
        let handler = arena.alloc(probe).unwrap();
        let actual = *arena.borrow(handler);
        assert_eq!(actual, probe);
    }

    fn produce_error_if_allocate_more_than_its_capacity() {
        let arena: Arena<u8, 2> = Arena::new();
        let _handler1 = arena.alloc(6).unwrap();
        let _handler2 = arena.alloc(7).unwrap();
        let should_be_error_handler = arena.alloc(8);
        match should_be_error_handler {
            Some(_) => assert!(false),
            None => assert!(true), // happy path
        }
    }

    #[test]
    fn can_allocate_once_and_mutate() {
        let probe = 0;
        let arena: Arena<u8, 1> = Arena::new();
        let handler = arena.alloc(probe).unwrap();
        *arena.borrow_mut(handler) += 1;
        let actual = *arena.borrow(handler);
        let expected = 1;
        assert_eq!(actual, expected);
    }

    #[test]
    fn can_allocate_twice_and_mutate() {
        let probe = 0;
        let arena: Arena<u8, 2> = Arena::new();
        let handler1 = arena.alloc(probe).unwrap();
        let handler2 = arena.alloc(probe).unwrap();
        *arena.borrow_mut(handler1) += 1;
        *arena.borrow_mut(handler2) += 2;
        let actual1 = *arena.borrow(handler1);
        let actual2 = *arena.borrow(handler2);
        let expected1 = 1;
        let expected2 = 2;
        assert_eq!(actual1, expected1);
        assert_eq!(actual2, expected2);
    }

    #[test]
    fn can_allocate_once_and_borrow_mut_only_once_at_a_time() {
        let probe = 0;
        let arena: Arena<u8, 2> = Arena::new();
        let handler = arena.alloc(probe).unwrap();
        {
            let mut borrow_mut1 = arena.borrow_mut(handler);
            *borrow_mut1 += 1;
        }
        {
            let mut borrow_mut2 = arena.borrow_mut(handler);
            *borrow_mut2 += 2;
        }
        let actual1 = *arena.borrow(handler);
        let actual2 = *arena.borrow(handler);
        let expected1 = 3;
        let expected2 = 3;
        assert_eq!(actual1, expected1);
        assert_eq!(actual2, expected2);
    }

    #[test]
    fn can_allocate_once_and_borrow_immutable_many_at_same_time() {
        let probe = 7;
        let arena: Arena<u8, 2> = Arena::new();
        let handler = arena.alloc(probe).unwrap();
        let borrow_1 = arena.borrow(handler);
        let borrow_2 = arena.borrow(handler);
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
        let arena: Arena<u8, 2> = Arena::new();
        let handler = arena.alloc(probe).unwrap();
        let borrow_mut1 = arena.borrow_mut(handler);
        let should_be_runtime_error = arena.try_borrow_mut(handler);
        match should_be_runtime_error {
            Ok(_) => assert!(false),
            Err(_) => assert!(true), // happy path
        }
    }
}
