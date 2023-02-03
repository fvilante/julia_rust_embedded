use core::cell::Cell;

use lib_1::utils::cursor::Cursor;

pub struct ArquivoDeEixo {
    // PARAMETROS DE MOVIMENTO
    pub posicao_inicial: Cell<u16>,
    // pub posicao_final: Cell<u16>,
    // pub aceleracao_de_avanco: Cell<u16>,
    // pub aceleracao_de_retorno: Cell<u16>,
    // pub velocidade_de_avanco: Cell<u16>,
    // pub velocidade_de_retorno: Cell<u16>,
    // PARAMETROS DE IMPRESSAO
    // pub numero_de_mensagem_no_avanco: Cell<u16>,
    // pub numero_de_mensagem_no_retorno: Cell<u16>,
    // pub primeira_mensagem_no_avanco: Cell<u16>,
    // pub ultima_mensagem_no_avanco: Cell<u16>,
    // pub primeira_mensagem_no_retorno: Cell<u16>,
    // pub ultima_mensagem_no_retorno: Cell<u16>,
    // // PARAMETROS DE IMPRESSAO
    // pub logica_do_sinal_de_impressao: Cell<Cursor>,
    // pub largura_do_sinal_de_impressao: Cell<u16>,
    // pub reversao_de_mensagem_via_serial: Cell<Cursor>,
    // pub selecao_de_mensagem_via_serial: Cell<Cursor>,
    // // PARAMETROS DE CICLO
    // pub retardo_no_start_automatico: Cell<u16>,
    // pub retardo_no_start_externo: Cell<u16>,
    // pub start_automatico_no_avanco: Cell<Cursor>,
    // pub start_automatico_no_retorno: Cell<Cursor>,
    // pub modo_de_trabalho_do_eixo: Cell<Cursor>,
    // // INTERTRAVAMENTO ENTRE DOIS EIXOS
    // pub antecipacao_da_saida_de_start: Cell<u16>,
    // pub saida_de_start_no_avaco: Cell<Cursor>,
    // pub saida_de_start_no_retorno: Cell<Cursor>,
    // pub entrada_de_start_entre_eixos: Cell<Cursor>,
    // pub retardo_do_start_entre_eixos: Cell<u16>,
    // pub start_pelo_teclado_e_externo: Cell<Cursor>,
    // pub retardo_no_sinal_de_impressao: Cell<u16>,
    // pub retardo_no_start_passo_a_passo: Cell<u16>,
    // pub start_automatico_passo_a_passo: Cell<Cursor>,
    // pub saida_de_start_passo_a_passo: Cell<Cursor>,
}

impl Default for ArquivoDeEixo {
    fn default() -> Self {
        Self {
            posicao_inicial: Default::default(),
            //posicao_final: Default::default(),
            //aceleracao_de_avanco: Default::default(),
            //aceleracao_de_retorno: Default::default(),
            //velocidade_de_avanco: Default::default(),
            //velocidade_de_retorno: Default::default(),
            //numero_de_mensagem_no_avanco: Default::default(),
            //numero_de_mensagem_no_retorno: Default::default(),
            //primeira_mensagem_no_avanco: Default::default(),
            //ultima_mensagem_no_avanco: Default::default(),
            //primeira_mensagem_no_retorno: Default::default(),
            //ultima_mensagem_no_retorno: Default::default(),
            //logica_do_sinal_de_impressao: Default::default(),
            //largura_do_sinal_de_impressao: Default::default(),
            //reversao_de_mensagem_via_serial: Default::default(),
            //selecao_de_mensagem_via_serial: Default::default(),
            //retardo_no_start_automatico: Default::default(),
            //retardo_no_start_externo: Default::default(),
            //start_automatico_no_avanco: Default::default(),
            //start_automatico_no_retorno: Default::default(),
            //modo_de_trabalho_do_eixo: Default::default(),
            //antecipacao_da_saida_de_start: Default::default(),
            //saida_de_start_no_avaco: Default::default(),
            //saida_de_start_no_retorno: Default::default(),
            //entrada_de_start_entre_eixos: Default::default(),
            //retardo_do_start_entre_eixos: Default::default(),
            //start_pelo_teclado_e_externo: Default::default(),
            //retardo_no_sinal_de_impressao: Default::default(),
            //retardo_no_start_passo_a_passo: Default::default(),
            //start_automatico_passo_a_passo: Default::default(),
            //saida_de_start_passo_a_passo: Default::default(),
        }
    }
}

pub struct ConfiguracaoDoEixo {
    pub numero_do_canal: Cell<u8>,
    //pub numero_de_pulso_do_giro: Cell<u16>,
    //pub janela_de_protecao_do_giro: Cell<u16>,
    //pub deslocamento_giro_do_motor: Cell<Cursor>,
    //pub giro_com_funcao_de_protecao: Cell<Cursor>,
    //pub giro_com_funcao_de_correcao: Cell<Cursor>,
    //pub logica_do_start_externo: Cell<Cursor>,
    //pub valor_da_posicao_de_referencia: Cell<Cursor>,
    //pub velocidade_para_referencia: Cell<u16>,
    //pub aceleracao_para_referencia: Cell<u16>,
    //pub reducao_da_corrente_em_repouso: Cell<Cursor>,
    //pub referencia_pelo_start_externo: Cell<Cursor>,
    //pub modo_turbo: Cell<Cursor>,
}

impl Default for ConfiguracaoDoEixo {
    fn default() -> Self {
        Self {
            numero_do_canal: Default::default(),
            //numero_de_pulso_do_giro: Default::default(),
            //janela_de_protecao_do_giro: Default::default(),
            //deslocamento_giro_do_motor: Default::default(),
            //giro_com_funcao_de_protecao: Default::default(),
            //giro_com_funcao_de_correcao: Default::default(),
            //logica_do_start_externo: Default::default(),
            //valor_da_posicao_de_referencia: Default::default(),
            //velocidade_para_referencia: Default::default(),
            //aceleracao_para_referencia: Default::default(),
            //reducao_da_corrente_em_repouso: Default::default(),
            //referencia_pelo_start_externo: Default::default(),
            //modo_turbo: Default::default(),
        }
    }
}

pub struct MachineModel {
    pub arquivo_de_eixo: ArquivoDeEixo,
    pub configuracao_do_eixo: ConfiguracaoDoEixo,
}

impl MachineModel {
    pub fn new() -> Self {
        Self {
            arquivo_de_eixo: ArquivoDeEixo::default(),
            configuracao_do_eixo: ConfiguracaoDoEixo::default(),
        }
    }
}
