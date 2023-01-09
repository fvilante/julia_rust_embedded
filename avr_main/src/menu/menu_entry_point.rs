use super::canvas::Canvas;
use super::flash::FlashString;
use super::keyboard::Keyboard;
use super::menu_manager::MenuManager;
use super::point::Point;
use super::point::Point1d;
use super::sub_menu_handle::MenuArquivoDeEixo;
use super::sub_menu_handle::MenuPrograma;
use super::sub_menu_handle::SubMenuHandle;
use super::widget::caption::Caption;
use super::widget::execucao;
use super::widget::execucao::MenuExecucao;
use super::widget::main_menu;
use super::widget::main_menu::MainMenu;
use super::widget::main_menu::State;
use super::widget::manual_mode::ManualModeMenu;
use super::widget::manual_mode::ManualModeState;
use super::widget::menu_item;
use super::widget::menu_item::MenuItemArgs;
use super::widget::menu_item::MenuItemWidget;
use super::widget::menu_item::{NumericalParameterArgs, OptionalParameterArgs};
use super::widget::optional::OptionEditorWidget;
use super::widget::splash::Splash;
use super::widget::sub_menu_render::MenuList;
use super::widget::unsigned16_widget::Content;
use super::widget::unsigned16_widget::Field;
use super::widget::widget_tests::optional_widget_test;
use crate::board::keyboard::KeyCode;
use crate::board::output_expander::OutputExpander;
use crate::board::{lcd, output_expander};
use crate::enviroment::front_panel::FrontPanel;
use crate::menu::accessor::Accessor;
use crate::menu::flash::FlashSlice;
use crate::menu::widget::menu_item::make_template_iterator;
use crate::menu::widget::menu_item::TemplateKind;
use crate::menu::widget::optional::make_options_buffer_from_array;
use crate::menu::widget::optional::OptionsBuffer;
use crate::menu::widget::sub_menu_render::SubMenuRender;
use crate::menu::widget::unsigned16_widget::Format;
use crate::menu::widget::widget::Widget;
use crate::menu::widget::widget_tests::SystemEnviroment;
use crate::microcontroler::delay::delay_ms;
use crate::microcontroler::timer::now;
use alloc::string::ToString;
use avr_progmem::string::PmString;
use avr_progmem::wrapper::ProgMem;
use core::cell::Cell;
use core::ops::Range;
use core::str::FromStr;
use heapless::String;
use heapless::Vec;
use lib_1::arena::arena::Arena;
use lib_1::arena::arena::ArenaId;
use lib_1::utils::common::convert_u16_to_string_decimal;
use lib_1::utils::cursor::Cursor;

use avr_progmem::progmem;

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

    let menu_root = SubMenuHandle::MenuArquivoDeEixo;

    let mut submenu = SubMenuRender::new(menu_root);

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
