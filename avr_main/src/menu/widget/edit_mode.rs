use super::widget::Editable;

pub struct EditMode {
    is_in_edit_mode: bool,
}

impl EditMode {
    pub fn new(is_in_edit_mode: bool) -> Self {
        Self { is_in_edit_mode }
    }
}

impl Editable for EditMode {
    fn set_edit_mode(&mut self, value: bool) {
        self.is_in_edit_mode = value;
    }
    fn is_in_edit_mode(&self) -> bool {
        self.is_in_edit_mode
    }
}
