//abstracts the access to a type in memory

type Setter<A> = fn(A);
type Getter<A> = fn() -> A;

//
// pub trait AccessorTrait<T> {
//     fn get(&self) -> T;
//     fn set(&mut self, value: T);
// }


enum Kind<T: Copy + 'static> {
    SetterGetter(Setter<T>, Getter<T>),
    Variable(&'static mut T),
}


pub struct Accessor<T: Copy + 'static> { // size = 4 bytes
    kind: Kind<T>
}

impl<T: Copy + 'static> Accessor<T> {
    pub fn new(setter: Setter<T>, getter: Getter<T>) -> Self {
        Self {
            kind: Kind::SetterGetter(setter, getter),
        }
    }

    pub fn from_variable(variable: &'static mut T) -> Self {
        Self {
            kind: Kind::Variable(variable)
        }
    }

    //pub fn clone(&self) -> Self {
    //    match self.kind {
    //        Kind::SetterGetter(setter, getter) => Self::new(setter, getter),
    //        Kind::Variable(variable) => Self::from_variable(variable),
    //    }        
    //}
}

impl<T: Copy + 'static> /*AccessorTrait<T> for*/ Accessor<T> {

    pub fn set(&mut self, value: T) {
        match &mut self.kind {
            Kind::SetterGetter(setter, _) => { setter(value); },
            Kind::Variable(variable) => {
                unsafe {
                    **variable = value;
                }
            }
        }
    }

    pub fn get(&self) -> T {
        match &self.kind {
            Kind::SetterGetter(_, getter) => { getter() },
            Kind::Variable(variable) => {
                unsafe {
                    **variable
                }
            }
        }
    }

}


