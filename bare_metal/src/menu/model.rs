use core::cell::Cell;

use cross_platform::{protocol::transport::transport_layer::TransportLayer, utils::cursor::Cursor};

use crate::{fatal_error, microcontroler::eeprom::EepromAddress};

///

/// Implamented by some object that can be serialized to be written and read in EEPROM
///
/// TODO: Consider to move this type to a better place
trait EepromStorable {
    /// Signature is used to inform that the eeprom is correctly initialized.
    ///
    /// When microcontroler is flashed first time, the eeprom is erased and is in an invalid state
    /// we use this signature to inform that the block of eeprom data is initialized
    const SIGNATURE: u16;
    /// Given initial address, write data and return next available address and size written in bytes
    /// TODO: KNOWN-ISSUES: EepromAddress currently only address first 255 bytes of eeprom
    fn save_into_eeprom(&self, initial_address: EepromAddress) -> (EepromAddress, u8);
    /// Given an initial address load data from eeprom in itself and return next address available
    /// and the size of bytes read
    /// /// TODO: KNOWN-ISSUES: only address first 255 bytes of eeprom
    fn load_from_eeprom(&mut self, initial_address: EepromAddress) -> (EepromAddress, u8);
}

///

pub struct ArquivoDeEixo {
    // PARAMETROS DE MOVIMENTO
    pub posicao_inicial: u16,
    pub posicao_final: u16,
    pub aceleracao_de_avanco: u16,
    pub aceleracao_de_retorno: u16,
    pub velocidade_de_avanco: u16,
    pub velocidade_de_retorno: u16,
    // PARAMETROS DE IMPRESSAO
    pub numero_de_mensagem_no_avanco: u16, // TODO: When possible may change to u8
    pub numero_de_mensagem_no_retorno: u16, // TODO: When possible may change to u8
    pub primeira_mensagem_no_avanco: u16,
    pub ultima_mensagem_no_avanco: u16,
    pub primeira_mensagem_no_retorno: u16,
    pub ultima_mensagem_no_retorno: u16,
    // PARAMETROS DE IMPRESSAO
    pub logica_do_sinal_de_impressao: Cursor,
    pub largura_do_sinal_de_impressao: u16,
    pub reversao_de_mensagem_via_serial: Cursor,
    pub selecao_de_mensagem_via_serial: Cursor,
    // PARAMETROS DE CICLO
    pub retardo_no_start_automatico: u16,
    pub retardo_no_start_externo: u16,
    pub start_automatico_no_avanco: Cursor,
    pub start_automatico_no_retorno: Cursor,
    pub modo_de_trabalho_do_eixo: Cursor,
    // INTERTRAVAMENTO ENTRE DOIS EIXOS
    pub antecipacao_da_saida_de_start: u16,
    pub saida_de_start_no_avaco: Cursor, // TODO: Correct typo `avaco` to `avanco`
    pub saida_de_start_no_retorno: Cursor,
    pub entrada_de_start_entre_eixos: Cursor,
    pub retardo_do_start_entre_eixos: u16,
    pub start_pelo_teclado_e_externo: Cursor,
    pub retardo_no_sinal_de_impressao: u16,
    pub retardo_no_start_passo_a_passo: u16,
    pub start_automatico_passo_a_passo: Cursor,
    pub saida_de_start_passo_a_passo: Cursor,
}

impl EepromStorable for ArquivoDeEixo {
    const SIGNATURE: u16 = 0xA000;

    fn save_into_eeprom(&self, initial_address: EepromAddress) -> (EepromAddress, u8) {
        let next = initial_address
            .write_u16(Self::SIGNATURE)
            .write_u16(self.posicao_inicial)
            .write_u16(self.posicao_final)
            .write_u16(self.aceleracao_de_avanco)
            .write_u16(self.aceleracao_de_retorno)
            .write_u16(self.velocidade_de_avanco)
            .write_u16(self.velocidade_de_retorno)
            .write_u16(self.numero_de_mensagem_no_avanco)
            .write_u16(self.numero_de_mensagem_no_retorno)
            .write_u16(self.primeira_mensagem_no_avanco)
            .write_u16(self.ultima_mensagem_no_avanco)
            .write_u16(self.primeira_mensagem_no_retorno)
            .write_u16(self.ultima_mensagem_no_retorno)
            .write_cursor(self.logica_do_sinal_de_impressao)
            .write_u16(self.largura_do_sinal_de_impressao)
            .write_cursor(self.reversao_de_mensagem_via_serial)
            .write_cursor(self.selecao_de_mensagem_via_serial)
            .write_u16(self.retardo_no_start_automatico)
            .write_u16(self.retardo_no_start_externo)
            .write_cursor(self.start_automatico_no_avanco)
            .write_cursor(self.start_automatico_no_retorno)
            .write_cursor(self.modo_de_trabalho_do_eixo)
            .write_u16(self.antecipacao_da_saida_de_start)
            .write_cursor(self.saida_de_start_no_avaco)
            .write_cursor(self.saida_de_start_no_retorno)
            .write_cursor(self.entrada_de_start_entre_eixos)
            .write_u16(self.retardo_do_start_entre_eixos)
            .write_cursor(self.start_pelo_teclado_e_externo)
            .write_u16(self.retardo_no_sinal_de_impressao)
            .write_u16(self.retardo_no_start_passo_a_passo)
            .write_cursor(self.start_automatico_passo_a_passo)
            .write_cursor(self.saida_de_start_passo_a_passo);

        let size_of_bytes_written = next.0 - initial_address.0;
        (next, size_of_bytes_written)
    }

    /// Given an initial address load data from eeprom in itself and return next address available
    /// and the size of bytes read
    /// /// TODO: KNOWN-ISSUES: only address first 255 bytes of eeprom
    fn load_from_eeprom(&mut self, initial_address: EepromAddress) -> (EepromAddress, u8) {
        let next = initial_address;
        let (signature, next) = next.read_u16();

        let signature_is_valid = signature == Self::SIGNATURE;

        if signature_is_valid {
            let (value, next) = next.read_u16();
            self.posicao_inicial = value;

            let (value, next) = next.read_u16();
            self.posicao_final = value;

            let (value, next) = next.read_u16();
            self.aceleracao_de_avanco = value;

            let (value, next) = next.read_u16();
            self.aceleracao_de_retorno = value;

            let (value, next) = next.read_u16();
            self.velocidade_de_avanco = value;

            let (value, next) = next.read_u16();
            self.velocidade_de_retorno = value;

            let (value, next) = next.read_u16();
            self.numero_de_mensagem_no_avanco = value;

            let (value, next) = next.read_u16();
            self.numero_de_mensagem_no_retorno = value;

            let (value, next) = next.read_u16();
            self.primeira_mensagem_no_avanco = value;

            let (value, next) = next.read_u16();
            self.ultima_mensagem_no_avanco = value;

            let (value, next) = next.read_u16();
            self.primeira_mensagem_no_retorno = value;

            let (value, next) = next.read_u16();
            self.ultima_mensagem_no_retorno = value;

            let (value, next) = next.read_cursor();
            self.logica_do_sinal_de_impressao = value;

            let (value, next) = next.read_u16();
            self.largura_do_sinal_de_impressao = value;

            let (value, next) = next.read_cursor();
            self.reversao_de_mensagem_via_serial = value;

            let (value, next) = next.read_cursor();
            self.selecao_de_mensagem_via_serial = value;

            let (value, next) = next.read_u16();
            self.retardo_no_start_automatico = value;

            let (value, next) = next.read_u16();
            self.retardo_no_start_externo = value;

            let (value, next) = next.read_cursor();
            self.start_automatico_no_avanco = value;

            let (value, next) = next.read_cursor();
            self.start_automatico_no_retorno = value;

            let (value, next) = next.read_cursor();
            self.modo_de_trabalho_do_eixo = value;

            let (value, next) = next.read_u16();
            self.antecipacao_da_saida_de_start = value;

            let (value, next) = next.read_cursor();
            self.saida_de_start_no_avaco = value;

            let (value, next) = next.read_cursor();
            self.saida_de_start_no_retorno = value;

            let (value, next) = next.read_cursor();
            self.entrada_de_start_entre_eixos = value;

            let (value, next) = next.read_u16();
            self.retardo_do_start_entre_eixos = value;

            let (value, next) = next.read_cursor();
            self.start_pelo_teclado_e_externo = value;

            let (value, next) = next.read_u16();
            self.retardo_no_sinal_de_impressao = value;

            let (value, next) = next.read_u16();
            self.retardo_no_start_passo_a_passo = value;

            let (value, next) = next.read_cursor();
            self.start_automatico_passo_a_passo = value;

            let (value, next) = next.read_cursor();
            self.saida_de_start_passo_a_passo = value;

            //
            let size_of_bytes_loadded = next.0 - initial_address.0;
            (next, size_of_bytes_loadded)
        } else {
            // EEPROM is not initialized yet
            // Then initialize it.

            Self::default().save_into_eeprom(initial_address);
            self.load_from_eeprom(initial_address)
        }
    }
}

impl Default for ArquivoDeEixo {
    fn default() -> Self {
        Self {
            posicao_inicial: 50,
            posicao_final: 600,
            aceleracao_de_avanco: 5000,
            aceleracao_de_retorno: 5000,
            velocidade_de_avanco: 8,
            velocidade_de_retorno: 8,
            numero_de_mensagem_no_avanco: 3,
            numero_de_mensagem_no_retorno: 3,
            primeira_mensagem_no_avanco: 200,
            ultima_mensagem_no_avanco: 400,
            primeira_mensagem_no_retorno: 400,
            ultima_mensagem_no_retorno: 200,
            logica_do_sinal_de_impressao: Default::default(),
            largura_do_sinal_de_impressao: 10,
            reversao_de_mensagem_via_serial: Default::default(),
            selecao_de_mensagem_via_serial: Default::default(),
            retardo_no_start_automatico: 10,
            retardo_no_start_externo: 10,
            start_automatico_no_avanco: Cursor::new(0, 2, 1),
            start_automatico_no_retorno: Cursor::new(0, 2, 1),
            modo_de_trabalho_do_eixo: Default::default(),
            antecipacao_da_saida_de_start: 50,
            saida_de_start_no_avaco: Default::default(),
            saida_de_start_no_retorno: Default::default(),
            entrada_de_start_entre_eixos: Default::default(),
            retardo_do_start_entre_eixos: 50,
            start_pelo_teclado_e_externo: Default::default(),
            retardo_no_sinal_de_impressao: 10,
            retardo_no_start_passo_a_passo: 50,
            start_automatico_passo_a_passo: Default::default(),
            saida_de_start_passo_a_passo: Default::default(),
        }
    }
}

// ********************************************************

pub struct ConfiguracaoDoEixo {
    pub numero_do_canal: u16,
    pub numero_de_pulso_do_giro: u16,
    pub janela_de_protecao_do_giro: u16,
    pub deslocamento_giro_do_motor: u16,
    pub giro_com_funcao_de_protecao: Cursor,
    pub giro_com_funcao_de_correcao: Cursor,
    pub logica_do_start_externo: Cursor,
    pub valor_da_posicao_de_referencia: u16,
    pub velocidade_para_referencia: u16,
    pub aceleracao_para_referencia: u16,
    pub reducao_da_corrente_em_repouso: Cursor,
    pub referencia_pelo_start_externo: Cursor,
    pub modo_turbo: Cursor,
}

impl Default for ConfiguracaoDoEixo {
    fn default() -> Self {
        Self {
            numero_do_canal: 0,
            numero_de_pulso_do_giro: 400,
            janela_de_protecao_do_giro: 50,
            deslocamento_giro_do_motor: 8100,
            giro_com_funcao_de_protecao: Default::default(),
            giro_com_funcao_de_correcao: Default::default(),
            logica_do_start_externo: Default::default(),
            valor_da_posicao_de_referencia: 50,
            velocidade_para_referencia: 500,
            aceleracao_para_referencia: 5000,
            reducao_da_corrente_em_repouso: Default::default(),
            referencia_pelo_start_externo: Default::default(),
            modo_turbo: Default::default(),
        }
    }
}

impl EepromStorable for ConfiguracaoDoEixo {
    const SIGNATURE: u16 = 0xB000;

    fn save_into_eeprom(&self, initial_address: EepromAddress) -> (EepromAddress, u8) {
        let next = initial_address
            .write_u16(Self::SIGNATURE)
            .write_u16(self.numero_do_canal)
            .write_u16(self.numero_de_pulso_do_giro)
            .write_u16(self.janela_de_protecao_do_giro)
            .write_u16(self.deslocamento_giro_do_motor)
            .write_cursor(self.giro_com_funcao_de_protecao)
            .write_cursor(self.giro_com_funcao_de_correcao)
            .write_cursor(self.logica_do_start_externo)
            .write_u16(self.valor_da_posicao_de_referencia)
            .write_u16(self.velocidade_para_referencia)
            .write_u16(self.aceleracao_para_referencia)
            .write_cursor(self.reducao_da_corrente_em_repouso)
            .write_cursor(self.referencia_pelo_start_externo)
            .write_cursor(self.modo_turbo);

        let size_of_bytes_written = next.0 - initial_address.0;
        (next, size_of_bytes_written)
    }

    fn load_from_eeprom(&mut self, initial_address: EepromAddress) -> (EepromAddress, u8) {
        let next = initial_address;
        let (signature, next) = next.read_u16();

        let signature_is_valid = signature == Self::SIGNATURE;

        if signature_is_valid {
            let (value, next) = next.read_u16();
            self.numero_do_canal = value;

            let (value, next) = next.read_u16();
            self.numero_de_pulso_do_giro = value;

            let (value, next) = next.read_u16();
            self.janela_de_protecao_do_giro = value;

            let (value, next) = next.read_u16();
            self.deslocamento_giro_do_motor = value;

            let (value, next) = next.read_cursor();
            self.giro_com_funcao_de_protecao = value;

            let (value, next) = next.read_cursor();
            self.giro_com_funcao_de_correcao = value;

            let (value, next) = next.read_cursor();
            self.logica_do_start_externo = value;

            let (value, next) = next.read_u16();
            self.valor_da_posicao_de_referencia = value;

            let (value, next) = next.read_u16();
            self.velocidade_para_referencia = value;

            let (value, next) = next.read_u16();
            self.aceleracao_para_referencia = value;

            let (value, next) = next.read_cursor();
            self.reducao_da_corrente_em_repouso = value;

            let (value, next) = next.read_cursor();
            self.referencia_pelo_start_externo = value;

            let (value, next) = next.read_cursor();
            self.modo_turbo = value;

            //
            let size_of_bytes_loadded = next.0 - initial_address.0;
            (next, size_of_bytes_loadded)
        } else {
            // EEPROM is not initialized yet
            // Then initialize it.

            Self::default().save_into_eeprom(initial_address);
            self.load_from_eeprom(initial_address)
        }
    }
}

///

pub struct ConfiguracaoDoEquipamento {
    pub velocidade_de_comunicacao: Cursor, // 0 => 2400, 1 => 9600
}

impl Default for ConfiguracaoDoEquipamento {
    fn default() -> Self {
        Self {
            velocidade_de_comunicacao: Cursor::new(0, 2, 0),
        }
    }
}

impl EepromStorable for ConfiguracaoDoEquipamento {
    const SIGNATURE: u16 = 0x0C00;

    fn save_into_eeprom(&self, initial_address: EepromAddress) -> (EepromAddress, u8) {
        let next = initial_address
            .write_u16(Self::SIGNATURE)
            .write_cursor(self.velocidade_de_comunicacao);

        let size_of_bytes_written = next.0 - initial_address.0;
        (next, size_of_bytes_written)
    }

    fn load_from_eeprom(&mut self, initial_address: EepromAddress) -> (EepromAddress, u8) {
        let next = initial_address;
        let (signature, next) = next.read_u16();

        let signature_is_valid = signature == Self::SIGNATURE;

        if signature_is_valid {
            let (value, next) = next.read_cursor();
            self.velocidade_de_comunicacao = value;

            //
            let size_of_bytes_loaded = next.0 - initial_address.0;
            (next, size_of_bytes_loaded)
        } else {
            // EEPROM is not initialized yet
            // Then initialize it.

            Self::default().save_into_eeprom(initial_address);
            self.load_from_eeprom(initial_address)
        }
    }
}

pub struct GuiState {
    /// TODO: Should use u8 instead of u16 here.
    pub numero_do_programa_do_eixo_x: u16,
}

impl Default for GuiState {
    fn default() -> Self {
        Self {
            numero_do_programa_do_eixo_x: 0,
        }
    }
}

///

pub struct DataModel {
    pub arquivo_de_eixo_00: ArquivoDeEixo,
    pub arquivo_de_eixo_01: ArquivoDeEixo,
    //pub arquivo_de_eixo_y: ArquivoDeEixo,
    pub configuracao_do_eixo_x: ConfiguracaoDoEixo,
    pub configuracao_do_eixo_y: ConfiguracaoDoEixo,
    pub configuracao_do_equipamento: ConfiguracaoDoEquipamento,
    // parametros the GUI (graphical user interface)
    pub gui_state: GuiState,
}

impl DataModel {
    const ADDR_LOW: u8 = 0x00;
    const ADDR_HIGH: u8 = 0x01;

    pub fn new() -> Self {
        Self {
            arquivo_de_eixo_00: ArquivoDeEixo::default(),
            arquivo_de_eixo_01: ArquivoDeEixo::default(),
            //arquivo_de_eixo_y: ArquivoDeEixo::default(),
            configuracao_do_eixo_x: ConfiguracaoDoEixo::default(),
            configuracao_do_eixo_y: ConfiguracaoDoEixo::default(),
            configuracao_do_equipamento: ConfiguracaoDoEquipamento::default(),
            gui_state: GuiState::default(),
        }
    }

    /// Start address to store `Arquivo de Eixo` data in eeprom
    fn get_initial_eeprom_address(&self) -> EepromAddress {
        match self.numero_do_programa_atual() {
            // TODO: Currently addresses is arbitrarily defined, change this for a more
            // formal and efficient approuch;
            0 => EepromAddress(0x00),
            1 => EepromAddress(100),
            // TODO: Make this error not being a Fatal Error but a recoverable error
            _ => fatal_error!(0x7F), // Invalid number of arquivo_de_eixo
        }
    }

    fn numero_do_programa_atual(&self) -> u16 {
        self.gui_state.numero_do_programa_do_eixo_x
    }

    pub fn get_arquivo_de_eixo_by_ref(&self) -> &ArquivoDeEixo {
        match self.numero_do_programa_atual() {
            0 => &self.arquivo_de_eixo_00,
            1 => &self.arquivo_de_eixo_01,
            // TODO: Make this error not being a Fatal Error but a recoverable error
            _ => fatal_error!(0x7F), // Invalid number of arquivo_de_eixo
        }
    }

    pub fn get_arquivo_de_eixo_by_ref_mut(&mut self) -> &mut ArquivoDeEixo {
        let numero_do_programa = self.gui_state.numero_do_programa_do_eixo_x;
        match numero_do_programa {
            0 => &mut self.arquivo_de_eixo_00,
            1 => &mut self.arquivo_de_eixo_01,
            // TODO: Make this error not being a Fatal Error but a recoverable error
            _ => fatal_error!(0x7F), // Invalid number of arquivo_de_eixo
        }
    }

    /// Saves data to EEPROM
    pub fn save_to_eeprom(&self) {
        let initial_address = self.get_initial_eeprom_address();
        let (next, _size) = self
            .get_arquivo_de_eixo_by_ref()
            .save_into_eeprom(initial_address);

        let (next, _size) = self.configuracao_do_eixo_x.save_into_eeprom(next);
        let (_next, _size) = self.configuracao_do_equipamento.save_into_eeprom(next);
    }

    /// loads data from EEPROM
    pub fn load_from_eeprom(&mut self) {
        let initial_address = self.get_initial_eeprom_address();
        let (next, _address) = self
            .get_arquivo_de_eixo_by_ref_mut()
            .load_from_eeprom(initial_address);
        let (next, _size) = self.configuracao_do_eixo_x.load_from_eeprom(next);
        let (_next, _size) = self.configuracao_do_equipamento.load_from_eeprom(next);
    }
}

/////

pub struct CmppData<'a> {
    pub arquivo_de_eixo: &'a ArquivoDeEixo,
    pub configuracao_de_eixo: &'a ConfiguracaoDoEixo,
}

/// TODO: Eventually this function should be place in a better location instead of in this module.
pub fn send_all(transport: &TransportLayer, data: &CmppData) {
    let CmppData {
        arquivo_de_eixo,
        configuracao_de_eixo,
    } = data;

    transport
        .posicao_inicial()
        .set(arquivo_de_eixo.posicao_inicial.into());

    transport
        .posicao_final()
        .set(arquivo_de_eixo.posicao_final.into());

    transport
        .aceleracao_de_avanco()
        .set(arquivo_de_eixo.aceleracao_de_avanco.into());

    transport
        .aceleracao_de_retorno()
        .set(arquivo_de_eixo.aceleracao_de_retorno.into());

    transport
        .velocidade_de_avanco()
        .set(arquivo_de_eixo.velocidade_de_avanco.into());

    transport
        .velocidade_de_retorno()
        .set(arquivo_de_eixo.velocidade_de_retorno.into());

    transport
        .numero_de_mensagem_no_avanco()
        .set(arquivo_de_eixo.numero_de_mensagem_no_avanco.into());

    transport
        .numero_de_mensagem_no_retorno()
        .set(arquivo_de_eixo.numero_de_mensagem_no_retorno.into());

    transport
        .primeira_mensagem_no_avanco()
        .set(arquivo_de_eixo.primeira_mensagem_no_avanco.into());

    transport
        .ultima_mensagem_no_avanco()
        .set(arquivo_de_eixo.ultima_mensagem_no_avanco.into());

    transport
        .primeira_mensagem_no_retorno()
        .set(arquivo_de_eixo.primeira_mensagem_no_retorno.into());

    transport
        .ultima_mensagem_no_retorno()
        .set(arquivo_de_eixo.ultima_mensagem_no_retorno.into());

    transport
        .logica_do_sinal_de_impressao()
        .set(arquivo_de_eixo.logica_do_sinal_de_impressao.into());

    transport
        .largura_do_sinal_de_impressao()
        .set(arquivo_de_eixo.largura_do_sinal_de_impressao.into());

    transport
        .reversao_de_mensagem_via_serial()
        .set(arquivo_de_eixo.reversao_de_mensagem_via_serial.into());

    transport
        .selecao_de_mensagem_via_serial()
        .set(arquivo_de_eixo.selecao_de_mensagem_via_serial.into());

    transport
        .retardo_no_start_automatico()
        .set(arquivo_de_eixo.retardo_no_start_automatico.into());

    transport
        .retardo_no_start_externo()
        .set(arquivo_de_eixo.retardo_no_start_externo.into());

    transport
        .start_automatico_no_avanco()
        .set(arquivo_de_eixo.start_automatico_no_avanco.into());

    transport
        .start_automatico_no_retorno()
        .set(arquivo_de_eixo.start_automatico_no_retorno.into());

    transport
        .modo_de_trabalho_do_eixo()
        .set(arquivo_de_eixo.modo_de_trabalho_do_eixo.into());

    transport
        .antecipacao_da_saida_de_start()
        .set(arquivo_de_eixo.antecipacao_da_saida_de_start.into());

    transport
        .saida_de_start_no_avaco()
        .set(arquivo_de_eixo.saida_de_start_no_avaco.into());

    transport
        .saida_de_start_no_retorno()
        .set(arquivo_de_eixo.saida_de_start_no_retorno.into());

    transport
        .entrada_de_start_entre_eixos()
        .set(arquivo_de_eixo.entrada_de_start_entre_eixos.into());

    transport
        .retardo_do_start_entre_eixos()
        .set(arquivo_de_eixo.retardo_do_start_entre_eixos.into());

    transport
        .start_pelo_teclado_e_externo()
        .set(arquivo_de_eixo.start_pelo_teclado_e_externo.into());

    transport
        .retardo_no_sinal_de_impressao()
        .set(arquivo_de_eixo.retardo_no_sinal_de_impressao.into());

    transport
        .retardo_no_start_passo_a_passo()
        .set(arquivo_de_eixo.retardo_no_start_passo_a_passo.into());

    transport
        .start_automatico_passo_a_passo()
        .set(arquivo_de_eixo.start_automatico_passo_a_passo.into());

    transport
        .saida_de_start_passo_a_passo()
        .set(arquivo_de_eixo.saida_de_start_passo_a_passo.into());

    transport
        .janela_de_protecao_do_giro()
        .set(configuracao_de_eixo.janela_de_protecao_do_giro.into());

    transport
        .deslocamento_giro_do_motor()
        .set(configuracao_de_eixo.deslocamento_giro_do_motor.into());

    transport
        .giro_com_funcao_de_protecao()
        .set(configuracao_de_eixo.giro_com_funcao_de_protecao.into());

    transport
        .giro_com_funcao_de_correcao()
        .set(configuracao_de_eixo.giro_com_funcao_de_correcao.into());

    transport
        .logica_do_start_externo()
        .set(configuracao_de_eixo.logica_do_start_externo.into());

    transport
        .valor_da_posicao_de_referencia()
        .set(configuracao_de_eixo.valor_da_posicao_de_referencia.into());

    transport
        .velocidade_para_referencia()
        .set(configuracao_de_eixo.velocidade_para_referencia.into());

    transport
        .aceleracao_para_referencia()
        .set(configuracao_de_eixo.aceleracao_para_referencia.into());

    transport
        .reducao_da_corrente_em_repouso()
        .set(configuracao_de_eixo.reducao_da_corrente_em_repouso.into());

    transport
        .referencia_pelo_start_externo()
        .set(configuracao_de_eixo.referencia_pelo_start_externo.into());
}
