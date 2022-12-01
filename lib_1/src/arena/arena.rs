use core::{cell::{Cell, RefCell}, marker::PhantomData};
use heapless::Vec;

use crate::utils::common::usize_to_u8_clamper;


/// Areana Element Refewrence
pub struct ArenaId<T> {
    index: u8,
    phantom: PhantomData<T>
}

pub struct Arena<T, const CAPACITY: usize> {
    data_base: RefCell<Vec<Cell<T>, CAPACITY>>, //TODO: Maybe this be converted to an array and cell.as_slice_of_cells should be used. (?!)
}

impl<T, const SIZE: usize> Arena<T, SIZE> {
    pub const fn new() -> Self {
        Self {
            data_base: RefCell::new(Vec::new()),
        }
    }

    fn make_hash(index: usize) -> u8 {
        usize_to_u8_clamper(index)
    }

    /// Allocates a new arena_bucket of type T; returns None if data_base is out of capacity.
    pub fn alloc(&mut self, initial_value: T) -> Option<ArenaId<T>> {
        let new_cell = Cell::new(initial_value);
        let data_base = self.data_base.get_mut();
        let index = data_base.len();
        let arena_index = Self::make_hash(index);
        let result = data_base.push(new_cell);
        match result {
            Ok(_) => Some(ArenaId{ index: arena_index, phantom: PhantomData }),
            Err(_) => None,
        }
    }

    pub fn get_mut(&mut self, arena_id: ArenaId<T>) -> &mut Cell<T> {
        let data_base = self.data_base.get_mut();
        data_base.get_mut(arena_id.index as usize).unwrap()
    }

    pub fn get(&mut self, handler: ArenaId<T>) -> &Cell<T> {
        let data_base = self.data_base.get_mut();
        data_base.get(handler.index as usize).unwrap()
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_allocate_once_mutable() {
        let mut arena: Arena<u8, 1> = Arena::new();
        let probe = 0;
        let handler = arena.alloc(probe).unwrap();
        let actual = arena.get(handler).get();
        assert_eq!(actual, probe); 
    }

    fn can_allocate_once_immutable() {
        let mut arena: Arena<u8, 1> = Arena::new();
        let probe = 0;
        let handler = arena.alloc(probe).unwrap();
        let actual = arena.get(handler).get();
        assert_eq!(actual, probe); 
    }

    #[test]
    fn it_contra_maps() {
        assert_eq!(1, 1); 
    }
}
