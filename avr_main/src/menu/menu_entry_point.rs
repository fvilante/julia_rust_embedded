use super::canvas::Canvas;
use super::flash::FlashString;
use super::keyboard::Keyboard;
use super::menu_manager::MenuManager;
use super::point::Point;
use super::point::Point1d;
use super::widget::caption::Caption;
use super::widget::execucao;
use super::widget::execucao::MenuExecucao;
use super::widget::field::Field;
use super::widget::field::FieldBuffer;
use super::widget::main_menu;
use super::widget::main_menu::MainMenu;
use super::widget::main_menu::State;
use super::widget::manual_mode::ManualModeMenu;
use super::widget::manual_mode::ManualModeState;
use super::widget::menu_item;
use super::widget::menu_item::parse_menu_item_constructor_string;
use super::widget::menu_item::MenuItem;
use super::widget::menu_item::MenuItemArgs;
use super::widget::menu_item::MenuItemParsed;
use super::widget::menu_item::{NumericalParameterArgs, OptionalParameterArgs};
use super::widget::optional::Optional;
use super::widget::splash::Splash;
use super::widget::sub_menu::MenuList;
use super::widget::widget_tests::optional_widget_test;
use crate::board::keyboard::KeyCode;
use crate::board::output_expander::OutputExpander;
use crate::board::{lcd, output_expander};
use crate::enviroment::front_panel::FrontPanel;
use crate::menu::accessor::Accessor;
use crate::menu::widget::optional::make_options_buffer_from_array;
use crate::menu::widget::optional::OptionsBuffer;
use crate::menu::widget::sub_menu::SubMenu;
use crate::menu::widget::widget::Widget;
use crate::menu::widget::widget_tests::SystemEnviroment;
use crate::microcontroler::delay::delay_ms;
use crate::microcontroler::timer::now;
use alloc::string::ToString;
use avr_progmem::string::PmString;
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

progmem! {

    //                          123456789012345678901234567890123456789 -> 39 characters
    static progmem string T0 = "Posicao inicial             ${nnnnn} mm";
    static progmem string T1 = "Posicao final               ${nnnnn} mm";
    static progmem string T2 = "Velocidade de avanco      ${nnnnn} mm/s";
    static progmem string T3 = "Velocidade de retorno     ${nnnnn} mm/s";
    static progmem string T4 = "Aceleracao de avanco     ${nnnnn} mm/s2";
    static progmem string T5 = "Aceleracao de reto       ${nnnnn} mm/s2";
    static progmem string T6 = "Numero de mensagens no avanco     ${nn}";
    static progmem string T7 = "Numero de mensagens no retorno    ${nn}";
    static progmem string T8 = "Modo continuo ou passo-a-passo [${alt1}]";
    static progmem string T9 = "Logica do start externo        [${alt2}]";


    static progmem string E0 = "Erro de carga de parametro";
    static progmem string POSICAO_INICIAL = "Posicao Inicial";
    static progmem string POSICAO_FINAL = "Posicao Final";
    static progmem string VELOCIDADE_DE_AVANCO = "Velocidade de Avanco";
    static progmem string VELOCIDADE_DE_RETORNO = "Velocidade de Retorno";
    static progmem string S4 = "Aceleracao de Avanco";
    static progmem string S5 = "Aceleracao de Retorno";
    static progmem string START_AUTOMATICO_NO_AVANCO = "Start Automatico no Avanco";
    static progmem string START_AUTOMATICO_NO_RETORNO = "Start Automatico no Retorno";
    static progmem string O1 = "Ligado";
    static progmem string O2 = "Deslig";
    static progmem string O3 = "Juca  ";
    static progmem string O4 = "Nego  ";

    //NOTE: it is possible to load any type in progmem not only strings
    static progmem A0: [u8; 6] = [0,1,2,3,4,5];
    static progmem string ERRO_01 = "Erro de construcao de string";
}

pub fn development_entry_point() -> ! {
    //optional_widget_test();

    let SystemEnviroment {
        mut canvas,
        mut keyboard,
        ..
    } = SystemEnviroment::new();

    canvas.render();

    //    //main menu
    //    let mut menu_execucao = MenuExecucao::new();
    //    let mut menu_manual = ManualModeMenu::new();
    //    let mut main_menu: MainMenu = MainMenu::new(menu_manual, menu_execucao);
    //
    //    // main loop
    //    loop {
    //
    //        if let Some(key) = keyboard.get_key() {
    //            main_menu.send_key(key);
    //        }
    //
    //        main_menu.update();
    //        main_menu.draw(&mut canvas);
    //        canvas.render();
    //    }
    //
    //    // menu view
    //    let splash1 = &mut Splash::new(None);
    //    let splash2 = &mut Splash::new(Some(splash1));
    //    let mut menu_manager = MenuManager::new(Some(splash2));
    //    loop {
    //        if let Some(key) = keyboard.get_key() {
    //            menu_manager.send_key(key);
    //        }
    //
    //        menu_manager.update();
    //        menu_manager.draw(&mut canvas);
    //        canvas.render();
    //    }

    // submenu

    // -----

    let mut value1: u16 = 0;
    let mut value2: u16 = 0;
    let mut value3 = Cursor::from_range(0..2, 0);

    fn to_pointer_mut<T>(ref_: &mut T) -> *mut T {
        ref_ as *mut T
    }

    let mut menu_list: MenuList = Vec::new();

    // =========================================================
    let mut menu_item = MenuItemArgs::Numerical(NumericalParameterArgs {
        point1_: 1,
        point2_: 33,
        text: FlashString::new(&POSICAO_INICIAL),
        variable: to_pointer_mut(&mut value1),
        initial_cursor_position: 0,
        number_of_digits: 4,
        valid_range: 0..100,
    });
    menu_list.push(menu_item);

    // =========================================================
    let mut menu_item = MenuItemArgs::Numerical(NumericalParameterArgs {
        point1_: 1,
        point2_: 33,
        text: FlashString::new(&POSICAO_FINAL),
        variable: to_pointer_mut(&mut value2),
        initial_cursor_position: 0,
        number_of_digits: 4,
        valid_range: 0..0xFFFF,
    });
    menu_list.push(menu_item);

    // =========================================================
    //options
    let mut menu_item = MenuItemArgs::Optional(OptionalParameterArgs {
        point1_: 1,
        point2_: 33,
        text: FlashString::new(&START_AUTOMATICO_NO_AVANCO),
        variable: to_pointer_mut(&mut value3),
        options_list: make_options_buffer_from_array([
            FlashString::new(&O1),
            FlashString::new(&O2),
            FlashString::new(&O3),
            FlashString::new(&O4),
        ]),
    });
    menu_list.push(menu_item);

    // =========================================================
    //    let mut menu_item = MenuItem::from_numerical(NumericalParameterArgs {
    //        point1_: 1,
    //        point2_: 33,
    //        text: FlashString::new(&POSICAO_FINAL),
    //        variable: &mut db.file_03,
    //        initial_cursor_position: 0,
    //        number_of_digits: 4,
    //        valid_range: 0..100,
    //    });
    //    menu_list.push(menu_item);//
    //
    //    // =========================================================
    //    let mut menu_item = MenuItem::from_numerical(NumericalParameterArgs {
    //        point1_: 1,
    //        point2_: 33,
    //        text: FlashString::new(&POSICAO_FINAL),
    //        variable: &mut db.file_04,
    //        initial_cursor_position: 0,
    //        number_of_digits: 4,
    //        valid_range: 0..0xFFFF,
    //    });
    //    menu_list.push(menu_item);
    //
    //    // =========================================================
    //    let mut menu_item = MenuItem::from_numerical(NumericalParameterArgs {
    //        point1_: 1,
    //        point2_: 33,
    //        text: FlashString::new(&POSICAO_INICIAL),
    //        variable: &mut db.file_05,
    //        initial_cursor_position: 0,
    //        number_of_digits: 4,
    //        valid_range: 0..100,
    //    });
    //    menu_list.push(menu_item);

    // =========================================================
    //    let mut menu_item = MenuItem::from_numerical(NumericalParameterArgs {
    //        point1_: 1,
    //        point2_: 33,
    //        text: FlashString::new(&POSICAO_FINAL),
    //        variable: &mut db.file_06,
    //        initial_cursor_position: 0,
    //        number_of_digits: 4,
    //        valid_range: 0..0xFFFF,
    //    });
    //
    //    menu_list.push(menu_item);

    // -----------------------------

    let mut submenu = SubMenu::new(menu_list);

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
