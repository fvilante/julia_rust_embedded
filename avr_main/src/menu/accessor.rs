//abstracts the access to a type in memory


use core::marker::PhantomData;
use core::cell::Cell;

use heapless::Vec;
use lib_1::utils::common::usize_to_u8_clamper;

pub struct Accessor<'a,T: Copy + 'a> { // size = 4 bytes
    variable: &'a mut T
}

impl<'a,T: Copy + 'a> Accessor<'a,T> {

    pub fn new(variable: &'a mut T) -> Self {
        Self {
            variable,
        }
    }

    pub fn from_accessor_controler<const SIZE: usize>(controler: &'static mut Arena<T, SIZE>, handler: ArenaId<T>) -> Self {
        let accessor = (*controler).get_mut(handler);
        Self::new(accessor.get_mut())
    }

}

impl<'a, T: Copy + 'a> /*AccessorTrait<T> for*/ Accessor<'a,T> {

    pub fn set(&mut self, value: T) {
        unsafe {
            *self.variable = value;
        }
    }

    pub fn get(&self) -> T {
        unsafe {
            *self.variable
        }
    }

}

/// Areana Element Refewrence
pub struct ArenaId<T> {
    index: u8,
    phantom: PhantomData<T>
}

pub struct Arena<T, const CAPACITY: usize> {
    data_base: Vec<Cell<T>, CAPACITY>,
}

impl<T, const SIZE: usize> Arena<T, SIZE> {
    pub const fn new() -> Self {
        Self {
            data_base: Vec::new(),
        }
    }

    fn make_hash(&self, index: usize) -> u8 {
        usize_to_u8_clamper(index)
    }

    /// Allocates a new arena_bucket of type T; returns None if data_base is out of capacity.
    pub fn alloc(&mut self, initial_value: T) -> Option<ArenaId<T>> {
        let new_cell = Cell::new(initial_value);
        let index = self.data_base.len();
        let arena_index = self.make_hash(index);
        let result = self.data_base.push(new_cell);
        match result {
            Ok(_) => Some(ArenaId{ index: arena_index, phantom: PhantomData }),
            Err(_) => None,
        }
    }

    pub fn get_mut(&mut self, arena_id: ArenaId<T>) -> &mut Cell<T> {
        self.data_base.get_mut(arena_id.index as usize).unwrap()
    }

    pub fn get(&self, handler: ArenaId<T>) -> &Cell<T> {
        self.data_base.get(handler.index as usize).unwrap()
    }

}




