use core::{cell::Cell, u8};

use lib_1::utils::common::usize_to_u8_clamper;

use super::{
    super::super::{
        model::MachineModel,
        widget::{menu_item::builder::MenuItemBuilder, menu_item::menu_item::MenuItemWidget},
    },
    core::SubmenuLayout,
    navigation_state::NavigationState,
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
pub enum SubMenuHandle {
    MenuPrograma,
    MenuArquivoDeEixo,
    MenuParametrosDeMovimento,
    MenuParametrosDeImpressao,
    MenuParametrosDeCiclo,
    MenuConfiguracaoDaImpressora,
    MenuIntertravamentoParaDoisEixos,
    MenuConfiguracaoDeEixo,
}

/// Used to store the navigation state of the submenu alongside the submenu itself
pub struct Register<T: SubmenuLayout> {
    pub menu: T,
    pub navigation_state: Cell<NavigationState>,
}

impl<T: SubmenuLayout> Register<T> {
    fn from_menu(menu: T) -> Self {
        let menu_length = usize_to_u8_clamper(menu.len());
        Self {
            menu,
            navigation_state: Cell::new(NavigationState::new_from_submenu_len(menu_length)),
        }
    }

    fn get_menu(&self) -> &T {
        &self.menu
    }

    fn get_navigation_state(&self) -> &Cell<NavigationState> {
        &&self.navigation_state
    }
}

impl<T: SubmenuLayout> SubmenuLayout for Register<T> {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        self.menu.get_item(index)
    }
}

/// The storage for all sub menus. If you create a new sub menu you must put it here.
/// TODO: May change name to `MenuRegister`
pub struct MenuStorage<'a> {
    model: &'a MachineModel,

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
}

impl<'a> MenuStorage<'a> {
    /// Constructs all the menus and initializes its internal state
    pub fn new(model: &'a MachineModel) -> Self {
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
        }
    }

    /// Retrieves an menu item given the sub menu and the index number of the menu item.
    /// If index is out of range than returns None
    pub fn get_item(&self, submenu_handle: SubMenuHandle, index: usize) -> Option<MenuItemWidget> {
        match submenu_handle {
            SubMenuHandle::MenuPrograma => self.MenuPrograma.get_item(index),
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
            SubMenuHandle::MenuConfiguracaoDeEixo => self.MenuConfiguracaoDoEixo.get_item(index),
        }
    }

    /// Gets the navigation state of the submenu
    /// TODO: Simplify this function implementation reusing self.get_item which has a similar implementation
    pub fn get_navigation_state(&self, submenu_handle: SubMenuHandle) -> &Cell<NavigationState> {
        match submenu_handle {
            SubMenuHandle::MenuPrograma => self.MenuPrograma.get_navigation_state(),
            SubMenuHandle::MenuArquivoDeEixo => self.MenuArquivoDeEixo.get_navigation_state(),
            SubMenuHandle::MenuParametrosDeMovimento => {
                self.MenuParametrosDeMovimento.get_navigation_state()
            }

            SubMenuHandle::MenuParametrosDeImpressao => {
                self.MenuParametrosDeImpressao.get_navigation_state()
            }

            SubMenuHandle::MenuParametrosDeCiclo => {
                self.MenuParametrosDeCiclo.get_navigation_state()
            }
            SubMenuHandle::MenuConfiguracaoDaImpressora => {
                self.MenuConfiguracaoDaImpressora.get_navigation_state()
            }

            SubMenuHandle::MenuIntertravamentoParaDoisEixos => {
                self.MenuIntertravamentoParaDoisEixos.get_navigation_state()
            }

            SubMenuHandle::MenuConfiguracaoDeEixo => {
                self.MenuConfiguracaoDoEixo.get_navigation_state()
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

pub struct MenuPrograma<'a> {
    model: &'a MachineModel,
}

impl<'a> MenuPrograma<'a> {
    pub const fn new(model: &'a MachineModel) -> Self {
        Self { model }
    }
}

impl SubmenuLayout for MenuPrograma<'_> {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        match index {
            0 => Some(
                MenuItemBuilder::from_text(&NUMERO_DO_PROGRAMA_PARA_EDICAO)
                    .add_conection_to_submenu(SubMenuHandle::MenuArquivoDeEixo)
                    .build(),
            ),

            1 => Some(
                MenuItemBuilder::from_text(&NUMERO_DO_PROGRAMA_DO_EIXO_X)
                    .add_conection_to_submenu(SubMenuHandle::MenuArquivoDeEixo)
                    .build(),
            ),

            2 => Some(
                MenuItemBuilder::from_text(&NUMERO_DO_PROGRAMA_DO_EIXO_Y)
                    .add_conection_to_submenu(SubMenuHandle::MenuArquivoDeEixo)
                    .build(),
            ),

            3 => Some(
                MenuItemBuilder::from_text(&COPIAR_O_PROGRAMA_NUMERO)
                    .add_conection_to_submenu(SubMenuHandle::MenuPrograma)
                    .build(),
            ),

            4 => Some(
                MenuItemBuilder::from_text(&TROCA_DE_NIVEL_DE_ACCESSO)
                    .add_conection_to_submenu(SubMenuHandle::MenuPrograma)
                    .build(),
            ),

            5 => Some(
                MenuItemBuilder::from_text(&CONFIGURACAO_DO_EIXO_X)
                    .add_conection_to_submenu(SubMenuHandle::MenuConfiguracaoDeEixo)
                    .build(),
            ),

            6 => Some(
                MenuItemBuilder::from_text(&CONFIGURACAO_DO_EIXO_Y)
                    .add_conection_to_submenu(SubMenuHandle::MenuConfiguracaoDeEixo)
                    .build(),
            ),

            7 => Some(
                MenuItemBuilder::from_text(&CONFIGURACAO_DO_EQUIPAMENTO)
                    .add_conection_to_submenu(SubMenuHandle::MenuPrograma)
                    .build(),
            ),

            8 => Some(
                MenuItemBuilder::from_text(&ROTINAS_DE_TESTES_E_VERIFICACAO)
                    .add_conection_to_submenu(SubMenuHandle::MenuPrograma)
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
    model: &'a MachineModel,
}

impl<'a> MenuArquivoDeEixo<'a> {
    pub const fn new(model: &'a MachineModel) -> Self {
        Self { model }
    }
}

impl SubmenuLayout for MenuArquivoDeEixo<'_> {
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

impl SubmenuLayout for MenuParametrosDeMovimento<'_> {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        match index {
            0 => Some(
                MenuItemBuilder::from_text(&POSICAO_INICIAL)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.posicao_inicial,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),

            1 => Some(
                MenuItemBuilder::from_text(&POSICAO_FINAL)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.posicao_final,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),

            2 => Some(
                MenuItemBuilder::from_text(&ACELERACAO_DE_AVANCO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.aceleracao_de_avanco,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),

            3 => Some(
                MenuItemBuilder::from_text(&ACELERACAO_DE_RETORNO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.aceleracao_de_retorno,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),
            4 => Some(
                MenuItemBuilder::from_text(&VELOCIDADE_DE_AVANCO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.velocidade_de_avanco,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),
            5 => Some(
                MenuItemBuilder::from_text(&VELOCIDADE_DE_RETORNO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.velocidade_de_retorno,
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

pub struct MenuParametrosDeImpressao<'a> {
    model: &'a MachineModel,
}

impl<'a> MenuParametrosDeImpressao<'a> {
    pub const fn new(model: &'a MachineModel) -> Self {
        Self { model }
    }
}

impl SubmenuLayout for MenuParametrosDeImpressao<'_> {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        match index {
            0 => Some(
                MenuItemBuilder::from_text(&NUMERO_DE_MENSAGEM_NO_AVANCO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.numero_de_mensagem_no_avanco,
                        Some(0..99),
                        33,
                    )
                    .build(),
            ),

            1 => Some(
                MenuItemBuilder::from_text(&NUMERO_DE_MENSAGEM_NO_RETORNO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.numero_de_mensagem_no_retorno,
                        Some(0..99),
                        33,
                    )
                    .build(),
            ),

            2 => Some(
                MenuItemBuilder::from_text(&PRIMEIRA_MENSAGEM_NO_AVANCO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.primeira_mensagem_no_avanco,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),

            3 => Some(
                MenuItemBuilder::from_text(&PRIMEIRA_MENSAGEM_NO_RETORNO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.primeira_mensagem_no_avanco,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),

            4 => Some(
                MenuItemBuilder::from_text(&ULTIMA_MENSAGEM_NO_AVANCO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.ultima_mensagem_no_avanco,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),

            5 => Some(
                MenuItemBuilder::from_text(&ULTIMA_MENSAGEM_NO_RETORNO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.ultima_mensagem_no_retorno,
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

pub struct MenuParametrosDeCiclo<'a> {
    model: &'a MachineModel,
}

impl<'a> MenuParametrosDeCiclo<'a> {
    pub const fn new(model: &'a MachineModel) -> Self {
        Self { model }
    }
}

impl SubmenuLayout for MenuParametrosDeCiclo<'_> {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        match index {
            0 => Some(
                MenuItemBuilder::from_text(&RETARDO_NO_START_AUTOMATICO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.retardo_no_start_automatico,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),

            1 => Some(
                MenuItemBuilder::from_text(&RETARDO_NO_START_EXTERNO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.retardo_no_start_externo,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),

            2 => Some(
                MenuItemBuilder::from_text(&START_AUTOMATICO_NO_AVANCO)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo.start_automatico_no_avanco,
                        Options::ligado_desligado(),
                        30,
                    )
                    .build(),
            ),

            3 => Some(
                MenuItemBuilder::from_text(&START_AUTOMATICO_NO_RETORNO)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo.start_automatico_no_retorno,
                        Options::ligado_desligado(),
                        30,
                    )
                    .build(),
            ),

            4 => Some(
                MenuItemBuilder::from_text(&MODO_DE_TRABALHO_DO_EIXO)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo.modo_de_trabalho_do_eixo,
                        Options::continuo_passo_a_passo(),
                        30,
                    )
                    .build(),
            ),

            _ => None,
        }
    }
}

////////////////////////////////////////////////////

pub struct MenuConfiguracaoDaImpressora<'a> {
    model: &'a MachineModel,
}

impl<'a> MenuConfiguracaoDaImpressora<'a> {
    pub const fn new(model: &'a MachineModel) -> Self {
        Self { model }
    }
}

impl SubmenuLayout for MenuConfiguracaoDaImpressora<'_> {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        match index {
            0 => Some(
                MenuItemBuilder::from_text(&LOGICA_DO_SINAL_DE_IMPRESSAO)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo.logica_do_sinal_de_impressao,
                        Options::ligado_desligado(),
                        30,
                    )
                    .build(),
            ),

            1 => Some(
                MenuItemBuilder::from_text(&LARGURA_DO_SINAL_DE_IMPRESSAO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.largura_do_sinal_de_impressao,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),

            2 => Some(
                MenuItemBuilder::from_text(&REVERSAO_DE_MENSAGEM_VIA_SERIAL)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo.reversao_de_mensagem_via_serial,
                        Options::ligado_desligado(),
                        30,
                    )
                    .build(),
            ),

            3 => Some(
                MenuItemBuilder::from_text(&SELECAO_DE_MENSAGEM_VIA_SERIAL)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo.selecao_de_mensagem_via_serial,
                        Options::ligado_desligado(),
                        30,
                    )
                    .build(),
            ),

            _ => None,
        }
    }
}

////////////////////////////////////////////////////

pub struct MenuIntertravamentoParaDoisEixos<'a> {
    model: &'a MachineModel,
}

impl<'a> MenuIntertravamentoParaDoisEixos<'a> {
    pub const fn new(model: &'a MachineModel) -> Self {
        Self { model }
    }
}

impl SubmenuLayout for MenuIntertravamentoParaDoisEixos<'_> {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        match index {
            0 => Some(
                MenuItemBuilder::from_text(&ANTECIPACAO_DA_SAIDA_DE_START)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.antecipacao_da_saida_de_start,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),

            1 => Some(
                MenuItemBuilder::from_text(&SAIDA_DE_START_NO_AVANCO)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo.saida_de_start_no_avaco,
                        Options::ligado_desligado(),
                        30,
                    )
                    .build(),
            ),

            2 => Some(
                MenuItemBuilder::from_text(&SAIDA_DE_START_NO_RETORNO)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo.saida_de_start_no_retorno,
                        Options::ligado_desligado(),
                        30,
                    )
                    .build(),
            ),

            3 => Some(
                MenuItemBuilder::from_text(&ENTRADA_DE_START_ENTRE_EIXOS)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo.entrada_de_start_entre_eixos,
                        Options::ligado_desligado(),
                        30,
                    )
                    .build(),
            ),

            4 => Some(
                MenuItemBuilder::from_text(&RETARDO_DO_START_ENTRE_EIXOS)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.retardo_do_start_entre_eixos,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),

            5 => Some(
                MenuItemBuilder::from_text(&START_PELO_TECLADO_E_EXTERNO)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo.start_pelo_teclado_e_externo,
                        Options::ligado_desligado(),
                        30,
                    )
                    .build(),
            ),

            6 => Some(
                MenuItemBuilder::from_text(&RETARDO_NO_START_PASSO_A_PASSO)
                    .add_numerical_variable(
                        &self.model.arquivo_de_eixo.retardo_no_start_passo_a_passo,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),

            7 => Some(
                MenuItemBuilder::from_text(&START_AUTOMATICO_PASSO_A_PASSO)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo.start_automatico_passo_a_passo,
                        Options::ligado_desligado(),
                        30,
                    )
                    .build(),
            ),

            8 => Some(
                MenuItemBuilder::from_text(&SAIDA_START_PASSO_A_PASSO)
                    .add_optional_variable(
                        &self.model.arquivo_de_eixo.saida_de_start_passo_a_passo,
                        Options::continuo_passo_a_passo(),
                        30,
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
    model: &'a MachineModel,
}

impl<'a> MenuConfiguracaoDeEixo<'a> {
    pub const fn new(model: &'a MachineModel) -> Self {
        Self { model }
    }
}

impl SubmenuLayout for MenuConfiguracaoDeEixo<'_> {
    fn get_item(&self, index: usize) -> Option<MenuItemWidget> {
        match index {
            0 => Some(
                MenuItemBuilder::from_text(&NUMERO_DO_CANAL_X)
                    .add_numerical_variable(
                        &self.model.configuracao_do_eixo.numero_do_canal,
                        Some(0..99),
                        33,
                    )
                    .build(),
            ),

            1 => Some(
                MenuItemBuilder::from_text(&NUMERO_DE_PULSO_DO_GIRO_X)
                    .add_numerical_variable(
                        &self.model.configuracao_do_eixo.numero_de_pulso_do_giro,
                        Some(0..0xFFFF),
                        33,
                    )
                    .build(),
            ),

            2 => Some(
                MenuItemBuilder::from_text(&JANELA_DE_PROTECAO_DO_GITO_X)
                    .add_numerical_variable(
                        &self.model.configuracao_do_eixo.janela_de_protecao_do_giro,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),

            3 => Some(
                MenuItemBuilder::from_text(&DESLOCAMENTO_GIRO_DO_MOTOR_X)
                    .add_optional_variable(
                        &self.model.configuracao_do_eixo.deslocamento_giro_do_motor,
                        Options::ligado_desligado(),
                        30,
                    )
                    .build(),
            ),

            4 => Some(
                MenuItemBuilder::from_text(&GIRO_COM_FUNCAO_DE_PROTECAO)
                    .add_optional_variable(
                        &self.model.configuracao_do_eixo.giro_com_funcao_de_protecao,
                        Options::ligado_desligado(),
                        30,
                    )
                    .build(),
            ),

            5 => Some(
                MenuItemBuilder::from_text(&GIRO_COM_FUNCAO_DE_CORRECAO)
                    .add_optional_variable(
                        &self.model.configuracao_do_eixo.giro_com_funcao_de_correcao,
                        Options::ligado_desligado(),
                        30,
                    )
                    .build(),
            ),

            6 => Some(
                MenuItemBuilder::from_text(&LOGICA_DO_START_EXTERNO)
                    .add_optional_variable(
                        &self.model.configuracao_do_eixo.logica_do_start_externo,
                        Options::aberto_fechado(),
                        30,
                    )
                    .build(),
            ),

            7 => Some(
                MenuItemBuilder::from_text(&VALOR_DA_POSICAO_DA_REFERENCIA)
                    .add_numerical_variable(
                        &self
                            .model
                            .configuracao_do_eixo
                            .valor_da_posicao_de_referencia,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),

            8 => Some(
                MenuItemBuilder::from_text(&VELOCIDADE_PARA_REFERENCIA_X)
                    .add_numerical_variable(
                        &self.model.configuracao_do_eixo.velocidade_para_referencia,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),

            9 => Some(
                MenuItemBuilder::from_text(&ACELERACAO_PARA_REFERENCIA_X)
                    .add_numerical_variable(
                        &self.model.configuracao_do_eixo.aceleracao_para_referencia,
                        Some(0..9999),
                        33,
                    )
                    .build(),
            ),

            10 => Some(
                MenuItemBuilder::from_text(&REDUCAO_DA_CORRENTE_EM_REPOUSO)
                    .add_optional_variable(
                        &self
                            .model
                            .configuracao_do_eixo
                            .reducao_da_corrente_em_repouso,
                        Options::ligado_desligado(),
                        30,
                    )
                    .build(),
            ),

            11 => Some(
                MenuItemBuilder::from_text(&REFERENCIA_PELO_START_EXTERNO)
                    .add_optional_variable(
                        &self
                            .model
                            .configuracao_do_eixo
                            .referencia_pelo_start_externo,
                        Options::ligado_desligado(),
                        30,
                    )
                    .build(),
            ),

            12 => Some(
                MenuItemBuilder::from_text(&MODO_TURBO_X)
                    .add_optional_variable(
                        &self.model.configuracao_do_eixo.modo_turbo,
                        Options::ligado_desligado(),
                        30,
                    )
                    .build(),
            ),

            _ => None,
        }
    }
}
