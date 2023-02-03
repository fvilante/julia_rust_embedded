use crate::menu::widget::menu_item::menu_item::MenuItemWidget;

/// Trait implemented by all sub menus
///
/// It make possible to given a sub menu to retrieve its menu items already in the Widget format.
/// Note that the Widget contains its view state, and it is brand new widget. It's your responsability
/// to own this object and control its natural lifetime.
pub trait SubmenuLayout {
    /// Gets the size of menu items inside the submenu
    fn get_item(&self, index: usize) -> Option<MenuItemWidget>;

    /// TODO: This algoritm may be highly optimized, because the length currently is obtained instantiating &
    /// throwing away all the menu items in memory. A better option may be to restructure datastructures
    /// to calculate this size in static time.
    fn len(&self) -> usize {
        for index in 0..u8::MAX {
            if let None = self.get_item(index as usize) {
                return index as usize;
            }
        }
        return 0;
    }
}
