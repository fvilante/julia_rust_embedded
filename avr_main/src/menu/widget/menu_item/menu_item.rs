use crate::{
    board::keyboard::KeyCode,
    menu::{
        canvas::Canvas,
        flash::FlashString,
        point::{Point, Point1d},
        widget::{caption::caption::Caption, submenu::hepers::LcdLine},
    },
};

use crate::menu::widget::submenu::spec::SubMenuHandle;

use super::super::{
    field::field::Field,
    widget::Editable,
    widget::{Saveble, Widget},
};

//

pub struct MenuItemWidget<'a> {
    point_and_caption: (Point1d, Caption),
    point_and_field: Option<(Point1d, Field<'a>)>,
    pub child: Option<SubMenuHandle>,
}

impl<'a> MenuItemWidget<'a> {
    /// NOTE: client should put point1 and point2 in the same line
    /// point1 = position of caption, point2 = position of field
    pub fn new(
        point_and_text: (Point1d, FlashString),
        point_and_field: Option<(Point1d, Field<'a>)>,
        child: Option<SubMenuHandle>,
    ) -> Self {
        let (point_a, text) = point_and_text;
        Self {
            point_and_caption: (point_a, Caption::new(text)),
            point_and_field,
            child,
        }
    }
}

impl Saveble for MenuItemWidget<'_> {
    fn restore_value(&mut self) {
        self.set_edit_mode(false); // terminate the edition
        let Some((_, field)) = &mut self.point_and_field else { return();};
        field.restore_value();
    }

    fn save_value(&mut self) {
        self.set_edit_mode(false); // terminate the edition
        let Some((_, field)) = &mut self.point_and_field else { return(); };
        field.save_value();
    }
}

impl MenuItemWidget<'_> {
    pub fn send_key(&mut self, key: KeyCode) {
        if self.is_in_edit_mode() {
            match key {
                // cancel edition
                KeyCode::KEY_ESC => {
                    self.restore_value();
                }

                // saves edition
                KeyCode::KEY_ENTER => {
                    self.save_value();
                }

                //delegate everything else
                _ => {
                    if let Some((_, field)) = &mut self.point_and_field {
                        field.send_key(key);
                    };
                }
            };
        }
    }

    pub fn update(&mut self) {
        let (_, caption) = &mut self.point_and_caption;
        caption.update();
        if let Some((_, field)) = &mut self.point_and_field {
            field.update();
        };
    }

    pub fn draw(&self, canvas: &mut Canvas, lcd_line: LcdLine) {
        let line = lcd_line as u8;
        let (point1, caption) = &self.point_and_caption;
        let point1: Point<u8> = Point::new(point1.pos, line);
        caption.draw(canvas, point1);
        if let Some((point2, field)) = &self.point_and_field {
            let point2: Point<u8> = Point::new(point2.pos, line);
            field.draw(canvas, point2);
        };
    }
}

impl MenuItemWidget<'_> {
    pub fn set_edit_mode(&mut self, value: bool) {
        if let Some((_, field)) = &mut self.point_and_field {
            field.set_edit_mode(value);
        };
    }

    pub fn is_in_edit_mode(&self) -> bool {
        if let Some((_, field)) = &self.point_and_field {
            field.is_in_edit_mode()
        } else {
            false
        }
    }
}
