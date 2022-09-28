#![no_std]
#![allow(dead_code)]


pub mod protocol;
pub mod types;
pub mod mock;


#[allow(unused_imports)]

#[cfg(test)]
mod tests {

   
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}