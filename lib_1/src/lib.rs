#![no_std]
#![allow(dead_code)]
#![feature(result_flattening)]


pub mod protocol;
pub mod types;
pub mod mock;
pub mod utils;


#[allow(unused_imports)]

#[cfg(test)]
mod tests {

   
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}