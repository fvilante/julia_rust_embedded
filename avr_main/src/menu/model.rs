use core::cell::Cell;

use lib_1::{
    protocol::{
        datalink::datalink::{word16::Word16, Status},
        transport::transport_layer::{TLError, TransportLayer},
    },
    utils::cursor::Cursor,
};

use crate::microcontroler::eeprom::EepromAddress;

pub struct ArquivoDeEixo {
    // PARAMETROS DE MOVIMENTO
    pub posicao_inicial: Cell<u16>,
    pub posicao_final: Cell<u16>,
    pub aceleracao_de_avanco: Cell<u16>,
    pub aceleracao_de_retorno: Cell<u16>,
    pub velocidade_de_avanco: Cell<u16>,
    pub velocidade_de_retorno: Cell<u16>,
    // PARAMETROS DE IMPRESSAO
    pub numero_de_mensagem_no_avanco: Cell<u16>,
    pub numero_de_mensagem_no_retorno: Cell<u16>,
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

impl ArquivoDeEixo {
    /// Signature is used to inform that the eeprom is correctly initialized.
    ///
    /// When microcontroler is flashed first time, the eeprom is erased and is in an invalid state
    /// we use this signature to inform that the block of eeprom data is initialized
    const SIGNATURE: u16 = 0xA000;

    /// Given initial address, write data and return next available address and size written in bytes
    /// TODO: KNOWN-ISSUES: only address first 255 bytes of eeprom
    fn save_into_eeprom(&self, mut initial_address: EepromAddress) -> (EepromAddress, u8) {
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
            posicao_inicial: Default::default(),
            posicao_final: Default::default(),
            aceleracao_de_avanco: Default::default(),
            aceleracao_de_retorno: Default::default(),
            velocidade_de_avanco: Default::default(),
            velocidade_de_retorno: Default::default(),
            numero_de_mensagem_no_avanco: Default::default(),
            numero_de_mensagem_no_retorno: Default::default(),
            primeira_mensagem_no_avanco: Default::default(),
            ultima_mensagem_no_avanco: Default::default(),
            primeira_mensagem_no_retorno: Default::default(),
            ultima_mensagem_no_retorno: Default::default(),
            logica_do_sinal_de_impressao: Default::default(),
            largura_do_sinal_de_impressao: Default::default(),
            reversao_de_mensagem_via_serial: Default::default(),
            selecao_de_mensagem_via_serial: Default::default(),
            retardo_no_start_automatico: Default::default(),
            retardo_no_start_externo: Default::default(),
            start_automatico_no_avanco: Default::default(),
            start_automatico_no_retorno: Default::default(),
            modo_de_trabalho_do_eixo: Default::default(),
            antecipacao_da_saida_de_start: Default::default(),
            saida_de_start_no_avaco: Default::default(),
            saida_de_start_no_retorno: Default::default(),
            entrada_de_start_entre_eixos: Default::default(),
            retardo_do_start_entre_eixos: Default::default(),
            start_pelo_teclado_e_externo: Default::default(),
            retardo_no_sinal_de_impressao: Default::default(),
            retardo_no_start_passo_a_passo: Default::default(),
            start_automatico_passo_a_passo: Default::default(),
            saida_de_start_passo_a_passo: Default::default(),
        }
    }
}

pub struct ConfiguracaoDoEixo {
    pub numero_do_canal: Cell<u16>,
    pub numero_de_pulso_do_giro: Cell<u16>,
    pub janela_de_protecao_do_giro: Cell<u16>,
    pub deslocamento_giro_do_motor: Cell<Cursor>,
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

impl ConfiguracaoDoEixo {
    const SIGNATURE: u16 = 0xB000;

    /// Given initial address, write data and return next available address and size written in bytes
    /// TODO: KNOWN-ISSUES: only address first 255 bytes of eeprom
    fn save_into_eeprom(&self, mut initial_address: EepromAddress) -> (EepromAddress, u8) {
        let next = initial_address
            .write_u16(Self::SIGNATURE)
            .write_u16(self.numero_do_canal.get());

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
            self.numero_do_canal.set(value);

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

impl Default for ConfiguracaoDoEixo {
    fn default() -> Self {
        Self {
            numero_do_canal: Default::default(),
            numero_de_pulso_do_giro: Default::default(),
            janela_de_protecao_do_giro: Default::default(),
            deslocamento_giro_do_motor: Default::default(),
            giro_com_funcao_de_protecao: Default::default(),
            giro_com_funcao_de_correcao: Default::default(),
            logica_do_start_externo: Default::default(),
            valor_da_posicao_de_referencia: Default::default(),
            velocidade_para_referencia: Default::default(),
            aceleracao_para_referencia: Default::default(),
            reducao_da_corrente_em_repouso: Default::default(),
            referencia_pelo_start_externo: Default::default(),
            modo_turbo: Default::default(),
        }
    }
}

///

pub struct MachineModel {
    pub arquivo_de_eixo_x: ArquivoDeEixo,
    //pub arquivo_de_eixo_y: ArquivoDeEixo,
    pub configuracao_do_eixo_x: ConfiguracaoDoEixo,
    //pub configuracao_do_eixo_y: ConfiguracaoDoEixo,
}

impl MachineModel {
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
        }
    }

    pub fn send_all<'a>(&'a self, transport: &'a TransportLayer) -> SendAllIterator<'a> {
        let cmpp_data = CmppData {
            arquivo_de_eixo: &self.arquivo_de_eixo_x,
            configuracao_de_eixo: &self.configuracao_do_eixo_x,
        };

        SendAllIterator::new(cmpp_data, transport)
    }

    pub fn save_to_eeprom(&self) {
        let (next, size) = self
            .arquivo_de_eixo_x
            .save_into_eeprom(Self::INITIAL_ADDRESS);

        let (next, size) = self.configuracao_do_eixo_x.save_into_eeprom(next);
    }

    pub fn load_from_eeprom(&mut self) {
        let (next, address) = self
            .arquivo_de_eixo_x
            .load_from_eeprom(Self::INITIAL_ADDRESS);
        let (next, size) = self.configuracao_do_eixo_x.load_from_eeprom(next);
    }
}

/////

pub struct CmppData<'a> {
    arquivo_de_eixo: &'a ArquivoDeEixo,
    configuracao_de_eixo: &'a ConfiguracaoDoEixo,
}

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
                    .numero_do_canal()
                    .set(configuracao_de_eixo.numero_do_canal.get().into()),
            ),
            32 => Some(
                transport
                    .numero_de_pulso_do_giro()
                    .set(configuracao_de_eixo.numero_de_pulso_do_giro.get().into()),
            ),
            33 => Some(
                transport
                    .janela_de_protecao_do_giro()
                    .set(configuracao_de_eixo.janela_de_protecao_do_giro.get().into()),
            ),
            34 => Some(
                transport
                    .deslocamento_giro_do_motor()
                    .set(configuracao_de_eixo.deslocamento_giro_do_motor.get().into()),
            ),
            35 => Some(
                transport.giro_com_funcao_de_protecao().set(
                    configuracao_de_eixo
                        .giro_com_funcao_de_protecao
                        .get()
                        .into(),
                ),
            ),
            36 => Some(
                transport.giro_com_funcao_de_correcao().set(
                    configuracao_de_eixo
                        .giro_com_funcao_de_correcao
                        .get()
                        .into(),
                ),
            ),
            37 => Some(
                transport
                    .logica_do_start_externo()
                    .set(configuracao_de_eixo.logica_do_start_externo.get().into()),
            ),
            38 => Some(
                transport.valor_da_posicao_de_referencia().set(
                    configuracao_de_eixo
                        .valor_da_posicao_de_referencia
                        .get()
                        .into(),
                ),
            ),
            39 => Some(
                transport
                    .velocidade_para_referencia()
                    .set(configuracao_de_eixo.velocidade_para_referencia.get().into()),
            ),
            40 => Some(
                transport
                    .aceleracao_para_referencia()
                    .set(configuracao_de_eixo.aceleracao_para_referencia.get().into()),
            ),
            41 => Some(
                transport.reducao_da_corrente_em_repouso().set(
                    configuracao_de_eixo
                        .reducao_da_corrente_em_repouso
                        .get()
                        .into(),
                ),
            ),
            42 => Some(
                transport.referencia_pelo_start_externo().set(
                    configuracao_de_eixo
                        .referencia_pelo_start_externo
                        .get()
                        .into(),
                ),
            ),
            43 => Some(
                transport
                    .modo_turbo()
                    .set(configuracao_de_eixo.modo_turbo.get().into()),
            ),
            _ => None,
        }
    }
}
