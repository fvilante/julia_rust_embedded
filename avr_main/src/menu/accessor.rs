//abstracts the access to a type in memory


use core::marker::PhantomData;
use core::cell::Cell;

use heapless::Vec;
use lib_1::arena::arena::{Arena, ArenaId};
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

    pub fn from_accessor_controler<'b: 'a,const SIZE: usize>(controler: &'b mut Arena<T, SIZE>, handler: ArenaId<T>) -> Accessor<'a,T> {
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




