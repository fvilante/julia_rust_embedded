use crate::{
    board::keyboard::KeyCode,
    menu::{canvas::Canvas, point::Point},
};

/// A component that can be draw on lcd display, receive key strokes from keyboard and update its own state
pub trait Widget {
    fn send_key(&mut self, key: KeyCode);
    fn update(&mut self);
    /// `start_point` points to the coordinates to start to draw the widget
    /// TODO: Not sure this parameter should be a rectangle or even do not exist
    /// TODO: Instead of `start_point` i should give a rectangle where the widget is responsible to draw it self
    /// while the client is responsible to define where this rectangle is placed in the screen. This kind of implementation
    /// make draw() method more generic
    fn draw(&self, canvas: &mut Canvas, start_point: Point);
}

/// Represents an editable type (ie: editable Widget)
pub trait Editable {
    fn set_edit_mode(&mut self, value: bool);
    fn is_in_edit_mode(&self) -> bool;
    fn toggle_edit_mode(&mut self) {
        if self.is_in_edit_mode() {
            self.set_edit_mode(false);
        } else {
            self.set_edit_mode(true);
        }
    }
}

/// Used for Widgets that may save or restore some content
pub trait Saveble {
    /// Restores original the value from variable to the widget.
    ///
    /// This is used to exit the field edition without saving the edited value
    fn restore_value(&mut self);

    /// Save the edited value by the user in the widget in the memory variable
    ///
    /// This is used to save the edited value in the variable.
    fn save_value(&mut self);
}
