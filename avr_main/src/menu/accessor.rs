//abstracts the access to a type in memory

type Setter<A> = fn(A);
type Getter<A> = fn() -> A;


pub enum AccessorEnum {
    U16(Accessor<u16>),
}

impl AccessorEnum {
    pub fn from_u16(setter: Setter<u16>, getter: Getter<u16>) -> Accessor<u16> {
        Accessor::new(setter, getter)
    }
}

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

    ///TODO: this mutable self is really necessary?
    pub fn set(&mut self, value: T) {
        (self.setter)(value);
    }

    pub fn get(&self) -> T {
        (self.getter)()
    }

}