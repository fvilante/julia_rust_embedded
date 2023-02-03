//abstracts the access to a type in memory

/// TODO: This type probabbly will be removed in future, avoid to use it and use Cell<T> or RefCell<T> instead, if possible.
pub struct Accessor<'a, T: Copy + 'a> {
    // size = 4 bytes
    variable: &'a mut T,
}

impl<'a, T: Copy + 'a> Accessor<'a, T> {
    pub fn new(variable: &'a mut T) -> Self {
        Self { variable }
    }

    //pub fn from_accessor_controler<const SIZE: usize>(controler: &'a mut Arena<T, SIZE>, handler: ArenaId<T>) -> Accessor<'a,T> {
    //    let accessor = (*controler).borrow_mut(handler);
    //    Self::new(accessor)
    //}
}

impl<'a, T: Copy + 'a> Accessor<'a, T> {
    pub fn get(&self) -> &T {
        &self.variable
    }

    pub fn get_mut(&mut self) -> &mut T {
        self.variable
    }
}
