//abstracts the access to a type in memory

pub struct Accessor<'a, T: Clone> {
    ref_mut: &'a mut T,
}

impl<'a, T: Clone> Accessor<'a, T> {

    pub fn new(ref_mut: &'a mut T) -> Self {
        Self {
            ref_mut,
        }
    }

    pub fn set(&mut self, value: T) {
        *self.ref_mut = value;
    }

    pub fn get(&self) -> T {
        (*self.ref_mut).clone()
    }

}