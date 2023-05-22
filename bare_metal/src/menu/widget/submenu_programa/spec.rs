/// Here we specify the concrete menu items and submenus of the application
///
/// If you want to add more submenus or menu items consider this commit a
/// an example: 9959bd686
///
/// Note that this is a first implementation of the menu, and there is plenty of room
/// to improve and to make the operation of add submenus or menuitems a lot more easier.
/// By for example using macros! etc
use core::{cell::Cell, u8};

use crate::string::flash::FlashString;

use super::{
    super::super::{
        model::DataModel,
        widget::{menu_item::builder::MenuItemBuilder, menu_item::menu_item::MenuItemWidget},
    },
    core::SubmenuLayout,
    navigation_state::NavigationStateModel,
    spec_options::Options,
};

///////////////////////////////////////////////////

/// import flash texts
use super::flash_texts::*;

////////////////////////////////////////////////////

/// Indexes the sub menus in the menu storage, it works like a reference pointer to the concrete menu.
/// If you create a new sub menu you must put it here.
///
/// TODO: Rename to `MenuStorageIndex`
#[derive(Copy, Clone, PartialEq)]
pub enum MenuProgramaHandle {
    MenuPrograma,
    MenuArquivoDeEixo,
    MenuParametrosDeMovimento,
    MenuParametrosDeImpressao,
    MenuParametrosDeCiclo,
    MenuConfiguracaoDaImpressora,
    MenuIntertravamentoParaDoisEixos,
    MenuConfiguracaoDeEixo,
    MenuConfiguracaoDoEquipamento,
}

/// Used to store the navigation state of the submenu alongside the submenu itself
pub struct Register<T: SubmenuLayout> {
    pub menu: T,
    pub navigation_state: Cell<NavigationStateModel>,
}

impl<T: SubmenuLayout> Register<T> {
    fn from_menu(menu: T) -> Self {
        Self {
            menu,
            navigation_state: Cell::new(NavigationStateModel::new()),
        }
    }

    fn get_menu(&self) -> &T {
        &self.menu
    }

    fn get_navigation_state(&self) -> &Cell<NavigationStateModel> {
        &&self.navigation_state
    }
}

impl<T: SubmenuLayout> SubmenuLayout for Register<T> {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        self.menu.get_item(index)
    }
}

/// The storage for all sub menus inside the submenu 'Programa'. If you create a new sub menu you must put it here.
/// TODO: May change name to `MenuRegister`
pub struct MenuProgramaView<'a> {
    model: &'a DataModel,

    pub MenuPrograma: Register<MenuPrograma<'a>>,

    // ARQUIVO DE EIXO
    pub MenuArquivoDeEixo: Register<MenuArquivoDeEixo<'a>>,
    pub MenuParametrosDeMovimento: Register<MenuParametrosDeMovimento<'a>>,
    pub MenuParametrosDeImpressao: Register<MenuParametrosDeImpressao<'a>>,
    pub MenuParametrosDeCiclo: Register<MenuParametrosDeCiclo<'a>>,
    pub MenuConfiguracaoDaImpressora: Register<MenuConfiguracaoDaImpressora<'a>>,
    pub MenuIntertravamentoParaDoisEixos: Register<MenuIntertravamentoParaDoisEixos<'a>>,
    //pub MenuParametrosDeSelecaoDeMensagem: Register<MenuParametrosDeSelecaoDeMensagem>,
    pub MenuConfiguracaoDoEixo: Register<MenuConfiguracaoDeEixo<'a>>,
    pub MenuConfiguracaoDoEquipamento: Register<MenuConfiguracaoDoEquipamento<'a>>,
}

impl<'a> MenuProgramaView<'a> {
    /// Constructs all the menus and initializes its internal state
    pub fn new(model: &'a DataModel) -> Self {
        Self {
            model,
            MenuPrograma: Register::from_menu(MenuPrograma::new(model)),
            MenuArquivoDeEixo: Register::from_menu(MenuArquivoDeEixo::new(model)),
            MenuParametrosDeMovimento: Register::from_menu(MenuParametrosDeMovimento::new(model)),
            MenuParametrosDeImpressao: Register::from_menu(MenuParametrosDeImpressao::new(model)),
            MenuParametrosDeCiclo: Register::from_menu(MenuParametrosDeCiclo::new(model)),
            MenuConfiguracaoDaImpressora: Register::from_menu(MenuConfiguracaoDaImpressora::new(
                model,
            )),
            MenuIntertravamentoParaDoisEixos: Register::from_menu(
                MenuIntertravamentoParaDoisEixos::new(model),
            ),
            MenuConfiguracaoDoEixo: Register::from_menu(MenuConfiguracaoDeEixo::new(model)),
            MenuConfiguracaoDoEquipamento: Register::from_menu(MenuConfiguracaoDoEquipamento::new(
                model,
            )),
        }
    }

    /// Retrieves an menu item given the sub menu and the index number of the menu item.
    /// If index is out of range than returns None
    pub fn get_item(
        &self,
        submenu_handle: MenuProgramaHandle,
        index: usize,
    ) -> Option<MenuItemWidget> {
        match submenu_handle {
            MenuProgramaHandle::MenuPrograma => self.MenuPrograma.get_item(index),
            MenuProgramaHandle::MenuArquivoDeEixo => self.MenuArquivoDeEixo.get_item(index),
            MenuProgramaHandle::MenuParametrosDeMovimento => {
                self.MenuParametrosDeMovimento.get_item(index)
            }

            MenuProgramaHandle::MenuParametrosDeImpressao => {
                self.MenuParametrosDeImpressao.get_item(index)
            }

            MenuProgramaHandle::MenuParametrosDeCiclo => self.MenuParametrosDeCiclo.get_item(index),
            MenuProgramaHandle::MenuConfiguracaoDaImpressora => {
                self.MenuConfiguracaoDaImpressora.get_item(index)
            }

            MenuProgramaHandle::MenuIntertravamentoParaDoisEixos => {
                self.MenuIntertravamentoParaDoisEixos.get_item(index)
            }
            MenuProgramaHandle::MenuConfiguracaoDeEixo => {
                self.MenuConfiguracaoDoEixo.get_item(index)
            }
            MenuProgramaHandle::MenuConfiguracaoDoEquipamento => {
                self.MenuConfiguracaoDoEquipamento.get_item(index)
            }
        }
    }

    /// Gets the navigation state of the submenu
    /// TODO: Simplify this function implementation reusing self.get_item which has a similar implementation
    pub fn get_navigation_state(
        &self,
        submenu_handle: MenuProgramaHandle,
    ) -> &Cell<NavigationStateModel> {
        match submenu_handle {
            MenuProgramaHandle::MenuPrograma => self.MenuPrograma.get_navigation_state(),
            MenuProgramaHandle::MenuArquivoDeEixo => self.MenuArquivoDeEixo.get_navigation_state(),
            MenuProgramaHandle::MenuParametrosDeMovimento => {
                self.MenuParametrosDeMovimento.get_navigation_state()
            }

            MenuProgramaHandle::MenuParametrosDeImpressao => {
                self.MenuParametrosDeImpressao.get_navigation_state()
            }

            MenuProgramaHandle::MenuParametrosDeCiclo => {
                self.MenuParametrosDeCiclo.get_navigation_state()
            }
            MenuProgramaHandle::MenuConfiguracaoDaImpressora => {
                self.MenuConfiguracaoDaImpressora.get_navigation_state()
            }

            MenuProgramaHandle::MenuIntertravamentoParaDoisEixos => {
                self.MenuIntertravamentoParaDoisEixos.get_navigation_state()
            }

            MenuProgramaHandle::MenuConfiguracaoDeEixo => {
                self.MenuConfiguracaoDoEixo.get_navigation_state()
            }
            MenuProgramaHandle::MenuConfiguracaoDoEquipamento => {
                self.MenuConfiguracaoDoEquipamento.get_navigation_state()
            }
        }
    }

    /// Given an sub menu index, get the size of menu items inside it.
    ///
    /// TODO: This algoritm may be highly optimized, because the length currently is obtained instantiating &
    /// throwing away all the menu items in memory. A better option may be to restructure datastructures
    /// to calculate this size in static time.
    pub fn len(&self, submenu_handle: MenuProgramaHandle) -> usize {
        for index in 0..u8::MAX {
            if let None = self.get_item(submenu_handle, index as usize) {
                return index as usize;
            }
        }
        return 0;
    }
}

////////////////////////////////////////////////////

pub struct MenuPrograma<'a> {
    model: &'a DataModel,
}

impl<'a> MenuPrograma<'a> {
    pub const fn new(model: &'a DataModel) -> Self {
        Self { model }
    }
}

impl SubmenuLayout for MenuPrograma<'_> {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        match index {
            0 => Some(
                MenuItemBuilder::from_text(&EDITAR_PROGRAMA_EIXO_X)
                    .add_conection_to_submenu(MenuProgramaHandle::MenuArquivoDeEixo)
                    .build(),
            ),

            1 => Some(
                MenuItemBuilder::from_text(&CONFIGURACAO_EIXO_X)
                    .add_conection_to_submenu(MenuProgramaHandle::MenuConfiguracaoDeEixo)
                    .build(),
            ),

            2 => Some(
                MenuItemBuilder::from_text(&CONFIGURACAO_DO_EQUIPAMENTO)
                    .add_conection_to_submenu(MenuProgramaHandle::MenuConfiguracaoDoEquipamento)
                    .build(),
            ),

            _ => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
//          MENU ARQUIVO DE EIXO
////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct MenuArquivoDeEixo<'a> {
    model: &'a DataModel,
}

impl<'a> MenuArquivoDeEixo<'a> {
    pub const fn new(model: &'a DataModel) -> Self {
        Self { model }
    }
}

impl SubmenuLayout for MenuArquivoDeEixo<'_> {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        match index {
            0 => Some(
                MenuItemBuilder::from_text(&PARAMETROS_DE_MOVIMENTO)
                    .add_conection_to_submenu(MenuProgramaHandle::MenuParametrosDeMovimento)
                    .build(),
            ),

            1 => Some(
                MenuItemBuilder::from_text(&PARAMETROS_DE_IMPRESSAO)
                    .add_conection_to_submenu(MenuProgramaHandle::MenuParametrosDeImpressao)
                    .build(),
            ),

            2 => Some(
                MenuItemBuilder::from_text(&CONFIGURACAO_DO_CICLO)
                    .add_conection_to_submenu(MenuProgramaHandle::MenuParametrosDeCiclo)
                    .build(),
            ),

            3 => Some(
                MenuItemBuilder::from_text(&CONFIGURACAO_DA_IMPRESSORA)
                    .add_conection_to_submenu(MenuProgramaHandle::MenuConfiguracaoDaImpressora)
                    .build(),
            ),

            4 => Some(
                MenuItemBuilder::from_text(&INTERTRAVAMENTO_DOIS_EIXOS_PASSO_A_PASSO)
                    .add_conection_to_submenu(MenuProgramaHandle::MenuIntertravamentoParaDoisEixos)
                    .build(),
            ),

            _ => None,
        }
    }
}

////////////////////////////////////////////////////

pub struct MenuParametrosDeMovimento<'a> {
    model: &'a DataModel,
}

impl<'a> MenuParametrosDeMovimento<'a> {
    pub const fn new(model: &'a DataModel) -> Self {
        Self { model }
    }
}

impl SubmenuLayout for MenuParametrosDeMovimento<'_> {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        match index {
            0 => Some(
                MenuItemBuilder::from_text(&POSICAO_INICIAL)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo_x.posicao_inicial,
                        Some(0..9999),
                        30,
                        Some((35, FlashString::new(&MILIMETROS))),
                    )
                    .build(),
            ),

            1 => Some(
                MenuItemBuilder::from_text(&POSICAO_FINAL)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo_x.posicao_final,
                        Some(0..9999),
                        30,
                        Some((35, FlashString::new(&MILIMETROS))),
                    )
                    .build(),
            ),

            2 => Some(
                MenuItemBuilder::from_text(&ACELERACAO_DE_AVANCO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo_x.aceleracao_de_avanco,
                        Some(0..9999),
                        30,
                        Some((35, FlashString::new(&MILIMETROS_POR_SEGUNDO_AO_QUADRADO))),
                    )
                    .build(),
            ),

            3 => Some(
                MenuItemBuilder::from_text(&ACELERACAO_DE_RETORNO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo_x.aceleracao_de_retorno,
                        Some(0..9999),
                        30,
                        Some((35, FlashString::new(&MILIMETROS_POR_SEGUNDO_AO_QUADRADO))),
                    )
                    .build(),
            ),
            4 => Some(
                MenuItemBuilder::from_text(&VELOCIDADE_DE_AVANCO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo_x.velocidade_de_avanco,
                        Some(0..9999),
                        30,
                        Some((35, FlashString::new(&MILIMETROS_POR_SEGUNDO))),
                    )
                    .build(),
            ),
            5 => Some(
                MenuItemBuilder::from_text(&VELOCIDADE_DE_RETORNO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo_x.velocidade_de_retorno,
                        Some(0..9999),
                        30,
                        Some((35, FlashString::new(&MILIMETROS_POR_SEGUNDO))),
                    )
                    .build(),
            ),

            _ => None,
        }
    }
}

////////////////////////////////////////////////////

pub struct MenuParametrosDeImpressao<'a> {
    model: &'a DataModel,
}

impl<'a> MenuParametrosDeImpressao<'a> {
    pub const fn new(model: &'a DataModel) -> Self {
        Self { model }
    }
}

impl SubmenuLayout for MenuParametrosDeImpressao<'_> {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        match index {
            0 => Some(
                MenuItemBuilder::from_text(&NUMERO_DE_MENSAGEM_NO_AVANCO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo_x.numero_de_mensagem_no_avanco,
                        Some(0..99),
                        35,
                        None,
                    )
                    .build(),
            ),

            1 => Some(
                MenuItemBuilder::from_text(&NUMERO_DE_MENSAGEM_NO_RETORNO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo_x.numero_de_mensagem_no_retorno,
                        Some(0..99),
                        35,
                        None,
                    )
                    .build(),
            ),

            2 => Some(
                MenuItemBuilder::from_text(&PRIMEIRA_MENSAGEM_NO_AVANCO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo_x.primeira_mensagem_no_avanco,
                        Some(0..9999),
                        33,
                        Some((38, FlashString::new(&MILIMETROS))),
                    )
                    .build(),
            ),

            3 => Some(
                MenuItemBuilder::from_text(&PRIMEIRA_MENSAGEM_NO_RETORNO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo_x.primeira_mensagem_no_retorno,
                        Some(0..9999),
                        33,
                        Some((38, FlashString::new(&MILIMETROS))),
                    )
                    .build(),
            ),

            4 => Some(
                MenuItemBuilder::from_text(&ULTIMA_MENSAGEM_NO_AVANCO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo_x.ultima_mensagem_no_avanco,
                        Some(0..9999),
                        33,
                        Some((38, FlashString::new(&MILIMETROS))),
                    )
                    .build(),
            ),

            5 => Some(
                MenuItemBuilder::from_text(&ULTIMA_MENSAGEM_NO_RETORNO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo_x.ultima_mensagem_no_retorno,
                        Some(0..9999),
                        33,
                        Some((38, FlashString::new(&MILIMETROS))),
                    )
                    .build(),
            ),

            _ => None,
        }
    }
}

////////////////////////////////////////////////////

pub struct MenuParametrosDeCiclo<'a> {
    model: &'a DataModel,
}

impl<'a> MenuParametrosDeCiclo<'a> {
    pub const fn new(model: &'a DataModel) -> Self {
        Self { model }
    }
}

impl SubmenuLayout for MenuParametrosDeCiclo<'_> {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        match index {
            0 => Some(
                MenuItemBuilder::from_text(&RETARDO_NO_START_AUTOMATICO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo_x.retardo_no_start_automatico,
                        Some(0..9999),
                        33,
                        Some((38, FlashString::new(&MILI_SEGUNDOS))),
                    )
                    .build(),
            ),

            1 => Some(
                MenuItemBuilder::from_text(&RETARDO_NO_START_EXTERNO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo_x.retardo_no_start_externo,
                        Some(0..9999),
                        33,
                        Some((38, FlashString::new(&MILI_SEGUNDOS))),
                    )
                    .build(),
            ),

            2 => Some(
                MenuItemBuilder::from_text(&START_AUTOMATICO_NO_AVANCO)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo_x.start_automatico_no_avanco,
                        Options::ligado_desligado(),
                        32,
                    )
                    .build(),
            ),

            3 => Some(
                MenuItemBuilder::from_text(&START_AUTOMATICO_NO_RETORNO)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo_x.start_automatico_no_retorno,
                        Options::ligado_desligado(),
                        32,
                    )
                    .build(),
            ),

            4 => Some(
                MenuItemBuilder::from_text(&MODO_DE_TRABALHO_DO_EIXO)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo_x.modo_de_trabalho_do_eixo,
                        Options::continuo_passo_a_passo(),
                        32,
                    )
                    .build(),
            ),

            _ => None,
        }
    }
}

////////////////////////////////////////////////////

pub struct MenuConfiguracaoDaImpressora<'a> {
    model: &'a DataModel,
}

impl<'a> MenuConfiguracaoDaImpressora<'a> {
    pub const fn new(model: &'a DataModel) -> Self {
        Self { model }
    }
}

impl SubmenuLayout for MenuConfiguracaoDaImpressora<'_> {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        match index {
            0 => Some(
                MenuItemBuilder::from_text(&LOGICA_DO_SINAL_DE_IMPRESSAO)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo_x.logica_do_sinal_de_impressao,
                        Options::aberto_fechado(),
                        32,
                    )
                    .build(),
            ),

            1 => Some(
                MenuItemBuilder::from_text(&LARGURA_DO_SINAL_DE_IMPRESSAO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo_x.largura_do_sinal_de_impressao,
                        Some(0..9999),
                        33,
                        Some((38, FlashString::new(&MILI_SEGUNDOS))),
                    )
                    .build(),
            ),

            2 => Some(
                MenuItemBuilder::from_text(&REVERSAO_DE_MENSAGEM_VIA_SERIAL)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo_x.reversao_de_mensagem_via_serial,
                        Options::ligado_desligado(),
                        32,
                    )
                    .build(),
            ),

            3 => Some(
                MenuItemBuilder::from_text(&SELECAO_DE_MENSAGEM_VIA_SERIAL)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo_x.selecao_de_mensagem_via_serial,
                        Options::ligado_desligado(),
                        32,
                    )
                    .build(),
            ),

            _ => None,
        }
    }
}

////////////////////////////////////////////////////

pub struct MenuIntertravamentoParaDoisEixos<'a> {
    model: &'a DataModel,
}

impl<'a> MenuIntertravamentoParaDoisEixos<'a> {
    pub const fn new(model: &'a DataModel) -> Self {
        Self { model }
    }
}

impl SubmenuLayout for MenuIntertravamentoParaDoisEixos<'_> {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        match index {
            0 => Some(
                MenuItemBuilder::from_text(&ANTECIPACAO_DA_SAIDA_DE_START)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo_x.antecipacao_da_saida_de_start,
                        Some(0..9999),
                        33,
                        Some((38, FlashString::new(&MILIMETROS))),
                    )
                    .build(),
            ),

            1 => Some(
                MenuItemBuilder::from_text(&SAIDA_DE_START_NO_AVANCO)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo_x.saida_de_start_no_avaco,
                        Options::ligado_desligado(),
                        32,
                    )
                    .build(),
            ),

            2 => Some(
                MenuItemBuilder::from_text(&SAIDA_DE_START_NO_RETORNO)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo_x.saida_de_start_no_retorno,
                        Options::ligado_desligado(),
                        32,
                    )
                    .build(),
            ),

            3 => Some(
                MenuItemBuilder::from_text(&ENTRADA_DE_START_ENTRE_EIXOS)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo_x.entrada_de_start_entre_eixos,
                        Options::ligado_desligado(),
                        32,
                    )
                    .build(),
            ),

            4 => Some(
                MenuItemBuilder::from_text(&RETARDO_DO_START_ENTRE_EIXOS)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo_x.retardo_do_start_entre_eixos,
                        Some(0..9999),
                        33,
                        Some((38, FlashString::new(&MILI_SEGUNDOS))),
                    )
                    .build(),
            ),

            5 => Some(
                MenuItemBuilder::from_text(&START_PELO_TECLADO_E_EXTERNO)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo_x.start_pelo_teclado_e_externo,
                        Options::ligado_desligado(),
                        32,
                    )
                    .build(),
            ),

            6 => Some(
                MenuItemBuilder::from_text(&RETARDO_NO_START_PASSO_A_PASSO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo_x.retardo_no_start_passo_a_passo,
                        Some(0..9999),
                        33,
                        Some((38, FlashString::new(&MILI_SEGUNDOS))),
                    )
                    .build(),
            ),

            7 => Some(
                MenuItemBuilder::from_text(&START_AUTOMATICO_PASSO_A_PASSO)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo_x.start_automatico_passo_a_passo,
                        Options::ligado_desligado(),
                        32,
                    )
                    .build(),
            ),

            8 => Some(
                MenuItemBuilder::from_text(&SAIDA_START_PASSO_A_PASSO)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo_x.saida_de_start_passo_a_passo,
                        Options::continuo_passo_a_passo(),
                        32,
                    )
                    .build(),
            ),

            _ => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
//          MENU CONFIGURACAO DE EIXO
////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct MenuConfiguracaoDeEixo<'a> {
    model: &'a DataModel,
}

impl<'a> MenuConfiguracaoDeEixo<'a> {
    pub const fn new(model: &'a DataModel) -> Self {
        Self { model }
    }
}

impl SubmenuLayout for MenuConfiguracaoDeEixo<'_> {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        match index {
            0 => Some(
                MenuItemBuilder::from_text(&NUMERO_DO_CANAL_X)
                    .add_numerical_variable(
                        &self.model.configuracao_do_eixo_x.numero_do_canal,
                        Some(0..99),
                        33,
                        None,
                    )
                    .build(),
            ),

            1 => Some(
                MenuItemBuilder::from_text(&NUMERO_DE_PULSO_DO_GIRO_X)
                    .add_numerical_variable(
                        &self.model.configuracao_do_eixo_x.numero_de_pulso_do_giro,
                        Some(0..999),
                        33,
                        None,
                    )
                    .build(),
            ),

            2 => Some(
                MenuItemBuilder::from_text(&JANELA_DE_PROTECAO_DO_GITO_X)
                    .add_numerical_variable(
                        &self.model.configuracao_do_eixo_x.janela_de_protecao_do_giro,
                        Some(0..9999),
                        33,
                        None,
                    )
                    .build(),
            ),

            3 => Some(
                MenuItemBuilder::from_text(&DESLOCAMENTO_GIRO_DO_MOTOR_X)
                    .add_numerical_variable(
                        &self.model.configuracao_do_eixo_x.deslocamento_giro_do_motor,
                        Some(0..0xFFFF),
                        33,
                        None,
                    )
                    .build(),
            ),

            4 => Some(
                MenuItemBuilder::from_text(&GIRO_COM_FUNCAO_DE_PROTECAO)
                    .add_optional_variable(
                        &self
                            .model
                            .configuracao_do_eixo_x
                            .giro_com_funcao_de_protecao,
                        Options::ligado_desligado(),
                        32,
                    )
                    .build(),
            ),

            5 => Some(
                MenuItemBuilder::from_text(&GIRO_COM_FUNCAO_DE_CORRECAO)
                    .add_optional_variable(
                        &self
                            .model
                            .configuracao_do_eixo_x
                            .giro_com_funcao_de_correcao,
                        Options::ligado_desligado(),
                        32,
                    )
                    .build(),
            ),

            6 => Some(
                MenuItemBuilder::from_text(&LOGICA_DO_START_EXTERNO)
                    .add_optional_variable(
                        &self.model.configuracao_do_eixo_x.logica_do_start_externo,
                        Options::aberto_fechado(),
                        32,
                    )
                    .build(),
            ),

            7 => Some(
                MenuItemBuilder::from_text(&VALOR_DA_POSICAO_DA_REFERENCIA)
                    .add_numerical_variable(
                        &self
                            .model
                            .configuracao_do_eixo_x
                            .valor_da_posicao_de_referencia,
                        Some(0..9999),
                        33,
                        None,
                    )
                    .build(),
            ),

            8 => Some(
                MenuItemBuilder::from_text(&VELOCIDADE_PARA_REFERENCIA_X)
                    .add_numerical_variable(
                        &self.model.configuracao_do_eixo_x.velocidade_para_referencia,
                        Some(0..9999),
                        33,
                        None,
                    )
                    .build(),
            ),

            9 => Some(
                MenuItemBuilder::from_text(&ACELERACAO_PARA_REFERENCIA_X)
                    .add_numerical_variable(
                        &self.model.configuracao_do_eixo_x.aceleracao_para_referencia,
                        Some(0..9999),
                        33,
                        None,
                    )
                    .build(),
            ),

            10 => Some(
                MenuItemBuilder::from_text(&REDUCAO_DA_CORRENTE_EM_REPOUSO)
                    .add_optional_variable(
                        &self
                            .model
                            .configuracao_do_eixo_x
                            .reducao_da_corrente_em_repouso,
                        Options::ligado_desligado(),
                        32,
                    )
                    .build(),
            ),

            11 => Some(
                MenuItemBuilder::from_text(&REFERENCIA_PELO_START_EXTERNO)
                    .add_optional_variable(
                        &self
                            .model
                            .configuracao_do_eixo_x
                            .referencia_pelo_start_externo,
                        Options::ligado_desligado(),
                        32,
                    )
                    .build(),
            ),

            12 => Some(
                MenuItemBuilder::from_text(&MODO_TURBO_X)
                    .add_optional_variable(
                        &self.model.configuracao_do_eixo_x.modo_turbo,
                        Options::ligado_desligado(),
                        32,
                    )
                    .build(),
            ),

            _ => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////
//          MENU CONFIGURACAO DO EQUIPAMENTO
////////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct MenuConfiguracaoDoEquipamento<'a> {
    model: &'a DataModel,
}

impl<'a> MenuConfiguracaoDoEquipamento<'a> {
    pub const fn new(model: &'a DataModel) -> Self {
        Self { model }
    }
}

impl SubmenuLayout for MenuConfiguracaoDoEquipamento<'_> {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        match index {
            0 => Some(
                MenuItemBuilder::from_text(&VELOCIDADE_DE_COMUNICACAO)
                    .add_optional_variable(
                        &self
                            .model
                            .configuracao_do_equipamento
                            .velocidade_de_comunicacao,
                        Options::baudrate_2400_9600(),
                        32,
                    )
                    .build(),
            ),
            // TODO: Remove the need of this duplicated parameter when possible
            1 => Some(
                MenuItemBuilder::from_text(&VELOCIDADE_DE_COMUNICACAO)
                    .add_optional_variable(
                        &self
                            .model
                            .configuracao_do_equipamento
                            .velocidade_de_comunicacao,
                        Options::baudrate_2400_9600(),
                        32,
                    )
                    .build(),
            ),
            _ => None,
        }
    }
}
