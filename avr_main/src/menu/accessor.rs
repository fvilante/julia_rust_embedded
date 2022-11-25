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

    pub fn from_accessor_controler<const SIZE: usize>(controler: &'static mut Accessor2Controler<T, SIZE>, handler: Accessor2Handler<T>) -> Self {
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

pub struct Accessor2Handler<T>{
    id: u8,
    phantom: PhantomData<T>
}

pub struct Accessor2Controler<T, const CAPACITY: usize> {
    data_base: Vec<Accessor2<T>, CAPACITY>,
}

impl<T, const SIZE: usize> Accessor2Controler<T, SIZE> {
    pub const fn new() -> Self {
        Self {
            data_base: Vec::new(),
        }
    }

    /// Allocates a new accessor; returns None if data_base is out of capacity.
    pub fn new_accessor(&mut self, initial_value: T) -> Option<Accessor2Handler<T>> {
        let new_accessor = Accessor2::new(initial_value);
        let index = self.data_base.len();
        let handler = usize_to_u8_clamper(index);
        let result = self.data_base.push(new_accessor);
        match result {
            Ok(_) => Some(Accessor2Handler{ id: handler, phantom: PhantomData }),
            Err(_) => None,
        }
    }

    pub fn get_mut(&mut self, handler: Accessor2Handler<T>) -> &mut Accessor2<T> {
        self.data_base.get_mut(handler.id as usize).unwrap()
    }

    pub fn get(&self, handler: Accessor2Handler<T>) -> &Accessor2<T> {
        self.data_base.get(handler.id as usize).unwrap()
    }

}




