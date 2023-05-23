pub mod adapter;
mod driver;
pub mod interface;
//

/// TODO: Remove the exposition of the driver, use the adapter instead.
pub use driver::{clear, lcd_initialize, print, print_u8, set_cursor, NUMBER_OF_LINES};
