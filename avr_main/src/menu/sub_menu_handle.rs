use core::{cell::Cell, u8};

use avr_progmem::progmem;
use lib_1::utils::cursor::Cursor;

use super::{
    flash::FlashString,
    widget::{
        menu_item::{
            MenuItemArgs, MenuItemWidget, NumericalParameterArgs, OptionalParameterArgs,
            SubmenuTitleArgs,
        },
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

    //ARQUIVO DE EIXO

    static progmem string PARAMETROS_DE_MOVIMENTO = "Parametro de Movimento...";
    static progmem string PARAMETROS_DE_IMPRESSAO = "Parametros de Impressao...";
    static progmem string CONFIGURACAO_DO_CICLO = "Configuracao do Ciclo...";
    static progmem string CONFIGURACAO_DA_IMPRESSORA = "Configuracao da impressora...";
    static progmem string INTERTRAVAMENTO_DOIS_EIXOS_PASSO_A_PASSO = "Intertravamento: dois eixos e pas/pas..";
    static progmem string PARAMETROS_SELECAO_DE_MENSAGEM = "Parametros de Selecao de mensagem...";

    // PARAMETROS DE MOVIMENTO

    static progmem string POSICAO_INICIAL = "Posicao inicial";
    static progmem string POSICAO_FINAL = "Posicao final";
    static progmem string ACELERACAO_DE_AVANCO = "Aceleracao de avanco";
    static progmem string ACELERACAO_DE_RETORNO = "Aceleracao de retorno";
    static progmem string VELOCIDADE_DE_AVANCO = "Velocidade de avanco";
    static progmem string VELOCIDADE_DE_RETORNO = "Velocidade de retorno";

    // PARAMETROS DE IMPRESSAO

    static progmem string NUMERO_DE_MENSAGEM_NO_AVANCO = "Numero de mensagem no avanco";
    static progmem string NUMERO_DE_MENSAGEM_NO_RETORNO= "Numero de mensagem no retorno";
    static progmem string PRIMEIRO_MENSAGEM_NO_AVANCO= "Primeira mensagem no avanco";
    static progmem string PRIMEIRO_MENSAGEM_NO_RETORNO= "Primeira mensagem no retorno";
    static progmem string ULTIMA_MENSAGEM_NO_AVANCO = "Ultima mensagem no avanco";
    static progmem string ULTIMA_MENSAGEM_NO_RETORNO = "Ultima mensagem no retorno";
    static progmem string MENSAGEM_REVERSA_LIGADA = "Mensagem reversa ligada";
    static progmem string NUMERO_DE_MULTIPLAS_IMPRESSOES = "Numero de multiplas impressoes";
    static progmem string PASSO_DAS_MULTIPLAS_IMPRESSOES = "passo das multiplas impressoes";




    static progmem string START_AUTOMATICO_NO_AVANCO = "Start Automatico no Avanco";
    static progmem string START_AUTOMATICO_NO_RETORNO = "Start Automatico no Retorno";
    static progmem string LIGADA = "Ligado";
    static progmem string DESLIGADA = "Deslig";
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
    pub MenuArquivoDeEixo: MenuArquivoDeEixo,
    pub MenuParametrosDeMovimento: MenuParametrosDeMovimento,
    pub MenuParametrosDeImpressao: MenuParametrosDeImpressao,
}

impl MenuStorage {
    pub const fn new() -> Self {
        Self {
            MenuArquivoDeEixo: MenuArquivoDeEixo::new(),
            MenuParametrosDeMovimento: MenuParametrosDeMovimento::new(),
            MenuParametrosDeImpressao: MenuParametrosDeImpressao::new(),
        }
    }
}

static mut MENU_STORAGE: MenuStorage = MenuStorage::new();

pub trait SubMenuTrait {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget>;
    fn len(&self) -> usize {
        /// TODO: This algoritm may be highly optimized, because the length is obtained instantiating &
        /// throwing away all the menu items in memory. A better option may be to restructure datastructures
        /// to calculate this size in static time.
        for index in 0..u8::MAX {
            if let None = self.get_item(index as usize) {
                return index as usize;
            }
        }
        return 0;
    }
}

#[derive(Copy, Clone)]
pub enum SubMenuHandle {
    MenuArquivoDeEixo,
    MenuParametrosDeMovimento,
    MenuParametrosDeImpressao,
}
impl SubMenuHandle {
    pub fn get_item<'a>(&self, index: usize) -> Option<MenuItemWidget<'a>> {
        match self {
            SubMenuHandle::MenuArquivoDeEixo => unsafe {
                MENU_STORAGE.MenuArquivoDeEixo.get_item(index)
            },
            SubMenuHandle::MenuParametrosDeMovimento => unsafe {
                MENU_STORAGE.MenuParametrosDeMovimento.get_item(index)
            },
            SubMenuHandle::MenuParametrosDeImpressao => unsafe {
                MENU_STORAGE.MenuParametrosDeImpressao.get_item(index)
            },
        }
    }

    pub fn len(&self) -> usize {
        /// TODO: This algoritm may be highly optimized, because the length is obtained instantiating &
        /// throwing away all the menu items in memory. A better option may be to restructure datastructures
        /// to calculate this size in static time.
        for index in 0..u8::MAX {
            if let None = self.get_item(index as usize) {
                return index as usize;
            }
        }
        return 0;
    }
}

pub struct MenuArquivoDeEixo {}

impl MenuArquivoDeEixo {
    pub const fn new() -> Self {
        Self {}
    }
}

impl SubMenuTrait for MenuArquivoDeEixo {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        let menu_item_args = match index {
            0 => Some(MenuItemArgs::SubmenuTitle(SubmenuTitleArgs {
                point1_: 1,
                text: FlashString::new(&PARAMETROS_DE_MOVIMENTO),
                child: Some(SubMenuHandle::MenuParametrosDeMovimento),
            })),

            1 => Some(MenuItemArgs::SubmenuTitle(SubmenuTitleArgs {
                point1_: 1,
                text: FlashString::new(&PARAMETROS_DE_IMPRESSAO),
                child: Some(SubMenuHandle::MenuParametrosDeImpressao),
            })),

            2 => Some(MenuItemArgs::SubmenuTitle(SubmenuTitleArgs {
                point1_: 1,
                text: FlashString::new(&CONFIGURACAO_DO_CICLO),
                child: Some(SubMenuHandle::MenuParametrosDeMovimento),
            })),

            3 => Some(MenuItemArgs::SubmenuTitle(SubmenuTitleArgs {
                point1_: 1,
                text: FlashString::new(&CONFIGURACAO_DA_IMPRESSORA),
                child: Some(SubMenuHandle::MenuParametrosDeMovimento),
            })),

            4 => Some(MenuItemArgs::SubmenuTitle(SubmenuTitleArgs {
                point1_: 1,
                text: FlashString::new(&INTERTRAVAMENTO_DOIS_EIXOS_PASSO_A_PASSO),
                child: Some(SubMenuHandle::MenuParametrosDeMovimento),
            })),

            5 => Some(MenuItemArgs::SubmenuTitle(SubmenuTitleArgs {
                point1_: 1,
                text: FlashString::new(&PARAMETROS_SELECAO_DE_MENSAGEM),
                child: Some(SubMenuHandle::MenuParametrosDeMovimento),
            })),

            _ => None,
        };

        if let Some(menu_args) = menu_item_args {
            Some(MenuItemWidget::from_menu_args(menu_args))
        } else {
            None
        }
    }
}

pub struct MenuParametrosDeMovimento {
    value0: Cell<Cursor>,
    value1: Cell<u16>,
    value2: Cell<u16>,
}

impl MenuParametrosDeMovimento {
    pub const fn new() -> Self {
        Self {
            value0: Cell::new(Cursor::new(0, 2, 1)),
            value1: Cell::new(0),
            value2: Cell::new(0),
        }
    }
}

impl SubMenuTrait for MenuParametrosDeMovimento {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        let menu_item_args = match index {
            0 => Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                point1_: 1,
                point2_: 33,
                text: FlashString::new(&POSICAO_INICIAL),
                parameters: Format {
                    initial_cursor_position: 0,
                    start: 0,
                    end: 9999,
                },
                child: None,
                variable: &self.value1,
            })),

            1 => Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                point1_: 1,
                point2_: 33,
                text: FlashString::new(&POSICAO_FINAL),
                parameters: Format {
                    initial_cursor_position: 0,
                    start: 0,
                    end: 9999,
                },
                child: None,
                variable: &self.value1,
            })),

            2 => Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                point1_: 1,
                point2_: 33,
                text: FlashString::new(&ACELERACAO_DE_AVANCO),
                parameters: Format {
                    initial_cursor_position: 0,
                    start: 0,
                    end: 9999,
                },
                child: None,
                variable: &self.value1,
            })),

            3 => Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                point1_: 1,
                point2_: 33,
                text: FlashString::new(&ACELERACAO_DE_RETORNO),
                parameters: Format {
                    initial_cursor_position: 0,
                    start: 0,
                    end: 9999,
                },
                child: None,
                variable: &self.value1,
            })),

            4 => Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                point1_: 1,
                point2_: 33,
                text: FlashString::new(&VELOCIDADE_DE_AVANCO),
                parameters: Format {
                    initial_cursor_position: 0,
                    start: 0,
                    end: 9999,
                },
                child: None,
                variable: &self.value1,
            })),

            5 => Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                point1_: 1,
                point2_: 33,
                text: FlashString::new(&VELOCIDADE_DE_RETORNO),
                parameters: Format {
                    initial_cursor_position: 0,
                    start: 0,
                    end: 9999,
                },
                child: None,
                variable: &self.value1,
            })),

            _ => None,
        };

        if let Some(menu_args) = menu_item_args {
            Some(MenuItemWidget::from_menu_args(menu_args))
        } else {
            None
        }
    }
}

pub struct MenuParametrosDeImpressao {
    value0: Cell<Cursor>,
    value1: Cell<u16>,
}

impl MenuParametrosDeImpressao {
    pub const fn new() -> Self {
        Self {
            value0: Cell::new(Cursor::new(0, 2, 1)),
            value1: Cell::new(0),
        }
    }
}

impl SubMenuTrait for MenuParametrosDeImpressao {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        let menu_item_args = match index {
            0 => Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                point1_: 1,
                point2_: 35,
                text: FlashString::new(&NUMERO_DE_MENSAGEM_NO_AVANCO),
                parameters: Format {
                    initial_cursor_position: 0,
                    start: 0,
                    end: 99,
                },
                child: None,
                variable: &self.value1,
            })),

            1 => Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                point1_: 1,
                point2_: 35,
                text: FlashString::new(&NUMERO_DE_MENSAGEM_NO_RETORNO),
                parameters: Format {
                    initial_cursor_position: 0,
                    start: 0,
                    end: 99,
                },
                child: None,
                variable: &self.value1,
            })),

            2 => Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                point1_: 1,
                point2_: 33,
                text: FlashString::new(&PRIMEIRO_MENSAGEM_NO_AVANCO),
                parameters: Format {
                    initial_cursor_position: 0,
                    start: 0,
                    end: 9999,
                },
                child: None,
                variable: &self.value1,
            })),

            3 => Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                point1_: 1,
                point2_: 33,
                text: FlashString::new(&PRIMEIRO_MENSAGEM_NO_RETORNO),
                parameters: Format {
                    initial_cursor_position: 0,
                    start: 0,
                    end: 9999,
                },
                child: None,
                variable: &self.value1,
            })),

            4 => Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                point1_: 1,
                point2_: 33,
                text: FlashString::new(&ULTIMA_MENSAGEM_NO_AVANCO),
                parameters: Format {
                    initial_cursor_position: 0,
                    start: 0,
                    end: 9999,
                },
                child: None,
                variable: &self.value1,
            })),

            5 => Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                point1_: 1,
                point2_: 33,
                text: FlashString::new(&ULTIMA_MENSAGEM_NO_RETORNO),
                parameters: Format {
                    initial_cursor_position: 0,
                    start: 0,
                    end: 9999,
                },
                child: None,
                variable: &self.value1,
            })),

            6 => Some(MenuItemArgs::Optional(OptionalParameterArgs {
                point1_: 1,
                point2_: 30,
                text: FlashString::new(&MENSAGEM_REVERSA_LIGADA),
                options_list: make_options_buffer_from_array([
                    FlashString::new(&LIGADA),
                    FlashString::new(&DESLIGADA),
                ]),
                child: None,
                variable: &self.value0,
            })),

            7 => Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                point1_: 1,
                point2_: 35,
                text: FlashString::new(&NUMERO_DE_MULTIPLAS_IMPRESSOES),
                parameters: Format {
                    initial_cursor_position: 0,
                    start: 0,
                    end: 99,
                },
                child: None,
                variable: &self.value1,
            })),

            8 => Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                point1_: 1,
                point2_: 33,
                text: FlashString::new(&PASSO_DAS_MULTIPLAS_IMPRESSOES),
                parameters: Format {
                    initial_cursor_position: 0,
                    start: 0,
                    end: 9999,
                },
                child: None,
                variable: &self.value1,
            })),

            _ => None,
        };

        if let Some(menu_args) = menu_item_args {
            Some(MenuItemWidget::from_menu_args(menu_args))
        } else {
            None
        }
    }
}

/*

pub struct Teste2 {
    value0: Cell<Cursor>,
    value1: Cell<u16>,
    value2: Cell<u16>,
}

impl Teste2 {
    pub const fn new() -> Self {
        Self {
            value0: Cell::new(Cursor::new(0, 2, 1)),
            value1: Cell::new(0),
            value2: Cell::new(0),
        }
    }
}

impl SubMenuTrait for Teste2 {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        let menu_item_args = match index {
            1 => {
                Some(MenuItemArgs::Optional(OptionalParameterArgs {
                    point1_: 1,
                    point2_: 30,
                    text: FlashString::new(&START_AUTOMATICO_NO_AVANCO),
                    options_list: make_options_buffer_from_array([
                        FlashString::new(&O1),
                        FlashString::new(&O2),
                        FlashString::new(&O3),
                        FlashString::new(&O4),
                    ]),
                    child: None,
                    variable: &self.value0,
                }))
            }

            0 => {
                Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                    point1_: 1,
                    point2_: 33,
                    text: FlashString::new(&POSICAO_INICIAL),
                    parameters: Format {
                        initial_cursor_position: 0,
                        start: 0,
                        end: 9999,
                    },
                    child: None,
                    variable: &self.value1,
                }))
            }

            2 => {
                Some(MenuItemArgs::Numerical(NumericalParameterArgs {
                    point1_: 1,
                    point2_: 33,
                    text: FlashString::new(&POSICAO_FINAL),
                    parameters: Format {
                        initial_cursor_position: 0,
                        start: 0,
                        end: 9999,
                    },
                    child: None,
                    variable: &self.value2,
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
}

 */
