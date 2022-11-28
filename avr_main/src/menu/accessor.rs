//abstracts the access to a type in memory


use core::marker::PhantomData;


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
        Self::new(&mut accessor.variable)
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


pub struct Accessor2<T> { 
    variable: T 
}

impl<T> Accessor2<T> {
    pub fn new(initial_value: T) -> Self {
        Self {
            variable: initial_value,
        }
    }

    pub fn get(&self) -> &T {
        &(self.variable)
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut (self.variable)
    }

}

pub struct ArenaId<T>{
    id: u8,
    phantom: PhantomData<T>
}

pub struct Arena<T, const CAPACITY: usize> {
    data_base: Vec<Accessor2<T>, CAPACITY>,
}

impl<T, const SIZE: usize> Arena<T, SIZE> {
    pub const fn new() -> Self {
        Self {
            data_base: Vec::new(),
        }
    }

    /// Allocates a new accessor; returns None if data_base is out of capacity.
    pub fn alloc(&mut self, initial_value: T) -> Option<ArenaId<T>> {
        let new_accessor = Accessor2::new(initial_value);
        let index = self.data_base.len();
        let handler = usize_to_u8_clamper(index);
        let result = self.data_base.push(new_accessor);
        match result {
            Ok(_) => Some(ArenaId{ id: handler, phantom: PhantomData }),
            Err(_) => None,
        }
    }

    pub fn get_mut(&mut self, handler: ArenaId<T>) -> &mut Accessor2<T> {
        self.data_base.get_mut(handler.id as usize).unwrap()
    }

    pub fn get(&self, handler: ArenaId<T>) -> &Accessor2<T> {
        self.data_base.get(handler.id as usize).unwrap()
    }

}




