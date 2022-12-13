extern crate alloc;
use alloc::boxed::Box;

pub struct Profunctor<'a, A, B> {
    f: Box<dyn FnMut(A) -> B + 'a>,
}

impl<'a, A, B> Profunctor<'a, A, B> {
    fn new(f: impl FnMut(A) -> B + 'a) -> Self {
        Self { f: Box::new(f) }
    }

    fn unwrap(&mut self, data: A) -> B {
        (self.f)(data)
    }

    fn map<C>(&mut self, mut f: impl FnMut(B) -> C + 'a) -> Profunctor<A, C> {
        let g = move |a: A| {
            let b = self.unwrap(a);
            let c = f(b);
            c
        };
        Profunctor::new(g)
    }

    fn contra_map<A0>(&mut self, mut f: impl FnMut(A0) -> A + 'a) -> Profunctor<A0, B> {
        let g = move |a0: A0| {
            let a = f(a0);
            let b = self.unwrap(a);
            b
        };
        Profunctor::new(g)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_maps() {
        let z = 0;
        let a = Profunctor::new(|k: i32| 12 + z + k)
            .map(|b| b + 10)
            .unwrap(0);
        assert_eq!(a, 22);
    }

    #[test]
    fn it_contra_maps() {
        let z = 0;
        let a = Profunctor::new(|k: i32| 12 + z + k)
            .contra_map(|env: i32| env + 1)
            .unwrap(1_i32);
        assert_eq!(a, 14_i32);
    }
}
