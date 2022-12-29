use avr_progmem::progmem;

use super::{
    flash::FlashString,
    widget::{
        menu_item::{MenuItemArgs, MenuItemWidget, NumericalParameterArgs, OptionalParameterArgs},
        optional::make_options_buffer_from_array,
        unsigned16_widget::Format,
    },
};

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
}

struct MenuStorage {
    pub MenuPrograma: MenuPrograma,
    pub MenuArquivoDeEixo: MenuArquivoDeEixo,
}

impl MenuStorage {
    pub const fn new() -> Self {
        Self {
            MenuPrograma: MenuPrograma::new(),
            MenuArquivoDeEixo: MenuArquivoDeEixo::new(),
        }
    }
}

static MENU_STORAGE: MenuStorage = MenuStorage::new();

pub enum SubMenuHandle {
    MenuPrograma,
    MenuArquivoDeEixo,
}

impl SubMenuHandle {
    pub fn get_item<'a>(&self, index: usize) -> Option<MenuItemWidget> {
        match self {
            SubMenuHandle::MenuPrograma => MENU_STORAGE.MenuPrograma.get_item(index),
            SubMenuHandle::MenuArquivoDeEixo => MENU_STORAGE.MenuArquivoDeEixo.get_item(index),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            SubMenuHandle::MenuPrograma => MENU_STORAGE.MenuPrograma.len(),
            SubMenuHandle::MenuArquivoDeEixo => MENU_STORAGE.MenuArquivoDeEixo.len(),
        }
    }
}

pub struct MenuPrograma;

impl MenuPrograma {
    pub const fn new() -> Self {
        Self {}
    }

    pub fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        let menu_item_args = match index {
            0 => {
                Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                    point1_: 1,
                    point2_: 33,
                    text: FlashString::new(&POSICAO_INICIAL),
                    //variable: unsafe { &mut value1 },
                    parameters: Format {
                        initial_cursor_position: 0,
                        start: 0,
                        end: 9999,
                    },
                    child: Some(SubMenuHandle::MenuArquivoDeEixo),
                }))
            }

            1 => {
                Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                    point1_: 1,
                    point2_: 33,
                    text: FlashString::new(&POSICAO_FINAL),
                    //variable: unsafe { &mut value2 },
                    parameters: Format {
                        initial_cursor_position: 0,
                        start: 0,
                        end: 9999,
                    },
                    child: None,
                }))
            }

            2 => {
                Some(MenuItemArgs::Optional(OptionalParameterArgs {
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
                    child: None,
                }))
            }

            3 => {
                Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                    point1_: 1,
                    point2_: 33,
                    text: FlashString::new(&POSICAO_INICIAL),
                    //variable: unsafe { &mut value2 },
                    parameters: Format {
                        initial_cursor_position: 0,
                        start: 0,
                        end: 9999,
                    },
                    child: None,
                }))
            }

            4 => {
                Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                    point1_: 1,
                    point2_: 33,
                    text: FlashString::new(&POSICAO_FINAL),
                    //variable: unsafe { &mut value2 },
                    parameters: Format {
                        initial_cursor_position: 0,
                        start: 0,
                        end: 9999,
                    },
                    child: None,
                }))
            }

            5 => {
                Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                    point1_: 1,
                    point2_: 33,
                    text: FlashString::new(&POSICAO_INICIAL),
                    //variable: unsafe { &mut value2 },
                    parameters: Format {
                        initial_cursor_position: 0,
                        start: 0,
                        end: 9999,
                    },
                    child: None,
                }))
            }

            //
            6 => {
                Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                    point1_: 1,
                    point2_: 33,
                    text: FlashString::new(&POSICAO_INICIAL),
                    //variable: unsafe { &mut value1 },
                    parameters: Format {
                        initial_cursor_position: 0,
                        start: 0,
                        end: 9999,
                    },
                    child: None,
                }))
            }

            7 => {
                Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                    point1_: 1,
                    point2_: 33,
                    text: FlashString::new(&POSICAO_FINAL),
                    //variable: unsafe { &mut value2 },
                    parameters: Format {
                        initial_cursor_position: 0,
                        start: 0,
                        end: 9999,
                    },
                    child: None,
                }))
            }

            8 => {
                Some(MenuItemArgs::Optional(OptionalParameterArgs {
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
                    child: None,
                }))
            }

            9 => {
                Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                    point1_: 1,
                    point2_: 33,
                    text: FlashString::new(&POSICAO_INICIAL),
                    //variable: unsafe { &mut value2 },
                    parameters: Format {
                        initial_cursor_position: 0,
                        start: 0,
                        end: 9999,
                    },
                    child: None,
                }))
            }

            10 => {
                Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                    point1_: 1,
                    point2_: 33,
                    text: FlashString::new(&POSICAO_FINAL),
                    //variable: unsafe { &mut value2 },
                    parameters: Format {
                        initial_cursor_position: 0,
                        start: 0,
                        end: 9999,
                    },
                    child: None,
                }))
            }

            11 => {
                Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                    point1_: 1,
                    point2_: 33,
                    text: FlashString::new(&POSICAO_INICIAL),
                    //variable: unsafe { &mut value2 },
                    parameters: Format {
                        initial_cursor_position: 0,
                        start: 0,
                        end: 9999,
                    },
                    child: None,
                }))
            }

            12 => {
                Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                    point1_: 1,
                    point2_: 33,
                    text: FlashString::new(&POSICAO_INICIAL),
                    //variable: unsafe { &mut value1 },
                    parameters: Format {
                        initial_cursor_position: 0,
                        start: 0,
                        end: 9999,
                    },
                    child: None,
                }))
            }

            13 => {
                Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                    point1_: 1,
                    point2_: 33,
                    text: FlashString::new(&POSICAO_FINAL),
                    //variable: unsafe { &mut value2 },
                    parameters: Format {
                        initial_cursor_position: 0,
                        start: 0,
                        end: 9999,
                    },
                    child: None,
                }))
            }

            14 => {
                Some(MenuItemArgs::Optional(OptionalParameterArgs {
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
                    child: None,
                }))
            }

            15 => {
                Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                    point1_: 1,
                    point2_: 33,
                    text: FlashString::new(&POSICAO_INICIAL),
                    //variable: unsafe { &mut value2 },
                    parameters: Format {
                        initial_cursor_position: 0,
                        start: 0,
                        end: 9999,
                    },
                    child: None,
                }))
            }

            16 => {
                Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                    point1_: 1,
                    point2_: 33,
                    text: FlashString::new(&POSICAO_FINAL),
                    //variable: unsafe { &mut value2 },
                    parameters: Format {
                        initial_cursor_position: 0,
                        start: 0,
                        end: 9999,
                    },
                    child: None,
                }))
            }

            17 => {
                Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                    point1_: 1,
                    point2_: 33,
                    text: FlashString::new(&POSICAO_INICIAL),
                    //variable: unsafe { &mut value2 },
                    parameters: Format {
                        initial_cursor_position: 0,
                        start: 0,
                        end: 9999,
                    },
                    child: None,
                }))
            }

            _ => None,
        };

        if let Some(menu_args) = menu_item_args {
            Some(MenuItemWidget::from_menu_args(menu_args))
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        17
    }
}

pub struct MenuArquivoDeEixo;

impl MenuArquivoDeEixo {
    pub const fn new() -> Self {
        Self {}
    }

    pub fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        let menu_item_args = match index {
            0 => {
                Some(MenuItemArgs::Optional(OptionalParameterArgs {
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
                    child: None,
                }))
            }

            1 => {
                Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                    point1_: 1,
                    point2_: 33,
                    text: FlashString::new(&POSICAO_INICIAL),
                    //variable: unsafe { &mut value2 },
                    parameters: Format {
                        initial_cursor_position: 0,
                        start: 0,
                        end: 9999,
                    },
                    child: None,
                }))
            }

            2 => {
                Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                    point1_: 1,
                    point2_: 33,
                    text: FlashString::new(&POSICAO_FINAL),
                    //variable: unsafe { &mut value2 },
                    parameters: Format {
                        initial_cursor_position: 0,
                        start: 0,
                        end: 9999,
                    },
                    child: None,
                }))
            }

            _ => None,
        };

        if let Some(menu_args) = menu_item_args {
            Some(MenuItemWidget::from_menu_args(menu_args))
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        3
    }
}
