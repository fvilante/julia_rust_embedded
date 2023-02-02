use core::{cell::Cell, u8};

use lib_1::utils::cursor::Cursor;

use super::super::super::{
    flash::FlashString,
    model::MachineModel,
    widget::{
        field::optional::{make_options_buffer_from_array, OptionsBuffer},
        menu_item::builder::MenuItemBuilder,
        menu_item::menu_item::MenuItemWidget,
    },
};

///////////////////////////////////////////////////

/// import flash texts
use super::flash_text::*;

///////////////////////////////////////////////////

/// A storage for variable Options existent on the menu system
/// TODO: Eventually move this to a more appropriate place (ie: ".\widget/field/optional.rs")
struct Options;

impl Options {
    fn ligado_desligado() -> OptionsBuffer {
        let options_list = [FlashString::new(&LIGADO), FlashString::new(&DESLIGADO)];
        make_options_buffer_from_array(options_list)
    }

    fn continuo_passo_a_passo() -> OptionsBuffer {
        make_options_buffer_from_array([
            FlashString::new(&CONTINUO),
            FlashString::new(&PASSO_A_PASSO),
        ])
    }
}

////////////////////////////////////////////////////

/// Indexes the sub menus in the menu storage, it works like a reference pointer to the concrete menu.
/// If you create a new sub menu you must put it here.
///
/// TODO: Rename to `MenuStorageIndex`
#[derive(Copy, Clone)]
pub enum SubMenuHandle {
    MenuArquivoDeEixo,
    MenuParametrosDeMovimento,
    MenuParametrosDeImpressao,
    MenuParametrosDeCiclo,
    MenuConfiguracaoDaImpressora,
    MenuIntertravamentoParaDoisEixos,
}

/// The storage for all sub menus. If you create a new sub menu you must put it here.
/// TODO: May change name to `MenuRegister`
pub struct MenuStorage<'a> {
    model: &'a MachineModel,
    pub MenuArquivoDeEixo: MenuArquivoDeEixo,
    pub MenuParametrosDeMovimento: MenuParametrosDeMovimento<'a>,
    pub MenuParametrosDeImpressao: MenuParametrosDeImpressao,
    pub MenuParametrosDeCiclo: MenuParametrosDeCiclo,
    pub MenuConfiguracaoDaImpressora: MenuConfiguracaoDaImpressora,
    pub MenuIntertravamentoParaDoisEixos: MenuIntertravamentoParaDoisEixos,
}

impl<'a> MenuStorage<'a> {
    /// Constructs all the menus and initializes its internal state
    pub const fn new(model: &'a MachineModel) -> Self {
        Self {
            model,
            MenuArquivoDeEixo: MenuArquivoDeEixo::new(),
            MenuParametrosDeMovimento: MenuParametrosDeMovimento::new(model),
            MenuParametrosDeImpressao: MenuParametrosDeImpressao::new(),
            MenuParametrosDeCiclo: MenuParametrosDeCiclo::new(),
            MenuConfiguracaoDaImpressora: MenuConfiguracaoDaImpressora::new(),
            MenuIntertravamentoParaDoisEixos: MenuIntertravamentoParaDoisEixos::new(),
        }
    }

    /// Retrieves an menu item given the sub menu and the index number of the menu item.
    /// If index is out of range than returns None
    pub fn get_item(&self, submenu_handle: SubMenuHandle, index: usize) -> Option<MenuItemWidget> {
        match submenu_handle {
            SubMenuHandle::MenuArquivoDeEixo => self.MenuArquivoDeEixo.get_item(index),
            SubMenuHandle::MenuParametrosDeMovimento => {
                self.MenuParametrosDeMovimento.get_item(index)
            }

            SubMenuHandle::MenuParametrosDeImpressao => {
                self.MenuParametrosDeImpressao.get_item(index)
            }

            SubMenuHandle::MenuParametrosDeCiclo => self.MenuParametrosDeCiclo.get_item(index),
            SubMenuHandle::MenuConfiguracaoDaImpressora => {
                self.MenuConfiguracaoDaImpressora.get_item(index)
            }

            SubMenuHandle::MenuIntertravamentoParaDoisEixos => {
                self.MenuIntertravamentoParaDoisEixos.get_item(index)
            }
        }
    }

    /// Given an sub menu index, get the size of menu items inside it.
    ///
    /// TODO: This algoritm may be highly optimized, because the length currently is obtained instantiating &
    /// throwing away all the menu items in memory. A better option may be to restructure datastructures
    /// to calculate this size in static time.
    pub fn len(&self, submenu_handle: SubMenuHandle) -> usize {
        for index in 0..u8::MAX {
            if let None = self.get_item(submenu_handle, index as usize) {
                return index as usize;
            }
        }
        return 0;
    }
}

////////////////////////////////////////////////////

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

////////////////////////////////////////////////////

pub struct MenuArquivoDeEixo;

impl MenuArquivoDeEixo {
    pub const fn new() -> Self {
        Self {}
    }
}

impl SubmenuLayout for MenuArquivoDeEixo {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        match index {
            0 => Some(
                MenuItemBuilder::from_text(&PARAMETROS_DE_MOVIMENTO)
                    .add_conection_to_submenu(SubMenuHandle::MenuParametrosDeMovimento)
                    .build(),
            ),

            1 => Some(
                MenuItemBuilder::from_text(&PARAMETROS_DE_IMPRESSAO)
                    .add_conection_to_submenu(SubMenuHandle::MenuParametrosDeImpressao)
                    .build(),
            ),

            2 => Some(
                MenuItemBuilder::from_text(&CONFIGURACAO_DO_CICLO)
                    .add_conection_to_submenu(SubMenuHandle::MenuParametrosDeCiclo)
                    .build(),
            ),

            3 => Some(
                MenuItemBuilder::from_text(&CONFIGURACAO_DA_IMPRESSORA)
                    .add_conection_to_submenu(SubMenuHandle::MenuConfiguracaoDaImpressora)
                    .build(),
            ),

            4 => Some(
                MenuItemBuilder::from_text(&INTERTRAVAMENTO_DOIS_EIXOS_PASSO_A_PASSO)
                    .add_conection_to_submenu(SubMenuHandle::MenuIntertravamentoParaDoisEixos)
                    .build(),
            ),

            5 => Some(
                MenuItemBuilder::from_text(&PARAMETROS_SELECAO_DE_MENSAGEM)
                    .add_conection_to_submenu(SubMenuHandle::MenuParametrosDeMovimento)
                    .build(),
            ),

            _ => None,
        }
    }
}

////////////////////////////////////////////////////

pub struct MenuParametrosDeMovimento<'a> {
    model: &'a MachineModel,
}

impl<'a> MenuParametrosDeMovimento<'a> {
    pub const fn new(model: &'a MachineModel) -> Self {
        Self { model }
    }
}

impl<'a> SubmenuLayout for MenuParametrosDeMovimento<'a> {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        match index {
            0 => Some(
                MenuItemBuilder::from_text(&POSICAO_INICIAL)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.parametro1,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),

            1 => Some(
                MenuItemBuilder::from_text(&POSICAO_FINAL)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.parametro1,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),

            2 => Some(
                MenuItemBuilder::from_text(&ACELERACAO_DE_AVANCO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.parametro1,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),

            3 => Some(
                MenuItemBuilder::from_text(&ACELERACAO_DE_RETORNO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.parametro1,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),
            4 => Some(
                MenuItemBuilder::from_text(&VELOCIDADE_DE_AVANCO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.parametro1,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),
            5 => Some(
                MenuItemBuilder::from_text(&VELOCIDADE_DE_RETORNO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.parametro1,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),

            _ => None,
        }
    }
}

////////////////////////////////////////////////////

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

impl SubmenuLayout for MenuParametrosDeImpressao {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        match index {
            0 => Some(
                MenuItemBuilder::from_text(&NUMERO_DE_MENSAGEM_NO_AVANCO)
                    .add_numerical_variable(&self.value1, Some(0..99), 33)
                    .build(),
            ),

            1 => Some(
                MenuItemBuilder::from_text(&NUMERO_DE_MENSAGEM_NO_RETORNO)
                    .add_numerical_variable(&self.value1, Some(0..99), 33)
                    .build(),
            ),

            2 => Some(
                MenuItemBuilder::from_text(&PRIMEIRO_MENSAGEM_NO_AVANCO)
                    .add_numerical_variable(&self.value1, Some(0..9999), 33)
                    .build(),
            ),

            3 => Some(
                MenuItemBuilder::from_text(&PRIMEIRO_MENSAGEM_NO_RETORNO)
                    .add_numerical_variable(&self.value1, Some(0..9999), 33)
                    .build(),
            ),

            4 => Some(
                MenuItemBuilder::from_text(&ULTIMA_MENSAGEM_NO_AVANCO)
                    .add_numerical_variable(&self.value1, Some(0..9999), 33)
                    .build(),
            ),

            5 => Some(
                MenuItemBuilder::from_text(&ULTIMA_MENSAGEM_NO_RETORNO)
                    .add_numerical_variable(&self.value1, Some(0..9999), 33)
                    .build(),
            ),

            6 => Some(
                MenuItemBuilder::from_text(&ULTIMA_MENSAGEM_NO_RETORNO)
                    .add_optional_variable(&self.value0, Options::ligado_desligado(), 30)
                    .build(),
            ),

            7 => Some(
                MenuItemBuilder::from_text(&NUMERO_DE_MULTIPLAS_IMPRESSOES)
                    .add_numerical_variable(&self.value1, Some(0..9999), 33)
                    .build(),
            ),

            8 => Some(
                MenuItemBuilder::from_text(&PASSO_DAS_MULTIPLAS_IMPRESSOES)
                    .add_numerical_variable(&self.value1, Some(0..9999), 33)
                    .build(),
            ),

            _ => None,
        }
    }
}

////////////////////////////////////////////////////

pub struct MenuParametrosDeCiclo {
    value0: Cell<Cursor>,
    value1: Cell<u16>,
}

impl MenuParametrosDeCiclo {
    pub const fn new() -> Self {
        Self {
            value0: Cell::new(Cursor::new(0, 2, 1)),
            value1: Cell::new(0),
        }
    }
}

impl SubmenuLayout for MenuParametrosDeCiclo {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        match index {
            0 => Some(
                MenuItemBuilder::from_text(&RETARDO_NO_START_AUTOMATICO)
                    .add_numerical_variable(&self.value1, Some(0..9999), 33)
                    .build(),
            ),

            1 => Some(
                MenuItemBuilder::from_text(&RETARDO_NO_START_EXTERNO)
                    .add_numerical_variable(&self.value1, Some(0..9999), 33)
                    .build(),
            ),

            2 => Some(
                MenuItemBuilder::from_text(&START_AUTOMATICO_NO_AVANCO)
                    .add_optional_variable(&self.value0, Options::ligado_desligado(), 30)
                    .build(),
            ),

            3 => Some(
                MenuItemBuilder::from_text(&START_AUTOMATICO_NO_RETORNO)
                    .add_optional_variable(&self.value0, Options::ligado_desligado(), 30)
                    .build(),
            ),

            4 => Some(
                MenuItemBuilder::from_text(&MODO_DE_TRABALHO_DO_EIXO)
                    .add_optional_variable(&self.value0, Options::continuo_passo_a_passo(), 30)
                    .build(),
            ),

            _ => None,
        }
    }
}

////////////////////////////////////////////////////

pub struct MenuConfiguracaoDaImpressora {
    value0: Cell<Cursor>,
    value1: Cell<u16>,
}

impl MenuConfiguracaoDaImpressora {
    pub const fn new() -> Self {
        Self {
            value0: Cell::new(Cursor::new(0, 2, 1)),
            value1: Cell::new(0),
        }
    }
}

impl SubmenuLayout for MenuConfiguracaoDaImpressora {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        match index {
            0 => Some(
                MenuItemBuilder::from_text(&LOGICA_DO_SINAL_DE_IMPRESSAO)
                    .add_optional_variable(&self.value0, Options::ligado_desligado(), 30)
                    .build(),
            ),

            1 => Some(
                MenuItemBuilder::from_text(&LARGURA_DO_SINAL_DE_IMPRESSAO)
                    .add_numerical_variable(&self.value1, Some(0..9999), 33)
                    .build(),
            ),

            2 => Some(
                MenuItemBuilder::from_text(&REVERSAO_DE_MENSAGEM_VIA_SERIAL)
                    .add_optional_variable(&self.value0, Options::ligado_desligado(), 30)
                    .build(),
            ),

            3 => Some(
                MenuItemBuilder::from_text(&SELECAO_DE_MENSAGEM_VIA_SERIAL)
                    .add_optional_variable(&self.value0, Options::ligado_desligado(), 30)
                    .build(),
            ),

            _ => None,
        }
    }
}

////////////////////////////////////////////////////

pub struct MenuIntertravamentoParaDoisEixos {
    value0: Cell<Cursor>,
    value1: Cell<u16>,
}

impl MenuIntertravamentoParaDoisEixos {
    pub const fn new() -> Self {
        Self {
            value0: Cell::new(Cursor::new(0, 2, 1)),
            value1: Cell::new(0),
        }
    }
}

impl SubmenuLayout for MenuIntertravamentoParaDoisEixos {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        match index {
            0 => Some(
                MenuItemBuilder::from_text(&ANTECIPACAO_DA_SAIDA_DE_START)
                    .add_numerical_variable(&self.value1, Some(0..9999), 33)
                    .build(),
            ),

            1 => Some(
                MenuItemBuilder::from_text(&SAIDA_DE_START_NO_AVANCO)
                    .add_optional_variable(&self.value0, Options::ligado_desligado(), 30)
                    .build(),
            ),

            2 => Some(
                MenuItemBuilder::from_text(&SAIDA_DE_START_NO_RETORNO)
                    .add_optional_variable(&self.value0, Options::ligado_desligado(), 30)
                    .build(),
            ),

            3 => Some(
                MenuItemBuilder::from_text(&ENTRADA_DE_START_ENTRE_EIXOS)
                    .add_optional_variable(&self.value0, Options::ligado_desligado(), 30)
                    .build(),
            ),

            4 => Some(
                MenuItemBuilder::from_text(&RETARDO_DO_START_ENTRE_EIXOS)
                    .add_numerical_variable(&self.value1, Some(0..9999), 33)
                    .build(),
            ),

            5 => Some(
                MenuItemBuilder::from_text(&START_PELO_TECLADO_E_EXTERNO)
                    .add_optional_variable(&self.value0, Options::ligado_desligado(), 30)
                    .build(),
            ),

            6 => Some(
                MenuItemBuilder::from_text(&RETARDO_NO_START_PASSO_A_PASSO)
                    .add_numerical_variable(&self.value1, Some(0..9999), 33)
                    .build(),
            ),

            7 => Some(
                MenuItemBuilder::from_text(&START_AUTOMATICO_PASSO_A_PASSO)
                    .add_optional_variable(&self.value0, Options::ligado_desligado(), 30)
                    .build(),
            ),

            8 => Some(
                MenuItemBuilder::from_text(&SAIDA_START_PASSO_A_PASSO)
                    .add_optional_variable(&self.value0, Options::continuo_passo_a_passo(), 30)
                    .build(),
            ),

            _ => None,
        }
    }
}

////////////////////////////////////////////////////
