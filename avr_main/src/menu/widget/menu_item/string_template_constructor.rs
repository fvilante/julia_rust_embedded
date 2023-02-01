use crate::{
    board::keyboard::KeyCode,
    menu::{
        canvas::Canvas,
        flash::FlashString,
        point::{Point, Point1d},
        sub_menu_handle::SubMenuHandle,
    },
};

use super::super::{
    caption::Caption,
    field::field::Field,
    field::numerical::Format,
    field::optional::OptionsBuffer,
    sub_menu_render::LcdLine,
    widget::Editable,
    widget::{Saveble, Widget},
};
use avr_progmem::string::PmString;
use core::{cell::Cell, ops::Range};
use heapless::Vec;
use lib_1::utils::{common::usize_to_u8_clamper, cursor::Cursor};

/// Creates a parser for a menu_item template string
///
/// It parses the template string (for example: "Posicao inicial     ${nnnnn} mm/s") returning an interator
/// decomposing the parsed string
pub fn make_template_iterator(flash_string: FlashString) -> FlashTemplateIterator {
    FlashTemplateIterator {
        reminder: Some(flash_string),
        is_inside_token: false,
    }
}

pub enum TemplateKind {
    /// Pure caption
    Caption(FlashString),
    /// Pure field
    Field(FlashString),
    /// Represent not well formed template string.
    ///
    /// For example when you open a token but do not closes it before the end of the template string
    /// (ie: "Foo bar ${xxxxx  ").
    IllFormed(FlashString),
}

/// Flash template string parser
pub struct FlashTemplateIterator {
    /// contatins the string that still must to be parsed, at the end of iteration its value is None
    reminder: Option<FlashString>,
    is_inside_token: bool,
}

const BEGIN_TOKEN: &[char] = &['$', '{'];
const END_TOKEN: &[char] = &['}'];

// TODO: Improve readability of below code
impl Iterator for FlashTemplateIterator {
    type Item = TemplateKind;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(reminder) = self.reminder else {
            return None;
        };

        if self.is_inside_token {
            // If it is inside token we are waiting for an end token
            let Some(end_index) = reminder.find_index(&END_TOKEN) else {
                // Ill formed (open_token without end_token).
                self.is_inside_token = false;
                self.reminder = None;
                return Some(TemplateKind::IllFormed(reminder));
            };
            // Well formed token. (end_token located)
            self.is_inside_token = false;
            let field: FlashString = reminder.sub_string(0..end_index + 1);
            let new_reminder = reminder.sub_string(end_index + 1..reminder.len());
            self.reminder = if new_reminder.len() == 0 {
                None
            } else {
                Some(new_reminder)
            };
            // removed BEGIN_TOKEN and END_TOKEN from the field
            const BEGIN_TOKEN_LENGTH: u8 = usize_to_u8_clamper(BEGIN_TOKEN.len());
            const END_TOKEN_LENGTH: u8 = usize_to_u8_clamper(END_TOKEN.len());
            let field = field.sub_string(BEGIN_TOKEN_LENGTH..field.len() - (END_TOKEN_LENGTH));
            return Some(TemplateKind::Field(field));
            // NOTE: We will ignore the second Start_Token in the case of an Start_Token -> Start_Token -> End_Token
            // TODO: Maybe in future we should create escape code for the Tokens chars
        } else {
            // If  not is_inside_token then we are looking for begin_token
            let Some(begin_index) = reminder.find_index(&BEGIN_TOKEN) else {
                // but begin token does not exist then
                // this is a pure text (without token)
                self.is_inside_token = false;
                self.reminder = None;
                return Some(TemplateKind::Caption(reminder));
            };
            // begin_token exists
            self.is_inside_token = true;
            let caption: FlashString = reminder.sub_string(0..begin_index);
            self.reminder = Some(reminder.sub_string(begin_index..reminder.len()));
            return Some(TemplateKind::Caption(caption));
        }
    }
}
