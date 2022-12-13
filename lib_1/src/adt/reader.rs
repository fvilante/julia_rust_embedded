extern crate alloc;
use alloc::boxed::Box;

pub struct Reader<'a, A> {
    f: Box<dyn FnMut(A) -> () + 'a>,
}

impl<'a, A> Reader<'a, A> {
    pub fn new(f: impl FnMut(A) -> () + 'a) -> Self {
        Self { f: Box::new(f) }
    }

    pub fn unwrap(&mut self, data: A) -> () {
        (self.f)(data)
    }

    pub fn contra_map<A0>(&mut self, mut f: impl FnMut(A0) -> A + 'a) -> Reader<A0> {
        let g = move |a0: A0| {
            let a = f(a0);
            self.unwrap(a);
        };
        Reader::new(g)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    static mut STATE: u8 = 0;

    #[test]
    fn it_contra_maps() {
        let z = 0;
        let mut r0 = Reader::new(|k: u8| unsafe { STATE = 12 + z + k });
        let mut r1 = r0.contra_map(|u: i32| u.try_into().unwrap_or(66_u8));
        r1.unwrap(10);
        unsafe {
            assert_eq!(STATE, 22_u8);
        }
    }
}
