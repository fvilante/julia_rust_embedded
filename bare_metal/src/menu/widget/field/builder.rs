use core::{cell::Cell, ops::Range};

use lib_1::utils::cursor::Cursor;

use crate::string::flash::FlashString;

use super::{
    field::Field,
    numerical::Format,
    optional::{make_options_buffer_from_array, OptionsBuffer},
};

//////////////////////////////////////////////////////////////////////////////////

pub struct NumericalFieldBuilder<'a> {
    variable: &'a Cell<u16>,
    valid_range: Option<Range<u16>>,
    initial_cursor_position: Option<u8>,
}

impl<'a> NumericalFieldBuilder<'a> {
    /// Private constructor
    fn new(variable: &'a Cell<u16>) -> Self {
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
    pub fn build(&mut self) -> Field<'a> {
        let format = self.make_format();
        let variable = self.variable;
        Field::from_numerical(variable, format)
    }
}

//////////////////////////////////////////////////////////////////////////////////

pub struct OptionalFieldBuilder<'a, const SIZE: usize> {
    variable: &'a Cell<Cursor>,
    list: [FlashString; SIZE],
}

impl<'a, const SIZE: usize> OptionalFieldBuilder<'a, SIZE> {
    fn new(variable: &'a Cell<Cursor>, list: [FlashString; SIZE]) -> Self {
        Self { variable, list }
    }

    pub fn build(&self) -> Field<'a> {
        let (list, variable) = (self.list, self.variable);
        let __options: OptionsBuffer = make_options_buffer_from_array(list);
        Field::from_optional(variable, __options)
    }
}

//////////////////////////////////////////////////////////////////////////////////

pub struct FieldBuilder;

impl FieldBuilder {
    pub fn numerical<'a>(variable: &'a Cell<u16>) -> NumericalFieldBuilder<'a> {
        NumericalFieldBuilder::new(variable)
    }

    /// TODO: Make Cursor and the List coupled to avoid Cursor.Max and List.Len diverge from each other
    pub fn optional<'a, const SIZE: usize>(
        variable: &'a Cell<Cursor>,
        list: [FlashString; SIZE],
    ) -> OptionalFieldBuilder<'a, SIZE> {
        OptionalFieldBuilder::new(variable, list)
    }
}
