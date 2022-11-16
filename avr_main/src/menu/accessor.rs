//abstracts the access to a type in memory

type Setter<A> = fn(A);
type Getter<A> = fn() -> A;


pub trait AccessorTrait<T> {
    fn get(&self) -> T;
    fn set(&mut self, value: T);
}

pub struct Accessor<T> { // size = 4 bytes
    setter: Setter<T>, 
    getter: Getter<T>,
}

impl<T> Accessor<T> {
    pub fn new(setter: Setter<T>, getter: Getter<T>) -> Self {
        Self {
            setter,
            getter,
        }
    }

    pub fn clone(&self) -> Self {
        let setter = self.setter;
        let getter = self.getter;
        Self::new(setter, getter)
    }
}

impl<T> AccessorTrait<T> for Accessor<T> {

    fn set(&mut self, value: T) {
        (self.setter)(value);
    }

    fn get(&self) -> T {
        (self.getter)()
    }

}


