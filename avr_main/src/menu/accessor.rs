//abstracts the access to a type in memory


use core::marker::PhantomData;
use core::cell::{Cell, RefMut};

use heapless::Vec;
use lib_1::arena::arena::{Arena, ArenaId};
use lib_1::utils::common::usize_to_u8_clamper;

pub struct Accessor<'a,T: Copy + 'a> { // size = 4 bytes
    variable: &'a mut T,
}

impl<'a,T: Copy + 'a> Accessor<'a,T> {

    pub fn new(variable: &'a mut T,) -> Self {
        Self {
            variable,
        }
    }

    //pub fn from_accessor_controler<const SIZE: usize>(controler: &'a mut Arena<T, SIZE>, handler: ArenaId<T>) -> Accessor<'a,T> {
    //    let accessor = (*controler).borrow_mut(handler);
    //    Self::new(accessor)
    //}

}

impl<'a, T: Copy + 'a> Accessor<'a,T> {

    pub fn get(&self) -> &T {
        &self.variable
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.variable
    }

}




