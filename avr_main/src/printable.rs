//! Anything that implements [`IntoIterator<Item = u8>`] is considered printable for this system that
//! targets anything compatible with the conventional Hitachi LCD displays.
//!

pub trait Printable: IntoIterator<Item = u8> {}

impl<T: IntoIterator<Item = u8>> Printable for T {}
