#![no_std]
#![allow(dead_code)]

pub mod protocol;
pub mod types;
pub mod mock;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}