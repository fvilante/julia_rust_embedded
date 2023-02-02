use super::hepers::LcdLine;

use lib_1::utils::{
    common::{configure_bit, get_bit_at, reset_bit_at},
    cursor::Cursor,
};

/// Helper to optimize memory space consuption. The ideia is to store 2 numbers inside a single byte.
struct DecompressedNumbers(u8, u8);
const N2_BIT_POSITION: u8 = 7;

/// Encode two number using this criteria:
/// bits 0..6 represents first number, and bit 7 represents the second number
struct CompressedNumbers(u8);

impl TryFrom<DecompressedNumbers> for CompressedNumbers {
    type Error = ();

    fn try_from(value: DecompressedNumbers) -> Result<Self, Self::Error> {
        let DecompressedNumbers(n1, n2) = value;

        match (n1, n2) {
            // Valid range given `encoding` criteria
            (0..128, 0..2) => {
                let n1_compressed = n1;
                let n2_compressed = if n2 == 0 {
                    configure_bit(0, N2_BIT_POSITION, false)
                } else {
                    configure_bit(0, N2_BIT_POSITION, true)
                };
                let compressed = n1_compressed + n2_compressed;
                Result::Ok(CompressedNumbers(compressed))
            }

            // Invalid range
            _ => Result::Err(()),
        }
    }
}

impl From<CompressedNumbers> for DecompressedNumbers {
    fn from(value: CompressedNumbers) -> Self {
        let CompressedNumbers(x) = value;
        let n1 = reset_bit_at(x, N2_BIT_POSITION);
        let n2 = get_bit_at(x, N2_BIT_POSITION);
        DecompressedNumbers(n1, n2)
    }
}

/// Controls the state of the navigation on sub menu, which is what is the selected line in the list of items.
///
/// TODO: The memory footprint size this struct may be optimized going from 6 bytes to at least 3 bytes if I made a custom Cursor
/// because `Cursor::Start` is always zero, and `Cursor:End` of `lcd_line_cursor` is always 2 or const statically defined.  
#[derive(Copy, Clone)]
pub struct NavigationState {
    /// Uncompressed representation of NavigationState
    /// Controls the line of menu (see: LcdLine) which is current selected.
    lcd_line_cursor: Cursor,
    // First line to render in the lcd screen in relation to the list of menu items
    first_line_to_render: Cursor,
}

impl NavigationState {
    pub fn new_from_submenu_len(number_of_menu_items: u8) -> Self {
        /// This application uses a LCD 40 collumns by 2 Lines in future this may be generalized
        const TOTAL_NUMBER_OF_LINES_IN_LCD: u8 = 2;
        Self {
            lcd_line_cursor: {
                const DEFAULT_INITIAL_LINE_SELECTED: u8 = 0;
                Cursor::new(
                    0,
                    TOTAL_NUMBER_OF_LINES_IN_LCD,
                    DEFAULT_INITIAL_LINE_SELECTED,
                )
            },
            first_line_to_render: {
                let default_initial_menu_item = 0;
                Cursor::from_range(
                    0..number_of_menu_items as usize - (TOTAL_NUMBER_OF_LINES_IN_LCD - 1) as usize,
                    default_initial_menu_item,
                )
            },
        }
    }

    /// Scrolls menu down, return true if it the scrooll has already been exausted
    pub fn scroll_down(&mut self) -> bool {
        let has_exhausted = self.first_line_to_render.next();
        has_exhausted
    }

    /// Scrolls menu up, return true if it the scrooll has already been exausted
    pub fn scroll_up(&mut self) -> bool {
        let has_exhausted = self.first_line_to_render.previous();
        has_exhausted
    }

    pub fn key_down(&mut self) {
        let is_exhasted = self.lcd_line_cursor.next();
        if is_exhasted {
            self.scroll_down();
        };
    }

    pub fn key_up(&mut self) {
        let is_exhasted = self.lcd_line_cursor.previous();
        if is_exhasted {
            self.scroll_up();
        };
    }

    pub fn get_current_lcd_line(&self) -> LcdLine {
        LcdLine::from(self.lcd_line_cursor.get_current())
    }

    /// Finds for a given LcdLine the index of the item currently selected (in respect to menu list)
    pub fn get_current_index_for(&self, line: LcdLine) -> u8 {
        let lcd_line = line as u8;
        let first_line_to_render = self.first_line_to_render.get_current();
        let item_index = lcd_line + first_line_to_render;
        item_index
    }
}
