#![no_std]
#![allow(dead_code)]
#![feature(result_flattening)]


//  Memory allocation for embbeded systems 
//  For more see: 
//      https://docs.rust-embedded.org/book/collections/
//      https://docs.rs/alloc-shim/latest/alloc/
#[cfg(all(feature = "alloc", not(feature = "std")))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std as alloc;


pub mod protocol;
pub mod types;
pub mod mock;
pub mod utils;
pub mod adt;
//pub mod alloc;


#[allow(unused_imports)]

#[cfg(test)]
mod tests {

   
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}