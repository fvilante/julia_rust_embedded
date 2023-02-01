use core::{cell::Cell, u8};

use avr_progmem::progmem;
use lib_1::utils::cursor::Cursor;

use super::{
    flash::FlashString,
    model::MachineModel,
    widget::{
        field::optional::{make_options_buffer_from_array, OptionsBuffer},
        menu_item::builder::MenuItemBuilder,
        menu_item::menu_item::MenuItemWidget,
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

    // CONFIGURACAO DE CICLO

    static progmem string RETARDO_NO_START_AUTOMATICO = "Retardo no start automatico";
    static progmem string RETARDO_NO_START_EXTERNO = "Retardo no start externo";
    static progmem string START_AUTOMATICO_NO_AVANCO = "Start automatico no avanco";
    static progmem string START_AUTOMATICO_NO_RETORNO = "Start automatico no retorno";
    static progmem string MODO_DE_TRABALHO_DO_EIXO = "Modo de trabalho do eixo";

    // CONFIGURACAO DA IMPRESSORA

    static progmem string LOGICA_DO_SINAL_DE_IMPRESSAO = "Logica do sinal de impressao";
    static progmem string LARGURA_DO_SINAL_DE_IMPRESSAO = "Largura do sinal dimpressao";
    static progmem string REVERSAO_DE_MENSAGEM_VIA_SERIAL = "Reversao dmensagem via serial";
    static progmem string SELECAO_DE_MENSAGEM_VIA_SERIAL = "Selecao de mensagem via serial";

    // INTERTRAVAMENTO PARA DOIS EIXOS

    static progmem string ANTECIPACAO_DA_SAIDA_DE_START = "Antecipacao da saida de start";
    static progmem string SAIDA_DE_START_NO_AVANCO = "Saida de Start no avanco";
    static progmem string SAIDA_DE_START_NO_RETORNO = "Saida de Start no retorno";
    static progmem string ENTRADA_DE_START_ENTRE_EIXOS = "Entrada de start entre eixos";
    static progmem string RETARDO_DO_START_ENTRE_EIXOS = "Retardo do start entre eixo";
    static progmem string START_PELO_TECLADO_E_EXTERNO = "Start pelo teclado e externo";
    static progmem string RETARDO_NO_SINAL_DE_IMPRESSAO = "Retardo no sinal de impressao";
    static progmem string RETARDO_NO_START_PASSO_A_PASSO = "Retardo no start passo/passo";
    static progmem string START_AUTOMATICO_PASSO_A_PASSO = "Start automatico passo/passo";
    static progmem string SAIDA_START_PASSO_A_PASSO = "Saida de start passo a passo";

    //

    static progmem string LIGADO = "Ligado";
    static progmem string DESLIGADO = "Deslig";
    static progmem string CONTINUO = "Contin";
    static progmem string PASSO_A_PASSO = "PasPas";
    static progmem string ABERTO = "Aberto";
    static progmem string FECHADO = "fechado";
    static progmem string O3 = "Juca  ";
    static progmem string O4 = "Nego  ";

    //NOTE: it is possible to load any type in progmem not only strings
    static progmem TABLE_01: [u8; 6] = [0,1,2,3,4,5];
    static progmem TABLE_02: [u8; 1] = [
        0
    ];
    static progmem string ERRO_01 = "Erro de construcao de string";
}

/// TODO: Change name to `MenuDefinition`
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

pub trait SubmenuLayout {
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

#[derive(Copy, Clone)]
pub enum SubMenuHandle {
    MenuArquivoDeEixo,
    MenuParametrosDeMovimento,
    MenuParametrosDeImpressao,
    MenuParametrosDeCiclo,
    MenuConfiguracaoDaImpressora,
    MenuIntertravamentoParaDoisEixos,
}

pub struct MenuArquivoDeEixo {}

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
