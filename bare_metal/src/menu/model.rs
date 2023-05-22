use core::cell::Cell;

use cross_platform::{
    protocol::{
        datalink::datalink::Status,
        transport::transport_layer::{TLError, TransportLayer},
    },
    utils::cursor::Cursor,
};

use crate::microcontroler::eeprom::EepromAddress;

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
    pub posicao_inicial: Cell<u16>,
    pub posicao_final: Cell<u16>,
    pub aceleracao_de_avanco: Cell<u16>,
    pub aceleracao_de_retorno: Cell<u16>,
    pub velocidade_de_avanco: Cell<u16>,
    pub velocidade_de_retorno: Cell<u16>,
    // PARAMETROS DE IMPRESSAO
    pub numero_de_mensagem_no_avanco: Cell<u16>, // TODO: When possible may change to u8
    pub numero_de_mensagem_no_retorno: Cell<u16>, // TODO: When possible may change to u8
    pub primeira_mensagem_no_avanco: Cell<u16>,
    pub ultima_mensagem_no_avanco: Cell<u16>,
    pub primeira_mensagem_no_retorno: Cell<u16>,
    pub ultima_mensagem_no_retorno: Cell<u16>,
    // PARAMETROS DE IMPRESSAO
    pub logica_do_sinal_de_impressao: Cell<Cursor>,
    pub largura_do_sinal_de_impressao: Cell<u16>,
    pub reversao_de_mensagem_via_serial: Cell<Cursor>,
    pub selecao_de_mensagem_via_serial: Cell<Cursor>,
    // PARAMETROS DE CICLO
    pub retardo_no_start_automatico: Cell<u16>,
    pub retardo_no_start_externo: Cell<u16>,
    pub start_automatico_no_avanco: Cell<Cursor>,
    pub start_automatico_no_retorno: Cell<Cursor>,
    pub modo_de_trabalho_do_eixo: Cell<Cursor>,
    // INTERTRAVAMENTO ENTRE DOIS EIXOS
    pub antecipacao_da_saida_de_start: Cell<u16>,
    pub saida_de_start_no_avaco: Cell<Cursor>, // TODO: Correct typo `avaco` to `avanco`
    pub saida_de_start_no_retorno: Cell<Cursor>,
    pub entrada_de_start_entre_eixos: Cell<Cursor>,
    pub retardo_do_start_entre_eixos: Cell<u16>,
    pub start_pelo_teclado_e_externo: Cell<Cursor>,
    pub retardo_no_sinal_de_impressao: Cell<u16>,
    pub retardo_no_start_passo_a_passo: Cell<u16>,
    pub start_automatico_passo_a_passo: Cell<Cursor>,
    pub saida_de_start_passo_a_passo: Cell<Cursor>,
}

impl EepromStorable for ArquivoDeEixo {
    const SIGNATURE: u16 = 0xA000;

    fn save_into_eeprom(&self, initial_address: EepromAddress) -> (EepromAddress, u8) {
        let next = initial_address
            .write_u16(Self::SIGNATURE)
            .write_u16(self.posicao_inicial.get())
            .write_u16(self.posicao_final.get())
            .write_u16(self.aceleracao_de_avanco.get())
            .write_u16(self.aceleracao_de_retorno.get())
            .write_u16(self.velocidade_de_avanco.get())
            .write_u16(self.velocidade_de_retorno.get())
            .write_u16(self.numero_de_mensagem_no_avanco.get())
            .write_u16(self.numero_de_mensagem_no_retorno.get())
            .write_u16(self.primeira_mensagem_no_avanco.get())
            .write_u16(self.ultima_mensagem_no_avanco.get())
            .write_u16(self.primeira_mensagem_no_retorno.get())
            .write_u16(self.ultima_mensagem_no_retorno.get())
            .write_cursor(self.logica_do_sinal_de_impressao.get())
            .write_u16(self.largura_do_sinal_de_impressao.get())
            .write_cursor(self.reversao_de_mensagem_via_serial.get())
            .write_cursor(self.selecao_de_mensagem_via_serial.get())
            .write_u16(self.retardo_no_start_automatico.get())
            .write_u16(self.retardo_no_start_externo.get())
            .write_cursor(self.start_automatico_no_avanco.get())
            .write_cursor(self.start_automatico_no_retorno.get())
            .write_cursor(self.modo_de_trabalho_do_eixo.get())
            .write_u16(self.antecipacao_da_saida_de_start.get())
            .write_cursor(self.saida_de_start_no_avaco.get())
            .write_cursor(self.saida_de_start_no_retorno.get())
            .write_cursor(self.entrada_de_start_entre_eixos.get())
            .write_u16(self.retardo_do_start_entre_eixos.get())
            .write_cursor(self.start_pelo_teclado_e_externo.get())
            .write_u16(self.retardo_no_sinal_de_impressao.get())
            .write_u16(self.retardo_no_start_passo_a_passo.get())
            .write_cursor(self.start_automatico_passo_a_passo.get())
            .write_cursor(self.saida_de_start_passo_a_passo.get());

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
            self.posicao_inicial.set(value);

            let (value, next) = next.read_u16();
            self.posicao_final.set(value);

            let (value, next) = next.read_u16();
            self.aceleracao_de_avanco.set(value);

            let (value, next) = next.read_u16();
            self.aceleracao_de_retorno.set(value);

            let (value, next) = next.read_u16();
            self.velocidade_de_avanco.set(value);

            let (value, next) = next.read_u16();
            self.velocidade_de_retorno.set(value);

            let (value, next) = next.read_u16();
            self.numero_de_mensagem_no_avanco.set(value);

            let (value, next) = next.read_u16();
            self.numero_de_mensagem_no_retorno.set(value);

            let (value, next) = next.read_u16();
            self.primeira_mensagem_no_avanco.set(value);

            let (value, next) = next.read_u16();
            self.ultima_mensagem_no_avanco.set(value);

            let (value, next) = next.read_u16();
            self.primeira_mensagem_no_retorno.set(value);

            let (value, next) = next.read_u16();
            self.ultima_mensagem_no_retorno.set(value);

            let (value, next) = next.read_cursor();
            self.logica_do_sinal_de_impressao.set(value);

            let (value, next) = next.read_u16();
            self.largura_do_sinal_de_impressao.set(value);

            let (value, next) = next.read_cursor();
            self.reversao_de_mensagem_via_serial.set(value);

            let (value, next) = next.read_cursor();
            self.selecao_de_mensagem_via_serial.set(value);

            let (value, next) = next.read_u16();
            self.retardo_no_start_automatico.set(value);

            let (value, next) = next.read_u16();
            self.retardo_no_start_externo.set(value);

            let (value, next) = next.read_cursor();
            self.start_automatico_no_avanco.set(value);

            let (value, next) = next.read_cursor();
            self.start_automatico_no_retorno.set(value);

            let (value, next) = next.read_cursor();
            self.modo_de_trabalho_do_eixo.set(value);

            let (value, next) = next.read_u16();
            self.antecipacao_da_saida_de_start.set(value);

            let (value, next) = next.read_cursor();
            self.saida_de_start_no_avaco.set(value);

            let (value, next) = next.read_cursor();
            self.saida_de_start_no_retorno.set(value);

            let (value, next) = next.read_cursor();
            self.entrada_de_start_entre_eixos.set(value);

            let (value, next) = next.read_u16();
            self.retardo_do_start_entre_eixos.set(value);

            let (value, next) = next.read_cursor();
            self.start_pelo_teclado_e_externo.set(value);

            let (value, next) = next.read_u16();
            self.retardo_no_sinal_de_impressao.set(value);

            let (value, next) = next.read_u16();
            self.retardo_no_start_passo_a_passo.set(value);

            let (value, next) = next.read_cursor();
            self.start_automatico_passo_a_passo.set(value);

            let (value, next) = next.read_cursor();
            self.saida_de_start_passo_a_passo.set(value);

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
            posicao_inicial: Cell::new(50),
            posicao_final: Cell::new(600),
            aceleracao_de_avanco: Cell::new(5000),
            aceleracao_de_retorno: Cell::new(5000),
            velocidade_de_avanco: Cell::new(8),
            velocidade_de_retorno: Cell::new(8),
            numero_de_mensagem_no_avanco: Cell::new(3),
            numero_de_mensagem_no_retorno: Cell::new(3),
            primeira_mensagem_no_avanco: Cell::new(200),
            ultima_mensagem_no_avanco: Cell::new(400),
            primeira_mensagem_no_retorno: Cell::new(400),
            ultima_mensagem_no_retorno: Cell::new(200),
            logica_do_sinal_de_impressao: Default::default(),
            largura_do_sinal_de_impressao: Cell::new(10),
            reversao_de_mensagem_via_serial: Default::default(),
            selecao_de_mensagem_via_serial: Default::default(),
            retardo_no_start_automatico: Cell::new(10),
            retardo_no_start_externo: Cell::new(10),
            start_automatico_no_avanco: Cell::new(Cursor::new(0, 2, 1)),
            start_automatico_no_retorno: Cell::new(Cursor::new(0, 2, 1)),
            modo_de_trabalho_do_eixo: Default::default(),
            antecipacao_da_saida_de_start: Cell::new(50),
            saida_de_start_no_avaco: Default::default(),
            saida_de_start_no_retorno: Default::default(),
            entrada_de_start_entre_eixos: Default::default(),
            retardo_do_start_entre_eixos: Cell::new(50),
            start_pelo_teclado_e_externo: Default::default(),
            retardo_no_sinal_de_impressao: Cell::new(10),
            retardo_no_start_passo_a_passo: Cell::new(50),
            start_automatico_passo_a_passo: Default::default(),
            saida_de_start_passo_a_passo: Default::default(),
        }
    }
}

// ********************************************************

pub struct ConfiguracaoDoEixo {
    pub numero_do_canal: Cell<u16>,
    pub numero_de_pulso_do_giro: Cell<u16>,
    pub janela_de_protecao_do_giro: Cell<u16>,
    pub deslocamento_giro_do_motor: Cell<u16>,
    pub giro_com_funcao_de_protecao: Cell<Cursor>,
    pub giro_com_funcao_de_correcao: Cell<Cursor>,
    pub logica_do_start_externo: Cell<Cursor>,
    pub valor_da_posicao_de_referencia: Cell<u16>,
    pub velocidade_para_referencia: Cell<u16>,
    pub aceleracao_para_referencia: Cell<u16>,
    pub reducao_da_corrente_em_repouso: Cell<Cursor>,
    pub referencia_pelo_start_externo: Cell<Cursor>,
    pub modo_turbo: Cell<Cursor>,
}

impl Default for ConfiguracaoDoEixo {
    fn default() -> Self {
        Self {
            numero_do_canal: Cell::new(0),
            numero_de_pulso_do_giro: Cell::new(400),
            janela_de_protecao_do_giro: Cell::new(50),
            deslocamento_giro_do_motor: Cell::new(8100),
            giro_com_funcao_de_protecao: Default::default(),
            giro_com_funcao_de_correcao: Default::default(),
            logica_do_start_externo: Default::default(),
            valor_da_posicao_de_referencia: Cell::new(50),
            velocidade_para_referencia: Cell::new(500),
            aceleracao_para_referencia: Cell::new(5000),
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
            .write_u16(self.numero_do_canal.get())
            .write_u16(self.numero_de_pulso_do_giro.get())
            .write_u16(self.janela_de_protecao_do_giro.get())
            .write_u16(self.deslocamento_giro_do_motor.get())
            .write_cursor(self.giro_com_funcao_de_protecao.get())
            .write_cursor(self.giro_com_funcao_de_correcao.get())
            .write_cursor(self.logica_do_start_externo.get())
            .write_u16(self.valor_da_posicao_de_referencia.get())
            .write_u16(self.velocidade_para_referencia.get())
            .write_u16(self.aceleracao_para_referencia.get())
            .write_cursor(self.reducao_da_corrente_em_repouso.get())
            .write_cursor(self.referencia_pelo_start_externo.get())
            .write_cursor(self.modo_turbo.get());

        let size_of_bytes_written = next.0 - initial_address.0;
        (next, size_of_bytes_written)
    }

    fn load_from_eeprom(&mut self, initial_address: EepromAddress) -> (EepromAddress, u8) {
        let next = initial_address;
        let (signature, next) = next.read_u16();

        let signature_is_valid = signature == Self::SIGNATURE;

        if signature_is_valid {
            let (value, next) = next.read_u16();
            self.numero_do_canal.set(value);

            let (value, next) = next.read_u16();
            self.numero_de_pulso_do_giro.set(value);

            let (value, next) = next.read_u16();
            self.janela_de_protecao_do_giro.set(value);

            let (value, next) = next.read_u16();
            self.deslocamento_giro_do_motor.set(value);

            let (value, next) = next.read_cursor();
            self.giro_com_funcao_de_protecao.set(value);

            let (value, next) = next.read_cursor();
            self.giro_com_funcao_de_correcao.set(value);

            let (value, next) = next.read_cursor();
            self.logica_do_start_externo.set(value);

            let (value, next) = next.read_u16();
            self.valor_da_posicao_de_referencia.set(value);

            let (value, next) = next.read_u16();
            self.velocidade_para_referencia.set(value);

            let (value, next) = next.read_u16();
            self.aceleracao_para_referencia.set(value);

            let (value, next) = next.read_cursor();
            self.reducao_da_corrente_em_repouso.set(value);

            let (value, next) = next.read_cursor();
            self.referencia_pelo_start_externo.set(value);

            let (value, next) = next.read_cursor();
            self.modo_turbo.set(value);

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
    pub velocidade_de_comunicacao: Cell<Cursor>, // 0 => 2400, 1 => 9600
}

impl Default for ConfiguracaoDoEquipamento {
    fn default() -> Self {
        Self {
            velocidade_de_comunicacao: Cell::new(Cursor::new(0, 2, 0)),
        }
    }
}

impl EepromStorable for ConfiguracaoDoEquipamento {
    const SIGNATURE: u16 = 0x0C00;

    fn save_into_eeprom(&self, initial_address: EepromAddress) -> (EepromAddress, u8) {
        let next = initial_address
            .write_u16(Self::SIGNATURE)
            .write_cursor(self.velocidade_de_comunicacao.get());

        let size_of_bytes_written = next.0 - initial_address.0;
        (next, size_of_bytes_written)
    }

    fn load_from_eeprom(&mut self, initial_address: EepromAddress) -> (EepromAddress, u8) {
        let next = initial_address;
        let (signature, next) = next.read_u16();

        let signature_is_valid = signature == Self::SIGNATURE;

        if signature_is_valid {
            let (value, next) = next.read_cursor();
            self.velocidade_de_comunicacao.set(value);

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

pub struct DataModel {
    pub arquivo_de_eixo_x: ArquivoDeEixo,
    //pub arquivo_de_eixo_y: ArquivoDeEixo,
    pub configuracao_do_eixo_x: ConfiguracaoDoEixo,
    //pub configuracao_do_eixo_y: ConfiguracaoDoEixo,
    pub configuracao_do_equipamento: ConfiguracaoDoEquipamento,
}

impl DataModel {
    const ADDR_LOW: u8 = 0x00;
    const ADDR_HIGH: u8 = 0x01;

    /// Start address to store data in eeprom
    const INITIAL_ADDRESS: EepromAddress = EepromAddress(0x00);

    pub fn new() -> Self {
        Self {
            arquivo_de_eixo_x: ArquivoDeEixo::default(),
            //arquivo_de_eixo_y: ArquivoDeEixo::default(),
            configuracao_do_eixo_x: ConfiguracaoDoEixo::default(),
            //configuracao_do_eixo_y: ConfiguracaoDoEixo::default(),
            configuracao_do_equipamento: ConfiguracaoDoEquipamento::default(),
        }
    }

    pub fn send_all<'a>(&'a self, transport: &'a TransportLayer) -> SendAllIterator<'a> {
        let cmpp_data = CmppData {
            arquivo_de_eixo: &self.arquivo_de_eixo_x,
            configuracao_de_eixo: &self.configuracao_do_eixo_x,
        };

        SendAllIterator::new(cmpp_data, transport)
    }

    /// Saves data to EEPROM
    pub fn save_to_eeprom(&self) {
        let (next, _size) = self
            .arquivo_de_eixo_x
            .save_into_eeprom(Self::INITIAL_ADDRESS);

        let (next, _size) = self.configuracao_do_eixo_x.save_into_eeprom(next);
        let (_next, _size) = self.configuracao_do_equipamento.save_into_eeprom(next);
    }

    /// loads data from EEPROM
    pub fn load_from_eeprom(&mut self) {
        let (next, _address) = self
            .arquivo_de_eixo_x
            .load_from_eeprom(Self::INITIAL_ADDRESS);
        let (next, _size) = self.configuracao_do_eixo_x.load_from_eeprom(next);
        let (_next, _size) = self.configuracao_do_equipamento.load_from_eeprom(next);
    }
}

/////

pub struct CmppData<'a> {
    arquivo_de_eixo: &'a ArquivoDeEixo,
    configuracao_de_eixo: &'a ConfiguracaoDoEixo,
}

/// Sends all local cmpp parameters to the cmpp board (iterator)
pub struct SendAllIterator<'a> {
    // counts the current item to sent (starts at index -1 and it pre-increments counter)
    counter: i8,
    cmpp_data: CmppData<'a>,
    transport: &'a TransportLayer<'a>,
}

impl<'a> SendAllIterator<'a> {
    pub fn new(cmpp_data: CmppData<'a>, transport: &'a TransportLayer) -> Self {
        Self {
            counter: -1,
            cmpp_data,
            transport,
        }
    }
}

impl<'a> Iterator for SendAllIterator<'a> {
    type Item = Result<Status, TLError>;

    fn next(&mut self) -> Option<Self::Item> {
        let CmppData {
            arquivo_de_eixo,
            configuracao_de_eixo,
        } = self.cmpp_data;

        let transport = self.transport;

        self.counter += 1; // pre-increment

        match self.counter {
            0 => Some(
                transport
                    .posicao_inicial()
                    .set(arquivo_de_eixo.posicao_inicial.get().into()),
            ),
            1 => Some(
                transport
                    .posicao_final()
                    .set(arquivo_de_eixo.posicao_final.get().into()),
            ),
            2 => Some(
                transport
                    .aceleracao_de_avanco()
                    .set(arquivo_de_eixo.aceleracao_de_avanco.get().into()),
            ),
            3 => Some(
                transport
                    .aceleracao_de_retorno()
                    .set(arquivo_de_eixo.aceleracao_de_retorno.get().into()),
            ),
            4 => Some(
                transport
                    .velocidade_de_avanco()
                    .set(arquivo_de_eixo.velocidade_de_avanco.get().into()),
            ),
            5 => Some(
                transport
                    .velocidade_de_retorno()
                    .set(arquivo_de_eixo.velocidade_de_retorno.get().into()),
            ),
            6 => Some(
                transport
                    .numero_de_mensagem_no_avanco()
                    .set(arquivo_de_eixo.numero_de_mensagem_no_avanco.get().into()),
            ),
            7 => Some(
                transport
                    .numero_de_mensagem_no_retorno()
                    .set(arquivo_de_eixo.numero_de_mensagem_no_retorno.get().into()),
            ),
            8 => Some(
                transport
                    .primeira_mensagem_no_avanco()
                    .set(arquivo_de_eixo.primeira_mensagem_no_avanco.get().into()),
            ),
            9 => Some(
                transport
                    .ultima_mensagem_no_avanco()
                    .set(arquivo_de_eixo.ultima_mensagem_no_avanco.get().into()),
            ),
            10 => Some(
                transport
                    .primeira_mensagem_no_retorno()
                    .set(arquivo_de_eixo.primeira_mensagem_no_retorno.get().into()),
            ),
            11 => Some(
                transport
                    .ultima_mensagem_no_retorno()
                    .set(arquivo_de_eixo.ultima_mensagem_no_retorno.get().into()),
            ),
            12 => Some(
                transport
                    .logica_do_sinal_de_impressao()
                    .set(arquivo_de_eixo.logica_do_sinal_de_impressao.get().into()),
            ),
            13 => Some(
                transport
                    .largura_do_sinal_de_impressao()
                    .set(arquivo_de_eixo.largura_do_sinal_de_impressao.get().into()),
            ),
            14 => Some(
                transport
                    .reversao_de_mensagem_via_serial()
                    .set(arquivo_de_eixo.reversao_de_mensagem_via_serial.get().into()),
            ),
            15 => Some(
                transport
                    .selecao_de_mensagem_via_serial()
                    .set(arquivo_de_eixo.selecao_de_mensagem_via_serial.get().into()),
            ),
            16 => Some(
                transport
                    .retardo_no_start_automatico()
                    .set(arquivo_de_eixo.retardo_no_start_automatico.get().into()),
            ),
            17 => Some(
                transport
                    .retardo_no_start_externo()
                    .set(arquivo_de_eixo.retardo_no_start_externo.get().into()),
            ),
            18 => Some(
                transport
                    .start_automatico_no_avanco()
                    .set(arquivo_de_eixo.start_automatico_no_avanco.get().into()),
            ),
            19 => Some(
                transport
                    .start_automatico_no_retorno()
                    .set(arquivo_de_eixo.start_automatico_no_retorno.get().into()),
            ),
            20 => Some(
                transport
                    .modo_de_trabalho_do_eixo()
                    .set(arquivo_de_eixo.modo_de_trabalho_do_eixo.get().into()),
            ),
            21 => Some(
                transport
                    .antecipacao_da_saida_de_start()
                    .set(arquivo_de_eixo.antecipacao_da_saida_de_start.get().into()),
            ),
            22 => Some(
                transport
                    .saida_de_start_no_avaco()
                    .set(arquivo_de_eixo.saida_de_start_no_avaco.get().into()),
            ),
            23 => Some(
                transport
                    .saida_de_start_no_retorno()
                    .set(arquivo_de_eixo.saida_de_start_no_retorno.get().into()),
            ),
            24 => Some(
                transport
                    .entrada_de_start_entre_eixos()
                    .set(arquivo_de_eixo.entrada_de_start_entre_eixos.get().into()),
            ),
            25 => Some(
                transport
                    .retardo_do_start_entre_eixos()
                    .set(arquivo_de_eixo.retardo_do_start_entre_eixos.get().into()),
            ),
            26 => Some(
                transport
                    .start_pelo_teclado_e_externo()
                    .set(arquivo_de_eixo.start_pelo_teclado_e_externo.get().into()),
            ),
            27 => Some(
                transport
                    .retardo_no_sinal_de_impressao()
                    .set(arquivo_de_eixo.retardo_no_sinal_de_impressao.get().into()),
            ),
            28 => Some(
                transport
                    .retardo_no_start_passo_a_passo()
                    .set(arquivo_de_eixo.retardo_no_start_passo_a_passo.get().into()),
            ),
            29 => Some(
                transport
                    .start_automatico_passo_a_passo()
                    .set(arquivo_de_eixo.start_automatico_passo_a_passo.get().into()),
            ),
            30 => Some(
                transport
                    .saida_de_start_passo_a_passo()
                    .set(arquivo_de_eixo.saida_de_start_passo_a_passo.get().into()),
            ),
            31 => Some(
                transport
                    .janela_de_protecao_do_giro()
                    .set(configuracao_de_eixo.janela_de_protecao_do_giro.get().into()),
            ),
            32 => Some(
                transport
                    .deslocamento_giro_do_motor()
                    .set(configuracao_de_eixo.deslocamento_giro_do_motor.get().into()),
            ),
            33 => Some(
                transport.giro_com_funcao_de_protecao().set(
                    configuracao_de_eixo
                        .giro_com_funcao_de_protecao
                        .get()
                        .into(),
                ),
            ),
            34 => Some(
                transport.giro_com_funcao_de_correcao().set(
                    configuracao_de_eixo
                        .giro_com_funcao_de_correcao
                        .get()
                        .into(),
                ),
            ),
            35 => Some(
                transport
                    .logica_do_start_externo()
                    .set(configuracao_de_eixo.logica_do_start_externo.get().into()),
            ),
            36 => Some(
                transport.valor_da_posicao_de_referencia().set(
                    configuracao_de_eixo
                        .valor_da_posicao_de_referencia
                        .get()
                        .into(),
                ),
            ),
            37 => Some(
                transport
                    .velocidade_para_referencia()
                    .set(configuracao_de_eixo.velocidade_para_referencia.get().into()),
            ),
            38 => Some(
                transport
                    .aceleracao_para_referencia()
                    .set(configuracao_de_eixo.aceleracao_para_referencia.get().into()),
            ),
            39 => Some(
                transport.reducao_da_corrente_em_repouso().set(
                    configuracao_de_eixo
                        .reducao_da_corrente_em_repouso
                        .get()
                        .into(),
                ),
            ),
            40 => Some(
                transport.referencia_pelo_start_externo().set(
                    configuracao_de_eixo
                        .referencia_pelo_start_externo
                        .get()
                        .into(),
                ),
            ),
            _ => None,
        }
    }
}
