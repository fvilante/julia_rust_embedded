use core::{cell::Cell, ops::Range};

use cross_platform::utils::cursor::Cursor;

use crate::string::flash::FlashString;

use super::{
    field::Field,
    numerical::Format,
    optional::{make_options_buffer_from_array, OptionsBuffer},
};

//////////////////////////////////////////////////////////////////////////////////

pub struct NumericalFieldBuilder {
    variable: *mut u16,
    valid_range: Option<Range<u16>>,
    initial_cursor_position: Option<u8>,
}

impl NumericalFieldBuilder {
    /// Private constructor
    fn new(variable: *mut u16) -> Self {
        Self {
            variable,
            valid_range: None,
            initial_cursor_position: None,
        }
    }

    fn make_format(&self) -> Format {
        const DEFAULT_START_END: (u16, u16) = (0, 0xFFFF);
        const DEFAULT_INITIAL_CURSOR_POSITION: u8 = 0;

        let (start, end) = match &self.valid_range {
            Some(range) => (range.start, range.end),
            None => DEFAULT_START_END,
        };

        let initial_cursor_position = match self.initial_cursor_position {
            Some(value) => value,
            None => DEFAULT_INITIAL_CURSOR_POSITION,
        };

        Format {
            start,
            end,
            initial_cursor_position,
        }
    }

    /// Sets the valid range
    pub fn valid_range(&mut self, range: Range<u16>) -> &mut Self {
        self.valid_range = Some(range);
        self
    }

    /// Sets initial cursor position
    pub fn cursor_position(&mut self, position: u8) -> &mut Self {
        self.initial_cursor_position = Some(position);
        self
    }

    /// Builds the Field
    pub fn build(&mut self) -> Field {
        let format = self.make_format();
        let variable = self.variable;
        Field::from_numerical(variable, format)
    }
}

//////////////////////////////////////////////////////////////////////////////////

pub struct OptionalFieldBuilder<const SIZE: usize> {
    variable: *mut Cursor,
    list: [FlashString; SIZE],
}

impl<const SIZE: usize> OptionalFieldBuilder<SIZE> {
    fn new(variable: *mut Cursor, list: [FlashString; SIZE]) -> Self {
        Self { variable, list }
    }

    pub fn build(&self) -> Field {
        let (list, variable) = (self.list, self.variable);
        let __options: OptionsBuffer = make_options_buffer_from_array(list);
        Field::from_optional(variable, __options)
    }
}

//////////////////////////////////////////////////////////////////////////////////

pub struct FieldBuilder;

impl FieldBuilder {
    pub fn numerical(variable: *mut u16) -> NumericalFieldBuilder {
        NumericalFieldBuilder::new(variable)
    }

    /// TODO: Make Cursor and the List coupled to avoid Cursor.Max and List.Len diverge from each other
    pub fn optional<const SIZE: usize>(
        variable: *mut Cursor,
        list: [FlashString; SIZE],
    ) -> OptionalFieldBuilder<SIZE> {
        OptionalFieldBuilder::new(variable, list)
    }
}
