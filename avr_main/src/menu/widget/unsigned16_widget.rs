use core::ops::Range;
use core::str::{CharIndices, Chars, FromStr};

use alloc::borrow::ToOwned;
use heapless::{String, Vec};
use lib_1::utils::common::{convert_u16_to_string_decimal, usize_to_u8_clamper};

use crate::board::lcd;
use crate::{
    board::keyboard::KeyCode,
    menu::{accessor::Accessor, canvas::Canvas, point::Point, ratangular_wave::RectangularWave},
};

use super::optional::{OptionalEditableWidget, OptionsBuffer};
use super::{edit_mode::EditMode, widget::Editable, widget::Widget};
use lib_1::utils::cursor::Cursor;

/// Sets the max size of the [`Content`] type
///
/// NOTE: Do not choose a number less than 5 else you cannot represent u16 data types in its decimal representation (ie: 65535)
const MAX_NUMBER_OF_CHARS_IN_BUFFER: usize = 6;

/// A string buffer with static capacity defined and stack allocated
pub(super) type StringBuffer = String<MAX_NUMBER_OF_CHARS_IN_BUFFER>;

/// Represents a variable length  data, in memory, though a sequence of characters (ie: numbers, texts).
///
/// This type is a wrapper over the [`StringBuffer`] type
pub struct Content {
    data: StringBuffer,
}

impl Content {
    fn from_raw(data: StringBuffer) -> Self {
        Self { data }
    }

    /// New empty content
    pub fn new() -> Self {
        Self::from_raw(StringBuffer::new())
    }

    /// Constructs from [`&str`] returns None if str is greater than buffer capacity (see: [`MAX_NUMBER_OF_CHARS_IN_BUFFER`])
    pub fn from_str(s: &str) -> Option<Self> {
        if let Ok(data) = StringBuffer::from_str(s) {
            Some(Self::from_raw(data))
        } else {
            None
        }
    }

    /// Converts an [`u16`] to a [`Content`] and format the resulting number to the specified `number_of_digits`
    ///
    /// TODO: Use `number_of_digits` to represent the max digits you want for your u16 representation
    /// If the `u16` value is greater than max size that `number_of_digits` can contain, than than
    /// the u16 will be clamped silently.
    fn from_u16_formated(data: u16, number_of_digits: u8) -> Content {
        const blacket_char: char = '0';
        let s = convert_u16_to_string_decimal(data);
        let mut base = Content::from_str(s.as_str()).unwrap();
        let mut temp = Content::new();
        //leading zeros
        let len = usize_to_u8_clamper(base.len());
        for _ in len..number_of_digits {
            temp.push(blacket_char);
        }
        //actal number
        for char in base.chars() {
            temp.push(char);
        }
        temp
    }

    /// Converts a [`Content`] that is supposed to contains an number into an [`u16`] value
    ///
    /// If [`Content`] does not contains a number or if the convertion is not possible returns zero
    fn to_u16(&self) -> u16 {
        self.parse::<u16>().unwrap_or(0)
    }

    // ============== [ wrapping over heapless::String methods ] ================

    /// Returns lenght of Content.
    ///
    /// It must be less or equal to [`MAX_NUMBER_OF_CHARS_IN_BUFFER`]
    pub fn len(&self) -> usize {
        self.data.len()
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
///
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
        let end = usize_to_u8_clamper(initial_content.len());
        Self {
            cursor: Cursor::new(start, end, initial_cursor_position),
            content: initial_content,
        }
    }

    /// Changes the character in place of the current cursor position to given `new_char`
    pub fn change_cursor_item_to(&mut self, new_char: char) -> &mut Self {
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
        self
    }

    /// Move cursor to a single character to the right
    pub fn move_cursor_right(&mut self) -> &mut Self {
        self.cursor.next();
        self
    }

    /// Move cursor to a single character to the left
    pub fn move_cursor_left(&mut self) -> &mut Self {
        self.cursor.previous();
        self
    }

    /// Move cursor to the most left-sided character of the [`Content`]
    pub fn move_cursor_begin(&mut self) -> &mut Self {
        self.cursor.begin();
        self
    }

    /// Move cursor to the most right-sided character of the [`Content`]
    pub fn move_cursor_end(&mut self) -> &mut Self {
        self.cursor.end();
        self
    }

    /// Adds an given character in place of current cursor position and increment cursor once to the right
    pub fn addAndMoveRight(&mut self, item: char) -> &mut Self {
        self.change_cursor_item_to(item).move_cursor_right()
    }
}

/// Format parameters for [`NumberInputEditor`]
///
/// Wrapper of the main parameters of the [`NumberInputEditor`]
///
/// TODO: Does not make sense to have a number of digits greater than the valid.range.end. Protect against this
/// condition
pub struct Format {
    pub valid_range: Range<u16>,
    pub initial_cursor_position: u8,
}

impl Format {
    /// Given a valid_range.END calculates the least amount of digits necessary to represent it in decimal as a string
    pub fn get_number_of_digits(&self) -> u8 {
        let max = self.valid_range.end;
        let s = convert_u16_to_string_decimal(max);
        usize_to_u8_clamper(s.len())
    }
}

impl Clone for Format {
    fn clone(&self) -> Self {
        Self {
            valid_range: self.valid_range.clone(),
            initial_cursor_position: self.initial_cursor_position.clone(),
        }
    }
}

/// Just decorator around [`InputEditor`] for deal with formated numbers
struct NumberInputEditor {
    content_editor: InputEditor,
    format: Format,
}

impl NumberInputEditor {
    /// Private constructor. NOTE: Use method `from_u16` instead
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

    ///TODO: Remove this method or make it unnecessary if possible
    pub fn set_u16(&mut self, value: u16, format: Format) {
        let content = Content::from_u16_formated(value, format.get_number_of_digits());
        self.content_editor = InputEditor::new(content, format.initial_cursor_position);
    }

    /// Copy edited value to its [`u16`] representation assuring the value is clamped to `self.format.valid_range`
    pub fn as_u16_clamped(&self) -> u16 {
        let current_edited_value = Content::to_u16(&self.content_editor.content);
        let min = self.format.valid_range.start;
        let max = self.format.valid_range.end;
        let clamped_value = current_edited_value.clamp(min, max);
        clamped_value
    }

    pub fn get_current_cursor_index(&self) -> u8 {
        self.content_editor.cursor.get_current()
    }
    /// Reset cursor to its default initial position
    ///
    /// NOTE: This method is not necessary the same as begin() method
    ///
    /// TODO: Remove the necessity of this method being avoiding make the Self alive when no edition is hapenning
    /// by the user (for example when the info is just being show in screen without any edition mode)
    pub fn reset_cursor(&mut self, initial_cursor_position: u8) {
        let cursor = &mut self.content_editor.cursor;
        cursor.set_current(initial_cursor_position);
    }

    /// An iterator over the [char]s of a [`Unsigned16Editor`], and their positions.
    pub fn char_indices(&self) -> CharIndices {
        self.content_editor.content.char_indices()
    }
}

/// This [`Widget`] manages the edition of an number (unsigned integer) by the user using the keyboard and lcd display
pub struct NumberInputEditorWidget {
    u16_editor: NumberInputEditor,
    /// This class is responsible to generate the blinking character effect
    blink: RectangularWave,
    edit_mode: EditMode,
}

impl NumberInputEditorWidget {
    pub fn new(initial_value: u16, format: Format, is_in_edit_mode: bool) -> Self {
        const T_ON: u64 = 600;
        const T_OFF: u64 = 300;
        Self {
            u16_editor: NumberInputEditor::from_u16(initial_value, format.clone()),
            blink: RectangularWave::new(T_ON, T_OFF),
            edit_mode: EditMode::new(is_in_edit_mode),
        }
    }
}

impl Editable for NumberInputEditorWidget {
    fn set_edit_mode(&mut self, value: bool) {
        self.edit_mode.set_edit_mode(value)
    }

    fn is_in_edit_mode(&self) -> bool {
        self.edit_mode.is_in_edit_mode()
    }
}

impl Widget for NumberInputEditorWidget {
    fn send_key(&mut self, key: KeyCode) {
        if self.is_in_edit_mode() {
            let content_editor = &mut self.u16_editor.content_editor;
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

    fn draw(&self, canvas: &mut Canvas, start_point: Point) {
        canvas.set_cursor(start_point);
        let is_in_edit_mode = self.is_in_edit_mode();
        for (position, digit) in self.u16_editor.char_indices() {
            const blink_char: char = '_';
            let mut current_char = digit;
            let is_current_char_over_cursor =
                position == self.u16_editor.get_current_cursor_index() as usize;
            let is_time_to_blink = self.blink.read();
            if is_current_char_over_cursor && is_time_to_blink && is_in_edit_mode {
                current_char = blink_char;
            }
            canvas.print_char(current_char);
        }
    }
}

/* impl NumberInputEditorWidget<'_> {
    pub fn save_edition(&mut self) {
        let edited_value = self.u16_editor.as_u16();
        *self.variable = edited_value; // saves data.
                                       //TODO: Make below lines unnecessary
        let number_editor = &mut self.u16_editor;
        let format = number_editor.set_u16(edited_value, self.format.clone()); // saves displayed data
        number_editor.reset_cursor(self.format.initial_cursor_position);
    }

    pub fn abort_edition(&mut self) {
        let original_value = (*self.variable).clone();
        //TODO: Make below lines unnecessary
        let number_editor = &mut self.u16_editor;
        number_editor.set_u16(original_value, self.format.clone()); // saves displayed data
        number_editor.reset_cursor(self.format.initial_cursor_position);
    }
} */

pub enum FieldEnum {
    Numerical(NumberInputEditorWidget),
    Optional(OptionalEditableWidget),
}

impl FieldEnum {
    pub fn save_edition(&mut self) {
        match self {
            Self::Numerical(x) => {
                todo!();
            } //x.save_edition(),
            Self::Optional(x) => {
                todo!();
            } //x.save_edition(),
        }
    }

    pub fn abort_edition(&mut self) {
        match self {
            Self::Numerical(x) => {
                todo!();
            } //x.abort_edition(),
            Self::Optional(x) => {
                todo!();
            } //x.abort_edition(),
        }
    }
}

impl Widget for FieldEnum {
    fn send_key(&mut self, key: KeyCode) {
        match self {
            Self::Numerical(x) => x.send_key(key),
            Self::Optional(x) => x.send_key(key),
        }
    }

    fn update(&mut self) {
        match self {
            Self::Numerical(x) => x.update(),
            Self::Optional(x) => x.update(),
        }
    }

    fn draw(&self, canvas: &mut Canvas, start_point: Point) {
        match self {
            Self::Numerical(x) => x.draw(canvas, start_point),
            Self::Optional(x) => x.draw(canvas, start_point),
        }
    }
}

//Makes possible to edit a position of memory using Lcd display and keyboard
//esc abort edition, and enter confirm edition
pub struct Field {
    field_enum: FieldEnum,
    edit_mode: EditMode,
}

impl Field {
    pub fn new(field_enum: FieldEnum) -> Self {
        Self {
            field_enum,
            edit_mode: EditMode::new(false),
        }
    }

    pub fn from_numerical<'a>(variable: &'a mut u16, parameters: Format) -> Self {
        const is_in_edit_mode: bool = false; // does not start in edit mode
        let initial_value = (*variable).clone();
        let numerical_field =
            NumberInputEditorWidget::new(initial_value, parameters, is_in_edit_mode);
        let field_enum = FieldEnum::Numerical(numerical_field);
        Self::new(field_enum)
    }

    pub fn from_optional<'a>(options: OptionsBuffer, variable: &'a mut Cursor) -> Self {
        const is_in_edit_mode: bool = false; // does not start in edit mode
        let initial_selection = (*variable).clone();
        let optional = OptionalEditableWidget::new(initial_selection, options, is_in_edit_mode);
        let field_enum = FieldEnum::Optional(optional);
        Self::new(field_enum)
    }
}

impl Widget for Field {
    fn send_key(&mut self, key: KeyCode) {
        if self.is_in_edit_mode() {
            match key {
                // cancel edition
                KeyCode::KEY_ESC => {
                    self.set_edit_mode(false); // terminate edition
                    self.field_enum.abort_edition();
                }

                // saves edition
                KeyCode::KEY_ENTER => {
                    self.set_edit_mode(false); // terminate edition
                    self.field_enum.save_edition();
                }

                //delegate everything else
                _ => self.field_enum.send_key(key),
            };
        }
    }

    fn update(&mut self) {
        self.field_enum.update()
    }

    fn draw(&self, canvas: &mut Canvas, start_point: Point) {
        let is_in_edit_mode = self.is_in_edit_mode();
        self.field_enum.draw(canvas, start_point)
    }
}

impl Field {
    pub fn set_edit_mode(&mut self, value: bool) {
        self.edit_mode.set_edit_mode(value);
    }

    pub fn is_in_edit_mode(&self) -> bool {
        self.edit_mode.is_in_edit_mode()
    }
}
