use core::ops::Range;
use core::str::{CharIndices, FromStr};

use alloc::borrow::ToOwned;
use heapless::{String, Vec};
use lib_1::utils::common::{convert_u16_to_string_decimal, usize_to_u8_clamper};

use crate::board::lcd;
use crate::{
    board::keyboard::KeyCode,
    menu::{accessor::Accessor, canvas::Canvas, point::Point, ratangular_wave::RectangularWave},
};

use super::optional::{Optional, OptionsBuffer};
use super::{edit_mode::EditMode, widget::Editable, widget::Widget};
use lib_1::utils::cursor::Cursor;

/// Sets the max size of the [`Content`] type
///
/// NOTE: Do not choose a number less than 5 else you cannot represent u16 data types in its decimal representation (ie: 65535)
const MAX_NUMBER_OF_CHARS_IN_BUFFER: usize = 6;

/// A string that represents some data in memory
pub type Content = String<MAX_NUMBER_OF_CHARS_IN_BUFFER>;

/// Converts an [`u16`] to a [`Content`]
///
/// TODO: What happens if number_of_digits is out_of_range?
fn convert_u16_to_content(data: u16, number_of_digits: u8) -> Content {
    const blacket_char: char = '0';
    let s = convert_u16_to_string_decimal(data);
    let mut base: Content = String::from_str(s.as_str()).unwrap();
    let mut temp: Content = String::new();
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
fn convert_content_to_u16(content: &Content) -> u16 {
    content.parse::<u16>().unwrap_or(0)
}

/// A string that represents some data content being edited by a navigation cursor.
///
/// This type is agnostic to the type of data being edited, that means you can use this type to edit numbers and even text
struct ContentEditor {
    /// String content being edited.
    content: Content,
    /// Tracks current cursor position.
    cursor: Cursor,
}

impl ContentEditor {
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
        let mut s: Content = String::new();
        for (index, current_char) in self.content.char_indices() {
            if index == current_cursor {
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

/// Wrapper of the main parameters of the [`Unsigned16Editor`]
///
/// TODO: Does not make sense to have a number of digits greater than the valid.range.end. Protect against this condition
pub struct Parameters {
    pub valid_range: Range<u16>,
    /// Number of digits you want your editor have.
    ///
    /// Must be a number between 0 and 5.
    /// TODO: otherwise it will be clamped to the nearest edge of that range.
    pub number_of_digits: u8,
    pub initial_cursor_position: u8,
}

impl Clone for Parameters {
    fn clone(&self) -> Self {
        Self {
            valid_range: self.valid_range.clone(),
            number_of_digits: self.number_of_digits.clone(),
            initial_cursor_position: self.initial_cursor_position.clone(),
        }
    }
}

/// An `unsigned integer` cursor navigated editor
///
/// This type does supose that the edition is being performed in memory and does not include the `view` mechanism
struct Unsigned16Editor {
    content_editor: ContentEditor,
    parameters: Parameters,
}

impl Unsigned16Editor {
    pub fn new(initial_content: Content, parameters: Parameters) -> Self {
        Self {
            content_editor: ContentEditor::new(initial_content, parameters.initial_cursor_position),
            parameters,
        }
    }

    pub fn from_u16(initial_value: u16, parameters: Parameters) -> Self {
        let initial_content = convert_u16_to_content(initial_value, parameters.number_of_digits);
        Self::new(initial_content, parameters)
    }

    ///TODO: Remove this method or make it unnecessary if possible
    pub fn set_u16(&mut self, value: u16) {
        let Parameters {
            initial_cursor_position,
            number_of_digits,
            ..
        } = self.parameters;
        let content = convert_u16_to_content(value, number_of_digits);
        self.content_editor = ContentEditor::new(content, initial_cursor_position);
    }

    /// Converts edited value to its [`u16`] representation
    pub fn as_u16(&self) -> u16 {
        let value = convert_content_to_u16(&self.content_editor.content);
        value
    }

    /// Normalizes (clamp) the internal value to obey [`Parameters`] restrictions, in special the range condition
    pub fn to_u16_clamped(&self) -> u16 {
        let value = self.as_u16();
        let min = self.parameters.valid_range.start;
        let max = self.parameters.valid_range.end;
        let value_clamped = value.clamp(min, max);
        value_clamped
    }

    pub fn get_current_cursor_index(&self) -> usize {
        self.content_editor.cursor.get_current()
    }
    /// Reset cursor to its default initial position
    ///
    /// NOTE: This method is not necessary the same as begin() method
    ///
    /// TODO: Remove the necessity of this method being avoiding make the Self alive when no edition is hapenning
    /// by the user (for example when the info is just being show in screen without any edition mode)
    pub fn reset_cursor(&mut self) {
        let cursor = &mut self.content_editor.cursor;
        let initial_cursor_position = self.parameters.initial_cursor_position;
        cursor.set_current(initial_cursor_position);
    }

    /// An iterator over the [char]s of a [`Unsigned16Editor`], and their positions.
    pub fn char_indices(&self) -> CharIndices {
        self.content_editor.content.char_indices()
    }
}

/// This [`Widget`] manages the edition of an [`u16`] by the user using keyboard and lcd display
pub struct Unsigned16EditorWidget<'a> {
    number_editor: Unsigned16Editor,
    variable: &'a mut u16,
    blink: RectangularWave,
}

impl<'a> Unsigned16EditorWidget<'a> {
    pub fn new(variable: &'a mut u16, parameters: Parameters) -> Self {
        let value = (*variable).clone();
        Self {
            number_editor: Unsigned16Editor::from_u16(value, parameters),
            variable,
            blink: RectangularWave::new(600, 300),
        }
    }
}

impl Unsigned16EditorWidget<'_> {
    pub fn save_edition(&mut self) {
        let normalized_value = self.number_editor.to_u16_clamped();
        *self.variable = normalized_value; // saves data
                                           //TODO: Make below lines unnecessary
        let number_editor = &mut self.number_editor;
        number_editor.set_u16(normalized_value); // saves displayed data
        number_editor.reset_cursor();
    }

    pub fn abort_edition(&mut self) {
        let original_value = (*self.variable).clone();
        //TODO: Make below lines unnecessary
        let number_editor = &mut self.number_editor;
        number_editor.set_u16(original_value); // resets displayed data
        number_editor.reset_cursor();
    }
}

impl Unsigned16EditorWidget<'_> {
    pub fn send_key(&mut self, key: KeyCode) {
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
    }
}

impl Unsigned16EditorWidget<'_> {
    pub fn update(&mut self) {
        self.blink.update(); // blinks cursor
    }

    pub fn draw(&self, canvas: &mut Canvas, start_point: Point, is_in_edit_mode: bool) {
        canvas.set_cursor(start_point);
        for (position, digit) in self.number_editor.char_indices() {
            let blink_char = '_';
            let mut current_char = digit;
            let is_current_char_over_cursor =
                position == self.number_editor.get_current_cursor_index();
            let is_time_to_blink = self.blink.read() && is_in_edit_mode; // do not blink if it is not in edit mode
            if is_current_char_over_cursor && is_time_to_blink {
                current_char = blink_char;
            }
            canvas.print_char(current_char);
        }
    }
}

pub enum FieldEnum<'a> {
    Numerical(Unsigned16EditorWidget<'a>),
    Optional(Optional<'a>),
}

impl FieldEnum<'_> {
    pub fn save_edition(&mut self) {
        match self {
            Self::Numerical(x) => x.save_edition(),
            Self::Optional(x) => x.save_edition(),
        }
    }

    pub fn abort_edition(&mut self) {
        match self {
            Self::Numerical(x) => x.abort_edition(),
            Self::Optional(x) => x.abort_edition(),
        }
    }
}

impl FieldEnum<'_> {
    pub fn send_key(&mut self, key: KeyCode) {
        match self {
            Self::Numerical(x) => x.send_key(key),
            Self::Optional(x) => x.send_key(key),
        }
    }

    pub fn update(&mut self) {
        match self {
            Self::Numerical(x) => x.update(),
            Self::Optional(x) => x.update(),
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, start_point: Point, is_in_editing_mode: bool) {
        match self {
            Self::Numerical(x) => x.draw(canvas, start_point, is_in_editing_mode),
            Self::Optional(x) => x.draw(canvas, start_point, is_in_editing_mode),
        }
    }
}

//Makes possible to edit a position of memory using Lcd display and keyboard
//esc abort edition, and enter confirm edition
pub struct Field<'a> {
    field_enum: FieldEnum<'a>,
    edit_mode: EditMode,
}

impl<'a> Field<'a> {
    pub fn new(field_enum: FieldEnum<'a>) -> Self {
        Self {
            field_enum,
            edit_mode: EditMode::new(false),
        }
    }

    pub fn from_numerical(variable: &'a mut u16, parameters: Parameters) -> Self {
        let numerical_field = Unsigned16EditorWidget::new(variable, parameters);
        let field_enum = FieldEnum::Numerical(numerical_field);
        Self::new(field_enum)
    }

    pub fn from_optional(options: OptionsBuffer, variable: &'a mut Cursor) -> Self {
        let optional = Optional::new(options, variable);
        let field_enum = FieldEnum::Optional(optional);
        Self::new(field_enum)
    }
}

impl Field<'_> {
    pub fn send_key(&mut self, key: KeyCode) {
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

    pub fn update(&mut self) {
        self.field_enum.update()
    }

    pub fn draw(&self, canvas: &mut Canvas, start_point: Point) {
        let is_in_edit_mode = self.is_in_edit_mode();
        self.field_enum.draw(canvas, start_point, is_in_edit_mode)
    }
}

impl<'a> Field<'a> {
    pub fn set_edit_mode(&mut self, value: bool) {
        self.edit_mode.set_edit_mode(value);
    }

    pub fn is_in_edit_mode(&self) -> bool {
        self.edit_mode.is_in_edit_mode()
    }
}
