/// This module is made to represent a numerical widget
use core::cell::Cell;
use core::ops::Range;
use core::str::{CharIndices, Chars, FromStr};

use cross_platform::utils::numerical::{convert_u16_to_string_decimal, usize_to_u8_clamper};
use heapless::String;

use super::super::widget::Saveble;
use super::super::{widget::Editable, widget::Widget};
use crate::geometry::point::Point;
use crate::microcontroler::ratangular_wave::RectangularWave;
use crate::{board::keypad::KeyCode, menu::screen_buffer::ScreenBuffer};

use cross_platform::utils::cursor::Cursor;

/// Represents a variable length  data, in memory, though a sequence of characters (ie: numbers, texts).
/// This type is a wrapper over the [`FieldBuffer`] type
struct Content {
    /// A string buffer with static capacity defined and stack allocated
    /// NOTE: The `String` type here is not from the standard lib but from a lib named `heapless`.
    data: String<{ Self::SIZE }>,
}

impl Content {
    /// Sets the max size of the [`Content`] type
    /// MAX_NUMBER_OF_CHARS_IN_BUFFER
    /// NOTE: Do not choose a number less than 5 else you cannot represent u16 data types in
    /// its decimal representation (ie: 65535)
    const SIZE: usize = 6;

    const fn from_raw(data: String<{ Self::SIZE }>) -> Self {
        Self { data }
    }

    /// New empty content
    const fn new() -> Self {
        Self::from_raw(String::new())
    }

    /// Constructs from [`&str`] returns None if str is greater than buffer capacity (see: [`MAX_NUMBER_OF_CHARS_IN_BUFFER`])
    fn from_str(s: &str) -> Option<Self> {
        if let Ok(data) = String::from_str(s) {
            Some(Self::from_raw(data))
        } else {
            None
        }
    }

    /// Converts an [`u16`] to a [`Content`] and format the resulting number to the specified `number_of_digits`
    /// TODO: Use `number_of_digits` to represent the max digits you want for your u16 representation
    /// If the `u16` value is greater than max size that `number_of_digits` can contain, than
    /// the u16 will be clamped silently.
    fn from_u16_formated(data: u16, number_of_digits: u8) -> Content {
        const BLACKET_CHAR: char = '0';
        let s = convert_u16_to_string_decimal(data);
        let base = Content::from_str(s.as_str()).unwrap();
        let mut temp = Content::new();
        //leading zeros
        let len = base.len();
        for _ in len..number_of_digits {
            temp.push(BLACKET_CHAR);
        }
        //actal number
        for char in base.chars() {
            temp.push(char);
        }
        temp
    }

    /// Converts a [`Content`] that is supposed to contains an number into an [`u16`] value.
    /// If [`Content`] does not contains a number or if the convertion is not possible returns zero
    fn to_u16(&self) -> u16 {
        self.parse::<u16>().unwrap_or(0)
    }

    // ============== [ wrapping over heapless::String methods ] ================

    /// Returns lenght of Content. It must be less or equal to [`MAX_NUMBER_OF_CHARS_IN_BUFFER`]
    pub fn len(&self) -> u8 {
        self.data.len() as u8
    }

    pub fn char_indices(&self) -> CharIndices {
        self.data.char_indices()
    }

    pub fn chars(&self) -> Chars<'_> {
        self.data.chars()
    }

    pub fn clone(&self) -> Self {
        Self::from_raw(self.data.clone())
    }

    pub fn parse<F: FromStr>(&self) -> Option<F> {
        self.data.parse::<F>().ok()
    }

    pub fn push(&mut self, c: char) -> Option<()> {
        self.data.push(c).ok()
    }
}

/// A constant sized string that represents some data content being edited by a navigating cursor.
/// This type is agnostic to the type of data being edited, that means you can use this type to edit numbers and even text
struct InputEditor {
    /// String content being edited.
    content: Content,
    /// Tracks current cursor position.
    cursor: Cursor,
}

impl InputEditor {
    pub fn new(initial_content: Content, initial_cursor_position: u8) -> Self {
        let start = 0;
        let end = initial_content.len();
        Self {
            cursor: Cursor::new(start, end, initial_cursor_position),
            content: initial_content,
        }
    }

    /// Changes the character in place of the current cursor position to given `new_char`
    pub fn change_cursor_item_to(&mut self, new_char: char) {
        let current_cursor = self.cursor.get_current();
        let mut s = Content::new();
        for (index, current_char) in self.content.char_indices() {
            if index == current_cursor as usize {
                s.push(new_char).unwrap();
            } else {
                s.push(current_char).unwrap();
            }
        }
        self.content = s.clone();
    }

    /// Move cursor to a single character to the right
    pub fn move_cursor_right(&mut self) {
        self.cursor.next();
    }

    /// Move cursor to a single character to the left
    pub fn move_cursor_left(&mut self) {
        self.cursor.previous();
    }

    /// Move cursor to the most left-sided character of the [`Content`]
    pub fn move_cursor_begin(&mut self) {
        self.cursor.begin();
    }

    /// Move cursor to the most right-sided character of the [`Content`]
    pub fn move_cursor_end(&mut self) {
        self.cursor.end();
    }

    /// Adds an given character in place of current cursor position and increment cursor once to the right
    pub fn addAndMoveRight(&mut self, item: char) {
        self.change_cursor_item_to(item);
        self.move_cursor_right()
    }
}

/// Format parameters for the [`NumberInputEditor`] type. Wrapper around the main parameters of the [`NumberInputEditor`]
#[derive(Copy, Clone)]
pub struct Format {
    /// Initial admissible value (included)
    pub start: u16,
    /// Final admissible value (excluded)
    pub end: u16,
    /// In what digit cursor should start in. (from left to right, starting at 0 and going until Self::get_number_of_digits)
    /// The total number of digits is based in the number of digits necessary to represent the `Format::end` value
    pub initial_cursor_position: u8,
}

impl Format {
    /// Given a valid_range.END calculates the least amount of digits necessary to represent it in decimal as a string
    pub fn get_number_of_digits(&self) -> u8 {
        // TODO: When possible this code may be refactored to use pure math
        let max = self.end;
        let s = convert_u16_to_string_decimal(max);
        usize_to_u8_clamper(s.len())
    }

    pub fn get_valid_range(&self) -> Range<u16> {
        self.start..self.end
    }
}

/// Just a decorator around [`InputEditor`] for deal with formated numbers
struct NumberInputEditor {
    content_editor: InputEditor,
    format: Format,
}

impl NumberInputEditor {
    /// Private constructor. NOTE: Use method [`Self::from_u16`] instead
    fn new(initial_content: Content, format: Format) -> Self {
        Self {
            content_editor: InputEditor::new(initial_content, format.initial_cursor_position),
            format,
        }
    }

    /// Constructs an `Self` from a given [`u16`] value and a given `Format`.
    ///
    /// Use `number_of_digits` to represent the max digits you want for your u16 representation
    /// If the `u16` value is greater than max size that `number_of_digits` can contain, than than
    /// the u16 will be clamped silently.
    pub fn from_u16(initial_value: u16, format: Format) -> Self {
        let initial_content =
            Content::from_u16_formated(initial_value, format.get_number_of_digits());
        Self::new(initial_content, format)
    }

    /// Sets u16 value. If no format is given, it uses current format to set the value
    pub fn set_u16(&mut self, value: u16, format: Option<Format>) {
        let format: Format = match format {
            Some(format) => format,
            None => self.format,
        };
        let content = Content::from_u16_formated(value, format.get_number_of_digits());
        self.content_editor = InputEditor::new(content, format.initial_cursor_position);
    }

    /// Copy edited value to its [`u16`] representation assuring the value is clamped to `self.format.valid_range`
    pub fn as_u16_clamped(&self) -> u16 {
        let current_edited_value = Content::to_u16(&self.content_editor.content);
        let min = self.format.get_valid_range().start;
        let max = self.format.get_valid_range().end;
        let clamped_value = current_edited_value.clamp(min, max);
        clamped_value
    }

    pub fn get_current_cursor_index(&self) -> u8 {
        self.content_editor.cursor.get_current()
    }
    /// Reset cursor to its default initial position
    /// NOTE: This method is not necessary the same as begin() method
    /// TODO: Remove the necessity of this method being avoiding make the Self alive when no edition is hapenning
    /// by the user (for example when the info is just being show in screen without any edition mode)
    pub fn reset_cursor(&mut self, initial_cursor_position: u8) {
        let cursor = &mut self.content_editor.cursor;
        cursor.set_current(initial_cursor_position);
    }

    /// An iterator over the char's of a [`NumberInputEditor`], and their positions.
    pub fn char_indices(&self) -> CharIndices {
        self.content_editor.content.char_indices()
    }

    pub fn get_format(&self) -> Format {
        self.format.clone()
    }
}

/// This [`Widget`] manages the edition of an number (unsigned integer) by the user using the keyboard and lcd display
pub struct NumberInputEditorWidget<'a> {
    number_editor: NumberInputEditor,
    /// This class is responsible to generate the blinking character effect
    blink: RectangularWave,
    is_in_edit_mode_: bool,
    variable: &'a Cell<u16>,
}

impl<'a> NumberInputEditorWidget<'a> {
    pub fn new(variable: &'a Cell<u16>, format: Format, is_in_edit_mode_: bool) -> Self {
        const T_ON: u16 = 600;
        const T_OFF: u16 = 300;
        let initial_value = variable.get();
        Self {
            number_editor: NumberInputEditor::from_u16(initial_value, format.clone()),
            blink: RectangularWave::new(T_ON, T_OFF),
            is_in_edit_mode_,
            variable,
        }
    }
}

impl Saveble for NumberInputEditorWidget<'_> {
    /// Restore initial value, discard edited value
    fn restore_value(&mut self) {
        let initial_value = self.variable.get();
        self.number_editor.set_u16(initial_value, None);
    }

    /// Save value being edited
    fn save_value(&mut self) {
        let edited_value = self.number_editor.as_u16_clamped();
        self.variable.set(edited_value);
    }
}

impl Editable for NumberInputEditorWidget<'_> {
    fn set_edit_mode(&mut self, value: bool) {
        self.is_in_edit_mode_ = value
    }

    fn is_in_edit_mode(&self) -> bool {
        self.is_in_edit_mode_
    }
}

impl Widget for NumberInputEditorWidget<'_> {
    fn send_key(&mut self, key: KeyCode) {
        if self.is_in_edit_mode() {
            let content_editor = &mut self.number_editor.content_editor;
            match key {
                // navigation_key left and right
                KeyCode::KEY_SETA_BRANCA_ESQUERDA => {
                    content_editor.move_cursor_left();
                }
                KeyCode::KEY_SETA_BRANCA_DIREITA => {
                    content_editor.move_cursor_right();
                }
                KeyCode::KEY_DIRECIONAL_PARA_DIREITA => {
                    content_editor.move_cursor_right();
                }
                KeyCode::KEY_DIRECIONAL_PARA_ESQUERDA => {
                    content_editor.move_cursor_left();
                }
                // edition keys
                KeyCode::KEY_0 => {
                    content_editor.addAndMoveRight('0');
                }
                KeyCode::KEY_1 => {
                    content_editor.addAndMoveRight('1');
                }
                KeyCode::KEY_2 => {
                    content_editor.addAndMoveRight('2');
                }
                KeyCode::KEY_3 => {
                    content_editor.addAndMoveRight('3');
                }
                KeyCode::KEY_4 => {
                    content_editor.addAndMoveRight('4');
                }
                KeyCode::KEY_5 => {
                    content_editor.addAndMoveRight('5');
                }
                KeyCode::KEY_6 => {
                    content_editor.addAndMoveRight('6');
                }
                KeyCode::KEY_7 => {
                    content_editor.addAndMoveRight('7');
                }
                KeyCode::KEY_8 => {
                    content_editor.addAndMoveRight('8');
                }
                KeyCode::KEY_9 => {
                    content_editor.addAndMoveRight('9');
                }
                //everything else -> do nothing
                _ => {}
            }
        } else {
            // ignore keys -> do nothing
        }
    }

    fn update(&mut self) {
        self.blink.update(); // blinks cursor
    }

    fn draw(&self, screen_buffer: &mut ScreenBuffer, start_point: Point) {
        screen_buffer.set_cursor(start_point);
        for (position, digit) in self.number_editor.char_indices() {
            const BLINK_CHAR: char = '_';
            let mut current_char = digit;
            let is_current_char_over_cursor =
                position == self.number_editor.get_current_cursor_index() as usize;
            let is_time_to_blink = !self.blink.read();
            if is_current_char_over_cursor && is_time_to_blink && self.is_in_edit_mode() {
                current_char = BLINK_CHAR;
            }
            screen_buffer.print_char(current_char);
        }
    }
}
