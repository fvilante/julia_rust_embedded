//abstracts the access to a type in memory

type Setter<A> = fn(A);
type Getter<A> = fn() -> A;

pub struct Accessor<T> {
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

    pub fn set(&mut self, value: T) {
        (self.setter)(value);
    }

    pub fn get(&self) -> T {
        (self.getter)()
    }

}