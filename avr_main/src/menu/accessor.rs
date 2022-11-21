//abstracts the access to a type in memory

type Setter<A> = fn(A);
type Getter<A> = fn() -> A;


pub struct Accessor<'a,T: Copy + 'a> { // size = 4 bytes
    variable: &'a mut T
}

impl<'a,T: Copy + 'a> Accessor<'a,T> {

    pub fn new(variable: &'a mut T) -> Self {
        Self {
            variable,
        }
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


