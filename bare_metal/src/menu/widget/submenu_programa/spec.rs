/// Here we specify the concrete menu items and submenus of the application
///
/// If you want to add more submenus or menu items consider this commit a
/// an example: 9959bd686
///
/// Note that this is a first implementation of the menu, and there is plenty of room
/// to improve and to make the operation of add submenus or menuitems a lot more easier.
/// By for example using macros! etc
use core::{cell::Cell, u8};

use crate::{
    menu::widget::menu_item::builder::{
        MenuItemBuilder, NumericalParameter, OptionalParameter, SimpleMenu,
        SimpleMenuWithNumericalParameter,
    },
    string::flash::FlashString,
};

use super::{
    super::super::{model::DataModel, widget::menu_item::menu_item::MenuItemWidget},
    core::SubmenuLayout,
    navigation_state::NavigationStateModel,
    spec_options::Options,
};

///////////////////////////////////////////////////

/// import flash texts
use super::flash_texts::*;

////////////////////////////////////////////////////

/// Indexes the sub menus in the menu arena, it works like a reference pointer to the concrete menu.
/// If you create a new sub menu you must registry it here.
#[derive(Copy, Clone, PartialEq)]
pub enum MenuProgramaAreanaSelector {
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

/// Used to store the menu itself alongside its navigation state
pub struct Register<T, S> {
    pub menu: T,
    pub navigation_state: Cell<S>,
}

impl<T, S> Register<T, S> {
    fn get_menu(&self) -> &T {
        &self.menu
    }

    fn get_navigation_state(&self) -> &Cell<S> {
        &self.navigation_state
    }
}

impl<T> Register<T, NavigationStateModel> {
    fn from_submenu(menu: T) -> Self {
        Self {
            menu,
            navigation_state: Cell::new(NavigationStateModel::new()),
        }
    }
}

impl<T: SubmenuLayout> SubmenuLayout for Register<T, NavigationStateModel> {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        self.menu.get_item(index)
    }
}

type RegisterSubMenu<T> = Register<T, NavigationStateModel>;

/// The storage for all sub menus inside the submenu 'Programa'. If you create a new sub menu you must put it here.
///
/// TODO: When possible generalize the [`MenuProgramaAreana`] to include all menus used in the app
/// (ie: splash, main_menu, menu_execucao and menu_manual, etc)
pub struct MenuProgramaArena<'a> {
    model: &'a DataModel,

    pub MenuPrograma: RegisterSubMenu<MenuPrograma<'a>>,

    // ARQUIVO DE EIXO
    pub MenuArquivoDeEixo: RegisterSubMenu<MenuArquivoDeEixo<'a>>,
    pub MenuParametrosDeMovimento: RegisterSubMenu<MenuParametrosDeMovimento<'a>>,
    pub MenuParametrosDeImpressao: RegisterSubMenu<MenuParametrosDeImpressao<'a>>,
    pub MenuParametrosDeCiclo: RegisterSubMenu<MenuParametrosDeCiclo<'a>>,
    pub MenuConfiguracaoDaImpressora: RegisterSubMenu<MenuConfiguracaoDaImpressora<'a>>,
    pub MenuIntertravamentoParaDoisEixos: RegisterSubMenu<MenuIntertravamentoParaDoisEixos<'a>>,
    //pub MenuParametrosDeSelecaoDeMensagem: RegisterSubMenu<MenuParametrosDeSelecaoDeMensagem>,
    pub MenuConfiguracaoDoEixo: RegisterSubMenu<MenuConfiguracaoDeEixo<'a>>,
    pub MenuConfiguracaoDoEquipamento: RegisterSubMenu<MenuConfiguracaoDoEquipamento<'a>>,
}

impl<'a> MenuProgramaArena<'a> {
    /// Constructs all the menus and initializes its internal state
    pub fn new(model: &'a DataModel) -> Self {
        Self {
            model,
            MenuPrograma: Register::from_submenu(MenuPrograma::new(model)),
            MenuArquivoDeEixo: Register::from_submenu(MenuArquivoDeEixo::new(model)),
            MenuParametrosDeMovimento: Register::from_submenu(MenuParametrosDeMovimento::new(
                model,
            )),
            MenuParametrosDeImpressao: Register::from_submenu(MenuParametrosDeImpressao::new(
                model,
            )),
            MenuParametrosDeCiclo: Register::from_submenu(MenuParametrosDeCiclo::new(model)),
            MenuConfiguracaoDaImpressora: Register::from_submenu(
                MenuConfiguracaoDaImpressora::new(model),
            ),
            MenuIntertravamentoParaDoisEixos: Register::from_submenu(
                MenuIntertravamentoParaDoisEixos::new(model),
            ),
            MenuConfiguracaoDoEixo: Register::from_submenu(MenuConfiguracaoDeEixo::new(model)),
            MenuConfiguracaoDoEquipamento: Register::from_submenu(
                MenuConfiguracaoDoEquipamento::new(model),
            ),
        }
    }

    /// Retrieves an menu item given the sub menu and the index number of the menu item.
    /// If index is out of range than returns None
    pub fn get_item(
        &self,
        menu_selector: MenuProgramaAreanaSelector,
        index: usize,
    ) -> Option<MenuItemWidget> {
        match menu_selector {
            MenuProgramaAreanaSelector::MenuPrograma => self.MenuPrograma.get_item(index),
            MenuProgramaAreanaSelector::MenuArquivoDeEixo => self.MenuArquivoDeEixo.get_item(index),
            MenuProgramaAreanaSelector::MenuParametrosDeMovimento => {
                self.MenuParametrosDeMovimento.get_item(index)
            }

            MenuProgramaAreanaSelector::MenuParametrosDeImpressao => {
                self.MenuParametrosDeImpressao.get_item(index)
            }

            MenuProgramaAreanaSelector::MenuParametrosDeCiclo => {
                self.MenuParametrosDeCiclo.get_item(index)
            }
            MenuProgramaAreanaSelector::MenuConfiguracaoDaImpressora => {
                self.MenuConfiguracaoDaImpressora.get_item(index)
            }

            MenuProgramaAreanaSelector::MenuIntertravamentoParaDoisEixos => {
                self.MenuIntertravamentoParaDoisEixos.get_item(index)
            }
            MenuProgramaAreanaSelector::MenuConfiguracaoDeEixo => {
                self.MenuConfiguracaoDoEixo.get_item(index)
            }
            MenuProgramaAreanaSelector::MenuConfiguracaoDoEquipamento => {
                self.MenuConfiguracaoDoEquipamento.get_item(index)
            }
        }
    }

    /// Gets the navigation state of the submenu
    /// TODO: Simplify this function implementation reusing self.get_item which has a similar implementation
    pub fn get_navigation_state(
        &self,
        menu_selector: MenuProgramaAreanaSelector,
    ) -> &Cell<NavigationStateModel> {
        match menu_selector {
            MenuProgramaAreanaSelector::MenuPrograma => self.MenuPrograma.get_navigation_state(),
            MenuProgramaAreanaSelector::MenuArquivoDeEixo => {
                self.MenuArquivoDeEixo.get_navigation_state()
            }
            MenuProgramaAreanaSelector::MenuParametrosDeMovimento => {
                self.MenuParametrosDeMovimento.get_navigation_state()
            }
            MenuProgramaAreanaSelector::MenuParametrosDeImpressao => {
                self.MenuParametrosDeImpressao.get_navigation_state()
            }
            MenuProgramaAreanaSelector::MenuParametrosDeCiclo => {
                self.MenuParametrosDeCiclo.get_navigation_state()
            }
            MenuProgramaAreanaSelector::MenuConfiguracaoDaImpressora => {
                self.MenuConfiguracaoDaImpressora.get_navigation_state()
            }
            MenuProgramaAreanaSelector::MenuIntertravamentoParaDoisEixos => {
                self.MenuIntertravamentoParaDoisEixos.get_navigation_state()
            }
            MenuProgramaAreanaSelector::MenuConfiguracaoDeEixo => {
                self.MenuConfiguracaoDoEixo.get_navigation_state()
            }
            MenuProgramaAreanaSelector::MenuConfiguracaoDoEquipamento => {
                self.MenuConfiguracaoDoEquipamento.get_navigation_state()
            }
        }
    }

    /// Given an sub menu index, get the size of menu items inside it.
    ///
    /// TODO: This algoritm may be highly optimized, because the length currently is obtained instantiating &
    /// throwing away all the menu items in memory. A better option may be to restructure datastructures
    /// to calculate this size in static time.
    pub fn len(&self, menu_selector: MenuProgramaAreanaSelector) -> usize {
        for index in 0..u8::MAX {
            if let None = self.get_item(menu_selector, index as usize) {
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
            0 => {
                MenuItemBuilder::make_simple_menu_with_parameter(SimpleMenuWithNumericalParameter {
                    parameter_name: FlashString::new(&EDITAR_PROGRAMA_EIXO_X),
                    child_menu: MenuProgramaAreanaSelector::MenuArquivoDeEixo,
                    unit_of_measurement_text: None,
                    valid_range: 0..99,
                    variable: (30, &self.model.gui_state.numero_do_programa_do_eixo_x),
                })
            }

            1 => MenuItemBuilder::make_simple_menu(SimpleMenu {
                parent_name: FlashString::new(&CONFIGURACAO_EIXO_X),
                child_menu: MenuProgramaAreanaSelector::MenuConfiguracaoDeEixo,
            }),

            2 => MenuItemBuilder::make_simple_menu(SimpleMenu {
                parent_name: FlashString::new(&CONFIGURACAO_DO_EQUIPAMENTO),
                child_menu: MenuProgramaAreanaSelector::MenuConfiguracaoDoEquipamento,
            }),

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
            0 => MenuItemBuilder::make_simple_menu(SimpleMenu {
                parent_name: FlashString::new(&PARAMETROS_DE_MOVIMENTO),
                child_menu: MenuProgramaAreanaSelector::MenuParametrosDeMovimento,
            }),

            1 => MenuItemBuilder::make_simple_menu(SimpleMenu {
                parent_name: FlashString::new(&PARAMETROS_DE_IMPRESSAO),
                child_menu: MenuProgramaAreanaSelector::MenuParametrosDeImpressao,
            }),

            2 => MenuItemBuilder::make_simple_menu(SimpleMenu {
                parent_name: FlashString::new(&CONFIGURACAO_DO_CICLO),
                child_menu: MenuProgramaAreanaSelector::MenuParametrosDeCiclo,
            }),

            3 => MenuItemBuilder::make_simple_menu(SimpleMenu {
                parent_name: FlashString::new(&CONFIGURACAO_DA_IMPRESSORA),
                child_menu: MenuProgramaAreanaSelector::MenuConfiguracaoDaImpressora,
            }),

            4 => MenuItemBuilder::make_simple_menu(SimpleMenu {
                parent_name: FlashString::new(&INTERTRAVAMENTO_DOIS_EIXOS_PASSO_A_PASSO),
                child_menu: MenuProgramaAreanaSelector::MenuIntertravamentoParaDoisEixos,
            }),

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
            0 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&POSICAO_INICIAL),
                variable: (30, &self.model.arquivo_de_eixo_x.posicao_inicial),
                valid_range: 0..9999,
                unit_of_measurement_text: Some((35, FlashString::new(&MILIMETROS))),
            }),

            1 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&POSICAO_FINAL),
                variable: (30, &self.model.arquivo_de_eixo_x.posicao_final),
                valid_range: 0..9999,
                unit_of_measurement_text: Some((35, FlashString::new(&MILIMETROS))),
            }),

            2 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&ACELERACAO_DE_AVANCO),
                variable: (30, &self.model.arquivo_de_eixo_x.aceleracao_de_avanco),
                valid_range: 0..9999,
                unit_of_measurement_text: Some((
                    35,
                    FlashString::new(&MILIMETROS_POR_SEGUNDO_AO_QUADRADO),
                )),
            }),

            3 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&ACELERACAO_DE_RETORNO),
                variable: (30, &self.model.arquivo_de_eixo_x.aceleracao_de_retorno),
                valid_range: 0..9999,
                unit_of_measurement_text: Some((
                    35,
                    FlashString::new(&MILIMETROS_POR_SEGUNDO_AO_QUADRADO),
                )),
            }),

            4 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&VELOCIDADE_DE_AVANCO),
                variable: (30, &self.model.arquivo_de_eixo_x.velocidade_de_avanco),
                valid_range: 0..9999,
                unit_of_measurement_text: Some((35, FlashString::new(&MILIMETROS_POR_SEGUNDO))),
            }),

            5 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&VELOCIDADE_DE_RETORNO),
                variable: (30, &self.model.arquivo_de_eixo_x.velocidade_de_retorno),
                valid_range: 0..9999,
                unit_of_measurement_text: Some((35, FlashString::new(&MILIMETROS_POR_SEGUNDO))),
            }),

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
            0 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&NUMERO_DE_MENSAGEM_NO_AVANCO),
                variable: (
                    35,
                    &self.model.arquivo_de_eixo_x.numero_de_mensagem_no_avanco,
                ),
                valid_range: 0..99,
                unit_of_measurement_text: None,
            }),

            1 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&NUMERO_DE_MENSAGEM_NO_AVANCO),
                variable: (35, &self.model.arquivo_de_eixo_x.velocidade_de_retorno),
                valid_range: 0..99,
                unit_of_measurement_text: None,
            }),

            2 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&PRIMEIRA_MENSAGEM_NO_AVANCO),
                variable: (
                    33,
                    &self.model.arquivo_de_eixo_x.primeira_mensagem_no_avanco,
                ),
                valid_range: 0..99,
                unit_of_measurement_text: Some((38, FlashString::new(&MILIMETROS))),
            }),

            3 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&PRIMEIRA_MENSAGEM_NO_RETORNO),
                variable: (
                    33,
                    &self.model.arquivo_de_eixo_x.primeira_mensagem_no_avanco,
                ),
                valid_range: 0..9999,
                unit_of_measurement_text: Some((38, FlashString::new(&MILIMETROS))),
            }),

            4 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&ULTIMA_MENSAGEM_NO_AVANCO),
                variable: (33, &self.model.arquivo_de_eixo_x.ultima_mensagem_no_avanco),
                valid_range: 0..9999,
                unit_of_measurement_text: Some((38, FlashString::new(&MILIMETROS))),
            }),

            5 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&ULTIMA_MENSAGEM_NO_RETORNO),
                variable: (33, &self.model.arquivo_de_eixo_x.ultima_mensagem_no_retorno),
                valid_range: 0..9999,
                unit_of_measurement_text: Some((38, FlashString::new(&MILIMETROS))),
            }),

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
            0 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&RETARDO_NO_START_AUTOMATICO),
                variable: (
                    33,
                    &self.model.arquivo_de_eixo_x.retardo_no_start_automatico,
                ),
                valid_range: 0..9999,
                unit_of_measurement_text: Some((38, FlashString::new(&MILI_SEGUNDOS))),
            }),

            1 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&RETARDO_NO_START_EXTERNO),
                variable: (33, &self.model.arquivo_de_eixo_x.retardo_no_start_externo),
                valid_range: 0..9999,
                unit_of_measurement_text: Some((38, FlashString::new(&MILI_SEGUNDOS))),
            }),

            2 => MenuItemBuilder::make_optional_parameter(OptionalParameter {
                parameter_name: FlashString::new(&START_AUTOMATICO_NO_AVANCO),
                variable: (32, &self.model.arquivo_de_eixo_x.start_automatico_no_avanco),
                options_list: Options::ligado_desligado(),
            }),

            3 => MenuItemBuilder::make_optional_parameter(OptionalParameter {
                parameter_name: FlashString::new(&START_AUTOMATICO_NO_RETORNO),
                variable: (
                    32,
                    &self.model.arquivo_de_eixo_x.start_automatico_no_retorno,
                ),
                options_list: Options::ligado_desligado(),
            }),

            4 => MenuItemBuilder::make_optional_parameter(OptionalParameter {
                parameter_name: FlashString::new(&MODO_DE_TRABALHO_DO_EIXO),
                variable: (32, &self.model.arquivo_de_eixo_x.modo_de_trabalho_do_eixo),
                options_list: Options::continuo_passo_a_passo(),
            }),

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
            0 => MenuItemBuilder::make_optional_parameter(OptionalParameter {
                parameter_name: FlashString::new(&LOGICA_DO_SINAL_DE_IMPRESSAO),
                variable: (
                    32,
                    &self.model.arquivo_de_eixo_x.logica_do_sinal_de_impressao,
                ),
                options_list: Options::aberto_fechado(),
            }),

            1 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&LARGURA_DO_SINAL_DE_IMPRESSAO),
                variable: (
                    33,
                    &self.model.arquivo_de_eixo_x.largura_do_sinal_de_impressao,
                ),
                valid_range: 0..9999,
                unit_of_measurement_text: Some((38, FlashString::new(&MILI_SEGUNDOS))),
            }),

            2 => MenuItemBuilder::make_optional_parameter(OptionalParameter {
                parameter_name: FlashString::new(&REVERSAO_DE_MENSAGEM_VIA_SERIAL),
                variable: (
                    32,
                    &self.model.arquivo_de_eixo_x.reversao_de_mensagem_via_serial,
                ),
                options_list: Options::ligado_desligado(),
            }),

            3 => MenuItemBuilder::make_optional_parameter(OptionalParameter {
                parameter_name: FlashString::new(&SELECAO_DE_MENSAGEM_VIA_SERIAL),
                variable: (
                    32,
                    &self.model.arquivo_de_eixo_x.selecao_de_mensagem_via_serial,
                ),
                options_list: Options::ligado_desligado(),
            }),

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
            0 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&ANTECIPACAO_DA_SAIDA_DE_START),
                variable: (
                    33,
                    &self.model.arquivo_de_eixo_x.antecipacao_da_saida_de_start,
                ),
                valid_range: 0..9999,
                unit_of_measurement_text: Some((38, FlashString::new(&MILI_SEGUNDOS))),
            }),

            1 => MenuItemBuilder::make_optional_parameter(OptionalParameter {
                parameter_name: FlashString::new(&SAIDA_DE_START_NO_AVANCO),
                variable: (32, &self.model.arquivo_de_eixo_x.saida_de_start_no_avaco),
                options_list: Options::ligado_desligado(),
            }),

            2 => MenuItemBuilder::make_optional_parameter(OptionalParameter {
                parameter_name: FlashString::new(&SAIDA_DE_START_NO_RETORNO),
                variable: (32, &self.model.arquivo_de_eixo_x.saida_de_start_no_retorno),
                options_list: Options::ligado_desligado(),
            }),

            3 => MenuItemBuilder::make_optional_parameter(OptionalParameter {
                parameter_name: FlashString::new(&ENTRADA_DE_START_ENTRE_EIXOS),
                variable: (
                    32,
                    &self.model.arquivo_de_eixo_x.entrada_de_start_entre_eixos,
                ),
                options_list: Options::ligado_desligado(),
            }),

            4 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&RETARDO_DO_START_ENTRE_EIXOS),
                variable: (
                    33,
                    &self.model.arquivo_de_eixo_x.retardo_do_start_entre_eixos,
                ),
                valid_range: 0..9999,
                unit_of_measurement_text: Some((38, FlashString::new(&MILI_SEGUNDOS))),
            }),

            5 => MenuItemBuilder::make_optional_parameter(OptionalParameter {
                parameter_name: FlashString::new(&START_PELO_TECLADO_E_EXTERNO),
                variable: (
                    32,
                    &self.model.arquivo_de_eixo_x.start_pelo_teclado_e_externo,
                ),
                options_list: Options::ligado_desligado(),
            }),

            6 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&RETARDO_NO_START_PASSO_A_PASSO),
                variable: (
                    33,
                    &self.model.arquivo_de_eixo_x.retardo_no_start_passo_a_passo,
                ),
                valid_range: 0..9999,
                unit_of_measurement_text: Some((38, FlashString::new(&MILI_SEGUNDOS))),
            }),

            7 => MenuItemBuilder::make_optional_parameter(OptionalParameter {
                parameter_name: FlashString::new(&START_AUTOMATICO_PASSO_A_PASSO),
                variable: (
                    32,
                    &self.model.arquivo_de_eixo_x.start_automatico_passo_a_passo,
                ),
                options_list: Options::ligado_desligado(),
            }),

            8 => MenuItemBuilder::make_optional_parameter(OptionalParameter {
                parameter_name: FlashString::new(&SAIDA_START_PASSO_A_PASSO),
                variable: (
                    32,
                    &self.model.arquivo_de_eixo_x.saida_de_start_passo_a_passo,
                ),
                options_list: Options::continuo_passo_a_passo(),
            }),

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
            0 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&NUMERO_DO_CANAL_X),
                variable: (33, &self.model.configuracao_do_eixo_x.numero_do_canal),
                valid_range: 0..99, // TODO: test define range as `0..64`
                unit_of_measurement_text: None,
            }),

            1 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&NUMERO_DE_PULSO_DO_GIRO_X),
                variable: (
                    33,
                    &self.model.configuracao_do_eixo_x.numero_de_pulso_do_giro,
                ),
                valid_range: 0..999,
                unit_of_measurement_text: None,
            }),

            2 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&JANELA_DE_PROTECAO_DO_GITO_X),
                variable: (
                    33,
                    &self.model.configuracao_do_eixo_x.janela_de_protecao_do_giro,
                ),
                valid_range: 0..9999,
                unit_of_measurement_text: None,
            }),

            3 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&DESLOCAMENTO_GIRO_DO_MOTOR_X),
                variable: (
                    33,
                    &self.model.configuracao_do_eixo_x.deslocamento_giro_do_motor,
                ),
                valid_range: 0..0xFFFF,
                unit_of_measurement_text: None,
            }),

            4 => MenuItemBuilder::make_optional_parameter(OptionalParameter {
                parameter_name: FlashString::new(&GIRO_COM_FUNCAO_DE_PROTECAO),
                variable: (
                    32,
                    &self
                        .model
                        .configuracao_do_eixo_x
                        .giro_com_funcao_de_protecao,
                ),
                options_list: Options::ligado_desligado(),
            }),

            5 => MenuItemBuilder::make_optional_parameter(OptionalParameter {
                parameter_name: FlashString::new(&GIRO_COM_FUNCAO_DE_CORRECAO),
                variable: (
                    32,
                    &self
                        .model
                        .configuracao_do_eixo_x
                        .giro_com_funcao_de_correcao,
                ),
                options_list: Options::ligado_desligado(),
            }),

            6 => MenuItemBuilder::make_optional_parameter(OptionalParameter {
                parameter_name: FlashString::new(&LOGICA_DO_START_EXTERNO),
                variable: (
                    32,
                    &self.model.configuracao_do_eixo_x.logica_do_start_externo,
                ),
                options_list: Options::aberto_fechado(),
            }),

            7 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&VALOR_DA_POSICAO_DA_REFERENCIA),
                variable: (
                    33,
                    &self
                        .model
                        .configuracao_do_eixo_x
                        .valor_da_posicao_de_referencia,
                ),
                valid_range: 0..9999,
                unit_of_measurement_text: None,
            }),

            8 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&VELOCIDADE_PARA_REFERENCIA_X),
                variable: (
                    33,
                    &self.model.configuracao_do_eixo_x.velocidade_para_referencia,
                ),
                valid_range: 0..9999,
                unit_of_measurement_text: None,
            }),

            9 => MenuItemBuilder::make_numerical_parameter(NumericalParameter {
                parameter_name: FlashString::new(&ACELERACAO_PARA_REFERENCIA_X),
                variable: (
                    33,
                    &self.model.configuracao_do_eixo_x.aceleracao_para_referencia,
                ),
                valid_range: 0..9999,
                unit_of_measurement_text: None,
            }),

            10 => MenuItemBuilder::make_optional_parameter(OptionalParameter {
                parameter_name: FlashString::new(&REDUCAO_DA_CORRENTE_EM_REPOUSO),
                variable: (
                    32,
                    &self
                        .model
                        .configuracao_do_eixo_x
                        .reducao_da_corrente_em_repouso,
                ),
                options_list: Options::ligado_desligado(),
            }),

            11 => MenuItemBuilder::make_optional_parameter(OptionalParameter {
                parameter_name: FlashString::new(&REFERENCIA_PELO_START_EXTERNO),
                variable: (
                    32,
                    &self
                        .model
                        .configuracao_do_eixo_x
                        .referencia_pelo_start_externo,
                ),
                options_list: Options::ligado_desligado(),
            }),

            12 => MenuItemBuilder::make_optional_parameter(OptionalParameter {
                parameter_name: FlashString::new(&MODO_TURBO_X),
                variable: (32, &self.model.configuracao_do_eixo_x.modo_turbo),
                options_list: Options::ligado_desligado(),
            }),

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
            0 => MenuItemBuilder::make_optional_parameter(OptionalParameter {
                parameter_name: FlashString::new(&VELOCIDADE_DE_COMUNICACAO),
                variable: (
                    32,
                    &self
                        .model
                        .configuracao_do_equipamento
                        .velocidade_de_comunicacao,
                ),
                options_list: Options::baudrate_2400_9600(),
            }),

            // TODO: Remove the need of this duplicated parameter when possible
            1 => MenuItemBuilder::make_optional_parameter(OptionalParameter {
                parameter_name: FlashString::new(&VELOCIDADE_DE_COMUNICACAO),
                variable: (
                    32,
                    &self
                        .model
                        .configuracao_do_equipamento
                        .velocidade_de_comunicacao,
                ),
                options_list: Options::baudrate_2400_9600(),
            }),

            _ => None,
        }
    }
}
