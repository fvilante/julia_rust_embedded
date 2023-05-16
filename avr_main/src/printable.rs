//! Anything that can be condiered printable for this system.

pub trait Printable: IntoIterator<Item = u8> {}

/// Any type that implements [`IntoIterator<Item = u8>`] is considered automatically [`Printable`].
impl<T: IntoIterator<Item = u8>> Printable for T {}

struct Printer {}
