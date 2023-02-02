use super::model::MachineModel;
use super::widget::submenu::sub_menu_render::SubMenuRender;

use crate::menu::widget::submenu::sub_menu_handle::{MenuStorage, SubMenuHandle};

use crate::menu::widget::widget_tests::SystemEnviroment;

use crate::microcontroler::timer::now;

///

pub fn development_entry_point() -> ! {
    //optional_widget_test();

    let SystemEnviroment {
        mut canvas,
        mut keyboard,
        ..
    } = SystemEnviroment::new();

    /*     let slice = FlashSlice::new(&TABLE_02);
    lcd::lcd_initialize();
    for data in slice.to_iterator() {
        lcd::print_u8_in_hex(data);
    }
    loop {}

    canvas.render(); */

    let machine_model = MachineModel::new();

    let menu_storage: MenuStorage = MenuStorage::new(&machine_model);

    let menu_root = SubMenuHandle::MenuArquivoDeEixo;

    let mut submenu = SubMenuRender::new(menu_root, &menu_storage);

    let fps = 30; // frames_per_second
    let mut next_frame: u64 = now() + (1000 / fps);

    loop {
        if let Some(key) = keyboard.get_key() {
            submenu.send_key(key);
        }

        if now() > next_frame {
            next_frame = now() + (1000 / fps);
            submenu.update();
            submenu.draw(&mut canvas);
            canvas.render();
        }
    }
}
