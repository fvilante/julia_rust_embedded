use avr_progmem::progmem;

progmem! {

    // MENU PROGRAMA

    pub static progmem string NUMERO_DO_PROGRAMA_PARA_EDICAO =  "Numero do programa para edicao  : ##";
    pub static progmem string NUMERO_DO_PROGRAMA_DO_EIXO_X =    "Numero do programa do eixo X    : ##";
    pub static progmem string NUMERO_DO_PROGRAMA_DO_EIXO_Y =    "Numero do programa do eixo Y    : ##";
    pub static progmem string COPIAR_O_PROGRAMA_NUMERO =        "Copiar o programa numero : ## para: ##";
    pub static progmem string TROCA_DE_NIVEL_DE_ACCESSO =       "Troca do nivel de acesso     [******** ";
    pub static progmem string CONFIGURACAO_DO_EIXO_X =          "Configuracao do eixo X :      [******* ";
    pub static progmem string CONFIGURACAO_DO_EIXO_Y =          "Configuracao do eixo Y :      [******* ";
    pub static progmem string CONFIGURACAO_DO_EQUIPAMENTO =     "Configuracao do equipamento ...";
    pub static progmem string ROTINAS_DE_TESTES_E_VERIFICACAO = "Rotinas de testes e verificacao ...";


    //ARQUIVO DE EIXO

    pub static progmem string PARAMETROS_DE_MOVIMENTO = "Parametro de Movimento...";
    pub static progmem string PARAMETROS_DE_IMPRESSAO = "Parametros de Impressao...";
    pub static progmem string CONFIGURACAO_DO_CICLO = "Configuracao do Ciclo...";
    pub static progmem string CONFIGURACAO_DA_IMPRESSORA = "Configuracao da impressora...";
    pub static progmem string INTERTRAVAMENTO_DOIS_EIXOS_PASSO_A_PASSO = "Intertravamento: dois eixos e pas/pas..";
    // NAO ACHEI A LINHA ABAIXO NO CODIGO FONTE DO CPU-Z80, VOU DEIXAR PARA VERIFICAR ISTO NO TESTE EM LAB
    //pub static progmem string PARAMETROS_SELECAO_DE_MENSAGEM = "Parametros de Selecao de mensagem...";

    // PARAMETROS DE MOVIMENTO

    pub static progmem string POSICAO_INICIAL = "Posicao inicial";
    pub static progmem string POSICAO_FINAL = "Posicao final";
    pub static progmem string ACELERACAO_DE_AVANCO = "Aceleracao de avanco";
    pub static progmem string ACELERACAO_DE_RETORNO = "Aceleracao de retorno";
    pub static progmem string VELOCIDADE_DE_AVANCO = "Velocidade de avanco";
    pub static progmem string VELOCIDADE_DE_RETORNO = "Velocidade de retorno";

    // PARAMETROS DE IMPRESSAO

    pub static progmem string NUMERO_DE_MENSAGEM_NO_AVANCO = "Numero de mensagem no avanco";
    pub static progmem string NUMERO_DE_MENSAGEM_NO_RETORNO= "Numero de mensagem no retorno";
    pub static progmem string PRIMEIRO_MENSAGEM_NO_AVANCO= "Primeira mensagem no avanco";
    pub static progmem string PRIMEIRO_MENSAGEM_NO_RETORNO= "Primeira mensagem no retorno";
    pub static progmem string ULTIMA_MENSAGEM_NO_AVANCO = "Ultima mensagem no avanco";
    pub static progmem string ULTIMA_MENSAGEM_NO_RETORNO = "Ultima mensagem no retorno";
    pub static progmem string MENSAGEM_REVERSA_LIGADA = "Mensagem reversa ligada";
    pub static progmem string NUMERO_DE_MULTIPLAS_IMPRESSOES = "Numero de multiplas impressoes";
    pub static progmem string PASSO_DAS_MULTIPLAS_IMPRESSOES = "passo das multiplas impressoes";

    // CONFIGURACAO DE CICLO

    pub static progmem string RETARDO_NO_START_AUTOMATICO = "Retardo no start automatico";
    pub static progmem string RETARDO_NO_START_EXTERNO = "Retardo no start externo";
    pub static progmem string START_AUTOMATICO_NO_AVANCO = "Start automatico no avanco";
    pub static progmem string START_AUTOMATICO_NO_RETORNO = "Start automatico no retorno";
    pub static progmem string MODO_DE_TRABALHO_DO_EIXO = "Modo de trabalho do eixo";

    // CONFIGURACAO DA IMPRESSORA

    pub static progmem string LOGICA_DO_SINAL_DE_IMPRESSAO = "Logica do sinal de impressao";
    pub static progmem string LARGURA_DO_SINAL_DE_IMPRESSAO = "Largura do sinal dimpressao";
    pub static progmem string REVERSAO_DE_MENSAGEM_VIA_SERIAL = "Reversao dmensagem via serial";
    pub static progmem string SELECAO_DE_MENSAGEM_VIA_SERIAL = "Selecao de mensagem via serial";

    // INTERTRAVAMENTO PARA DOIS EIXOS

    pub static progmem string ANTECIPACAO_DA_SAIDA_DE_START = "Antecipacao da saida de start";
    pub static progmem string SAIDA_DE_START_NO_AVANCO = "Saida de Start no avanco";
    pub static progmem string SAIDA_DE_START_NO_RETORNO = "Saida de Start no retorno";
    pub static progmem string ENTRADA_DE_START_ENTRE_EIXOS = "Entrada de start entre eixos";
    pub static progmem string RETARDO_DO_START_ENTRE_EIXOS = "Retardo do start entre eixo";
    pub static progmem string START_PELO_TECLADO_E_EXTERNO = "Start pelo teclado e externo";
    pub static progmem string RETARDO_NO_SINAL_DE_IMPRESSAO = "Retardo no sinal de impressao";
    pub static progmem string RETARDO_NO_START_PASSO_A_PASSO = "Retardo no start passo/passo";
    pub static progmem string START_AUTOMATICO_PASSO_A_PASSO = "Start automatico passo/passo";
    pub static progmem string SAIDA_START_PASSO_A_PASSO = "Saida de start passo a passo";

    // MENU CONFIGURACAO DE EIXO

    pub static progmem string NUMERO_DO_CANAL_X = "Numero do canal X               ##     ";
    pub static progmem string NUMERO_DE_PULSO_DO_GIRO_X = "Numero de pulso do giro X       ####   ";
    pub static progmem string JANELA_DE_PROTECAO_DO_GITO_X = "Janela de protecao do giro X    ####   ";
    pub static progmem string DESLOCAMENTO_GIRO_DO_MOTOR_X = "Deslocamento/Giro do Motor X  [******* ";
    pub static progmem string GIRO_COM_FUNCAO_DE_PROTECAO = "Giro X com funcao de protecao [******* ";
    pub static progmem string GIRO_COM_FUNCAO_DE_CORRECAO = "Giro X com funcao de correcao [******* ";
    pub static progmem string LOGICA_DO_START_EXTERNO = "Logica do start externo X     [******* ";
    pub static progmem string VALOR_DA_POSICAO_DA_REFERENCIA = "Valor da posicao da referencia  ####   ";
    pub static progmem string VELOCIDADE_PARA_REFERENCIA_X = "Velocidade para referencia X    ####   ";
    pub static progmem string ACELERACAO_PARA_REFERENCIA_X = "Aceleracao para referencia X    ####   ";
    pub static progmem string REDUCAO_DA_CORRENTE_EM_REPOUSO = "Reducao da corrente em repouso[******* ";
    pub static progmem string REFERENCIA_PELO_START_EXTERNO = "Referencia pelo start externo [******* ";
    pub static progmem string MODO_TURBO_X = "Modo Turbo X                  [******* ";


    // OPTIONAL PARAMETER'S TEXT

    pub static progmem string LIGADO = "Ligado";
    pub static progmem string DESLIGADO = "Deslig";
    pub static progmem string CONTINUO = "Contin";
    pub static progmem string PASSO_A_PASSO = "PasPas";
    pub static progmem string ABERTO = "Aberto";
    pub static progmem string FECHADO = "fechado";


    //NOTE: it is possible to load any type in progmem not only strings
    pub static progmem TABLE_01: [u8; 6] = [0,1,2,3,4,5];
    pub static progmem TABLE_02: [u8; 1] = [
        0
    ];
    pub static progmem string ERRO_01 = "Erro de construcao de string";
}
