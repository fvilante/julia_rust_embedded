
/// *******************************************************************************
/// IMPORTANTE: Este codigo fonte é temporario e ele representa um trabalho inicial
/// de procurar reproduzir em linhas bem gerais a mesma ideia de especificacao do
/// menu Z80 em assembly porem em RUST. A ideia nao é portar o codigo, mas apenas 
/// compreender como o menu é espeficidado no Assembly, a fim de buscar reproduzir
/// um efeito (do ponto de vista do usuario) semelhante.
/// 
/// # Objetivo
/// 
/// O objetivo deste documento é avaliar o comportamento do Menu TTC3100 classico, 
/// de modo a poder reproduzir o seu comportamento.
/// 
/// 
/// # Qual é a estrutura literal de textos do menu?
/// 
/// 
/// Vou transferir do codigo fonte posijet, para um arquivo fonte em rust. 
/// (segue abaixo)
/// 
/// NOTA: Esta especificacao abaixo é um esboço e pode conter coisas desatualizadas
/// ou equivocadas. Utilize-o sobre sua conta e risco.
/// 
/// ******************************************************************************

enum NivelDeProtecao {
    Supervisor, 
    Manutencao,
    Operador,   
    Protegido,       
}

// Endereco serial dos parametros da placa do motor
struct WAddr(u8)


/// "Rotina de chamada de Menu"
/// FIXME: Como passar parametros para estas funcoes?
enum RotinaDeChamada {
    REDICAO, // usado em "Numero do programa para edicao"
    EXECUX, // usado em "Numero do programa do eixo X"
    EXECUY,
    COPIPRG, // "copiar programa numero ## para ##" //FIXME: Verificar se é tipo um MenuFunction::Selpar porem com 2 fields
    SUBMENX,    // "Configuracao do eixo X"
    SUBMENY,
    SEQUIPO, // "Configuracao do equipamento..."
    SUBMENT, // "Rotinas de testes e verificacao ..."
    SELPAR,  // Quando é um MITEM simples com um field apenas
}

struct RotinaDeConversores {
    Retorna, /// É apenas uma funcao de retorno (identidade), sem modificar o valor recebido
    ConvMm,
    ConvMm0,
    ConvSeg,
}

struct OptionalText {
    options_text: &str
}


/// -------------- TEXTO ORIGINAL DO CODIGO Z80 -------------------------------
/// ;BYTES DA MENSAGEM DE PARAMETROS INDEXADO POR IX
/// ;00-39 = Mensagem
/// ;40    = 0Dh
/// ;41    = Vago
/// ;42,43 = Rotina principal de chamada (normalmente teclado)	SELPARM
/// ;44,45 = Rotina de conversao	COTNVMM,CONVMM0,CONVSEG
/// ;46,47 = Endereco inicial do parametro na ram de usuario;
/// ;      = Ou endereco do submenu
/// ;48,49 = Endereco do parametro na ram de systema
/// ;50,51 = Ponteiro da mensagem de opcao
/// ----------------------------------------------------------------------------
/// ;Convencao dos simbolos utilizado na mascara
/// ;# = SO NUMERO				    @ = NUMERO OU PONTO DECIMAL	;
/// ;$ = HEXADECIMAL SEM PONTO		* = OPCAO			;
/// ;& = SUBSTITUICAO			    . = PONTO DECIMAL		;
/// ;---------------------------------------------------------------------------
struct SubmenuItem<'a> {
    mensagem: [u8;40], 
    // 0Dh
    waddr: Option<Waddr>, /// ;Endereco serial dos parametros da placa do motor
    rotina_de_chamada: RotinaDeChamada,
    conversor: RotinaDeConversores, /// FIXME: Aparentemente converte o valor das unidades de medida do usuario para unidades de medida da maquina.
    protection_level: ProtectionLevel,
    //-> texto_ascii: not_implemented!, /// In CPU_Z80_Board there is a lot of memory available, so the ascii_buffer was put here. But in Julia implementation we will construct the ascii char, from the value referencia just-in-time to improve memory performance.  
    variable: &'a Cell<u16>,
    optional: Option<OptionalText>,
}

enum NumericalBound {
    UpperBound(u16), // lower bound defaulted to zero in this case
    LowerAndUpperBound(u16, u16),
}

struct OptionalDetails;

enum OptionalBound {
    LigaDesliga,
    AbertoFechado,
    ContinuoPassoAPasso,
    Custom(OptionalDetails),
}



struct FieldKind {
    Numerical(NumericalBound),
    Optional(OptionalBound),
}

// Defines the specification of a particular Field of a particular SubMenuItem 
struct SubMenuItemField {
    kind: FieldKind,
    rotina: RotinaDeConversores // FIXME: Nao sei o significado disto

}

fn juca() {

    const TPALAR:u16 = 1000; /// Tempo do bip alarme (milisegundos)

    // Endereco serial dos parametros da placa do motor
    let sEeArL  = Waddr(0x1F);	// Equ	1Fh ;(=31)	;(3Eh)	/// EeProm Address register Low
    let sEeDr   = Waddr(0x1E);	// Equ	1Eh ;(=30)	;(3Dh)	/// EeProm Data register
    let sEeCr   = Waddr(0x1E);	// Equ	1Eh ;(=30)	;(3Ch)	/// EeProm Control register
    let SPosAtu = Waddr(0x30);	// Equ	30h ;(=48)	;(60h)	/// Posicao atual
    let sMaskEr = Waddr(0x45);	// Equ	45h ;(=69)	;(8Ah)	/// Macara de erro
    let sStatus = Waddr(0x49);	// Equ	49h ;(=73)	;(92h)	/// Macara de erro
    let sPosIni = Waddr(0x50);	// Equ	50h ;(=80)	;(A0h)	/// Posicao inicial programada
    let sPosFim = Waddr(0x51);	// Equ	51h ;(=81)	;(A2h)	/// Posicao final programada
    let sAcevan = Waddr(0x52);	// Equ	52h ;(=82)	;(A4h)	/// Aceleracao de avanco programada
    let sAceret = Waddr(0x53);	// Equ	53h ;(=83)	;(A6h)	/// Aceleracao de retorno programada
    let sVelAva = Waddr(0x54);	// Equ	54h ;(=84)	;(A8h)	/// Velocidade de avanco programada
    let sVelRet = Waddr(0x55);	// Equ	55h ;(=85)	;(AAh)	/// Velocidade de retorno programada
    let snMenAv = Waddr(0x56);	// Equ	56h ;(=86)	;(ACh)	/// Numero de mensagem no avanco
    let snMenRt = Waddr(0x56);	// Equ	56h ;(=86)	;(ADh)	/// Numero de mensagem no retorno
    let spPrtAv = Waddr(0x57);	// Equ	57h ;(=87)	;(AEh)	/// posicao da primeira impressao no avanco
    let spPrtRt = Waddr(0x58);	// Equ	58h ;(=88)	;(B0h)	/// posicao da primeira impressao no retorno
    let sUprtAV = Waddr(0x59);	// Equ	59h ;(=89)	;(B2h)	/// Posicao da ultima mensagem no avanco
    let sUprtRT = Waddr(0x5A);	// Equ	5Ah ;(=90)	;(B4h)	/// Posicao da ultima mensagem no retorno
    let slaSiPr = Waddr(0x5B);	// Equ	5Bh ;(=91)	;(B6h)	/// Largura do sinal de impressao
    let sStaAut = Waddr(0x5C);	// Equ	5Ch ;(=92)	;(B8h)	/// Tempo para o start automatico
    let sStaExt = Waddr(0x5D);	// Equ	5Dh ;(=93)	;(BAh)	/// Tempo para o start externo
    let sCotAnt = Waddr(0x5E);	// Equ	5Eh ;(=94)	;(BCh)	/// Cota de antecipacao do start entre eixos (pinelmatico)
    let sRetApp = Waddr(0x5F);	// Equ	5Fh ;(=95)	;(BEh)	/// Retardo para o start automatico passo a passo
    let sFlgPrg = Waddr(0x60);	// Equ	60h ;(=96)	;(C0h)	/// Bits de configura�ao do programa
    let sRetImp = Waddr(0x61);	// Equ	61h ;(=97)	;(C2h)	/// Retardo para o sinal de impressao
    let sPtaco  = Waddr(0x62);	// Equ	62h ;(=98)	;(C4h)	/// Divisor programado do taco
    let sJanela = Waddr(0x63);	// Equ	63h ;(=99)	;(C6h)	/// Janela de prote��o
    let snPulso = Waddr(0x64);	// Equ	64h ;(=100)	;(C8h)	/// Numero de pulso por Volta
    let sValRef = Waddr(0x65);	// Equ	65h ;(=101)	;(CAh)	/// Valor da referencia
    let sAceRef = Waddr(0x66);	// Equ	66h ;(=102)	;(CCh)	/// Acelera��o da referencia
    let sVelRef = Waddr(0x67);	// Equ	67h ;(=103)	;(CEh)	/// Velocidade da referencia
    let sFlgPrE = Waddr(0x68);	// Equ	68h ;(=104)	;(D0h)	/// Flags de controle especial
    let sPmotor = Waddr(0x68);	// Equ	68h ;(=104)	;(D1h)	/// Divisor programado do motor
    let sComSer = Waddr(0x69);	// Equ	69h ;(=105)	;(D2h)  /// Registro de controle via serial


    const normal: ProtectionLevel = {
        admin: AccessType::Modify,
        maintanance: AccessType::Modify,
        operation: AccessType::Modify,
        visitor: AccessType::ViewOnly,
        field_spec: FieldSpec,
    }

    let numero_do_programa_para_edicao = SubmenuItem {
        //     1234567890123456789012345678901234567890
        text: " Numero do programa para edicao  : ##   ",
        waddr: None,
        next_menu: Some(MenuId::MenuEdicao), //FIXME - HOW TO SEND THE PARAMETER ABOUT WHAT 'ARQUIVO DE EIXO' MUST BE EDITED? Apparently an Enumeration with payload should work, but who will receive and how it will be transmited? It seems that the widget would receive a reference to the MenuControler and use this reference do comand this communication like a man in the middle
        protection_level: normal,
    };

    let numero_do_programa_do_eixo_X = SubmenuItem {
        //     1234567890123456789012345678901234567890
        text: " Numero do programa do eixo X    : ##   ",
        waddr: None,
        next_menu: Some(MenuId::MenuEdicao), //FIXME - HOW TO SEND THE PARAMETER ABOUT WHAT 'ARQUIVO DE EIXO' MUST BE EDITED? Apparently an Enumeration with payload should work, but who will receive and how it will be transmited? It seems that the widget would receive a reference to the MenuControler and use this reference do comand this communication like a man in the middle
        protection_level: normal,
    };



}
