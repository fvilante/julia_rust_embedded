use core::{cell::{Cell, RefCell, RefMut, Ref, UnsafeCell}, marker::PhantomData};
use heapless::Vec;

use crate::utils::common::usize_to_u8_clamper;


/// Areana Element Refewrence
#[derive(Clone)]
pub struct ArenaId<T> {
    index: u8,
    phantom: PhantomData<T>
}

pub struct Arena<T, const CAPACITY: usize> {
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

    pub fn borrow_mut(&self, arena_id: ArenaId<T>) -> RefMut<T> {
        let index = arena_id.index as usize;
        let data_base = unsafe { &mut *self.data_base.get() };
        let element = data_base.get_mut(index).unwrap();
        let ref_mut = (*element).borrow_mut();
        ref_mut
    }

    pub fn borrow(&self, handler: ArenaId<T>) -> Ref<T> {
        let data_base = unsafe { &mut *self.data_base.get() };
        let ref_ref_element = data_base.get(handler.index as usize).unwrap();
        let ref_element = ref_ref_element;
        let borrow = ref_element.borrow();
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
        let actual = *arena.borrow(handler);
        assert_eq!(actual, probe); 
    }

    #[test]
    fn can_allocate_once_and_mutate() {
        let probe = 0;
        let mut arena: Arena<u8, 1> = Arena::new();
        let handler = arena.alloc(probe).unwrap();
        *arena.borrow_mut(handler.clone()) += 1;
        let actual = *arena.borrow(handler);
        let expected = 1;
        assert_eq!(actual, expected); 
    }

}
