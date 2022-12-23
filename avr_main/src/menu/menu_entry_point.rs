use super::canvas::Canvas;
use super::flash::FlashString;
use super::keyboard::Keyboard;
use super::menu_manager::MenuManager;
use super::point::Point;
use super::point::Point1d;
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
use super::widget::sub_menu::MenuList;
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
use crate::menu::widget::sub_menu::SubMenu;
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
    static progmem TABLE_01: [u8; 6] = [0,1,2,3,4,5];
    static progmem TABLE_02: [u8; 1] = [
        0
    ];
    static progmem string ERRO_01 = "Erro de construcao de string";

    static progmem TABLE_A: [X;3] = [
        X {
            point1_: 1,
            point2_: 30,
            format: Format {
                start: 0,
                end: 9999,
                initial_cursor_position: 2,
            }
        },
        X {
            point1_: 1,
            point2_: 30,
            format: Format {
                start: 0,
                end: 9999,
                initial_cursor_position: 3,
            }
        },
        X {
            point1_: 1,
            point2_: 30,
            format: Format {
                start: 0,
                end: 9999,
                initial_cursor_position: 4,
            }
        }
    ];

}

#[derive(Copy, Clone)]
struct X {
    point1_: u8,
    point2_: u8,
    format: Format,
}

pub struct SubMenuList {
    menu_list: MenuList,
}

impl SubMenuList {
    pub fn new() -> Self {
        Self {
            menu_list: Self::init(),
        }
    }

    fn init() -> MenuList {
        static mut value1: u16 = 0;
        static mut value2: u16 = 0;
        static mut value3: Cursor = Cursor::from_range(0..2, 0);

        let mut menu_list: MenuList = Vec::new();

        let mut menu_item = MenuItemArgs::Numerical(NumericalParameterArgs {
            point1_: TABLE_A.load_at(0).point1_,
            point2_: TABLE_A.load_at(0).point2_,
            text: FlashString::new(&POSICAO_INICIAL),
            //variable: unsafe { &mut value1 },
            parameters: TABLE_A.load_at(0).format,
        });
        menu_list.push(menu_item);

        // =========================================================
        let mut menu_item = MenuItemArgs::Numerical(NumericalParameterArgs {
            point1_: 1,
            point2_: 33,
            text: FlashString::new(&POSICAO_FINAL),
            //variable: unsafe { &mut value2 },
            parameters: Format {
                initial_cursor_position: 0,
                start: 0,
                end: 9999,
            },
        });
        menu_list.push(menu_item);

        // =========================================================
        //options
        let mut menu_item = MenuItemArgs::Optional(OptionalParameterArgs {
            point1_: 1,
            point2_: 30,
            text: FlashString::new(&START_AUTOMATICO_NO_AVANCO),
            //variable: unsafe { &mut value3 },
            options_list: make_options_buffer_from_array([
                FlashString::new(&O1),
                FlashString::new(&O2),
                FlashString::new(&O3),
                FlashString::new(&O4),
            ]),
        });
        menu_list.push(menu_item);

        // =========================================================
        let mut menu_item = MenuItemArgs::Numerical(NumericalParameterArgs {
            point1_: 1,
            point2_: 33,
            text: FlashString::new(&POSICAO_FINAL),
            //variable: unsafe { &mut value2 },
            parameters: Format {
                initial_cursor_position: 0,
                start: 0,
                end: 9999,
            },
        });
        menu_list.push(menu_item);

        // =========================================================
        let mut menu_item = MenuItemArgs::Numerical(NumericalParameterArgs {
            point1_: 1,
            point2_: 33,
            text: FlashString::new(&POSICAO_INICIAL),
            //variable: unsafe { &mut value1 },
            parameters: Format {
                initial_cursor_position: 0,
                start: 0,
                end: 9999,
            },
        });
        menu_list.push(menu_item);

        menu_list
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut MenuItemArgs> {
        self.menu_list.get_mut(index)
    }

    pub fn len(&self) -> usize {
        self.menu_list.len()
    }
}

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

    let sub_menu_list = SubMenuList::new();

    let mut submenu = SubMenu::new(sub_menu_list);

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
