//abstracts the access to a type in memory

type Setter<A> = fn(A);
type Getter<A> = fn() -> A;


pub struct Accessor<T: Copy + 'static> { // size = 4 bytes
    variable: &'static mut T
}

impl<T: Copy + 'static> Accessor<T> {

    pub fn new(variable: &'static mut T) -> Self {
        Self {
            variable,
        }
    }

}

impl<T: Copy + 'static> /*AccessorTrait<T> for*/ Accessor<T> {

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


