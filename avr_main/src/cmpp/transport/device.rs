use crate::{board::lcd, cmpp::datalink::transact::DatalinkError};

pub enum AddressMask {
    ByteLow,
    ByteHigh,
    Word,
    Bit0,
    Bit1,
    Bit2,
    Bit3,
    Bit4,
    Bit5,
    Bit6,
    Bit7,
    Bit8,
    Bit9,
    Bit10,
    Bit11,
    Bit12,
    Bit13,
    Bit14,
    Bit15, 
}

pub struct Address {
    word_address: u8, // cmd
    address_mask: AddressMask,
}


pub enum Choice {
    OpenedClosed,
    OnOff,
    ContinumPassToPass
}

pub enum DataType {
    Displacement,
    Duration,
    Velocity,
    Acceleration,
    Quantity,
    Choice(Choice),
    //
    Pulses  // exemple: "Janela de protecao do giro"
}




// represents a single cmpp parameter 
pub struct Parameter {
    index: Index,
    address: Address,
    data_type: DataType, 
}

// Programa de Eixo
pub enum Index {
    // programa de eixo
    PosicaoInicial,
    PosicaoFinal,
    AceleracaoDeAvanco,
    AceleracaoDeRetorno,
    VelocidadeDeAvanco,
    VelocidadeDeRetorno,
    NumeroDeMensagensNoAvanco,
    NumeroDeMensagensNoRetorno,
    PosicaoDaPrimeiraImpressaoNoAvanco,
    PosicaoDaPrimeiraImpressaoNoRetorno,
    PosicaoDaUltimaImpressaoNoAvanco,
    PosicaoDaUltimaImpressaoNoRetorno,
    LogicaDeSinalDeReversao,
    LogicaDeSinalDeImpressao,
    LarguraDoSinalDeImpressao,
    ReversaoDeMensagemViaSerial,
    SelecaoDeMensagemViaSerial,
    TempoParaStartAutomatico,
    TempoParaStartExterno,
    StartAutomaticoNoAvanco,
    StartAutomaticoNoRetorno,
    ModoContinuoPassoAPasso,
    AntecipacaoDaSaidaDeStart,
    SaidaDeStartNoAvanco,
    SaidaDeStartNoRetorno,
    EntradaDeStartEntreEixos,
    ReferenciaPeloStartExterno,
    // configuracao de eixo
    NumeroDePulsosPorGiroDoMotor,
    JanelaDeProtecaoParaOGiro,
    GiroComFuncaoDeProtecao,
    GiroComFuncaoDeCorrecao,
    LogicaDeStartExterno,
    ValorDaPosicaoDeReferencia,
    VelocidadeDeReferencia,
    AceleracaoDeReferencia,
    ReducaoDaCorrenteDeRepouso,
    EntradaDeStartExterno,
    //
    LastElement,
}

const MAX_SIZE: usize = Index::LastElement as usize;

pub const CASTING: [Parameter; MAX_SIZE ] = [
    Parameter { 
        index: Index::PosicaoInicial, 
        address: Address {
            word_address: 0x50,
            address_mask: AddressMask::Word,
        },
        data_type: DataType::Displacement,
    },

    Parameter { 
        index: Index::PosicaoFinal, 
        address: Address {
            word_address: 0x51,
            address_mask: AddressMask::Word,
        },
        data_type: DataType::Displacement,
    },

    Parameter { 
        index: Index::AceleracaoDeAvanco, 
        address: Address {
            word_address: 0x52,
            address_mask: AddressMask::Word,
        },
        data_type: DataType::Acceleration,
    },

    Parameter { 
        index: Index::AceleracaoDeRetorno, 
        address: Address {
            word_address: 0x53,
            address_mask: AddressMask::Word,
        },
        data_type: DataType::Acceleration,
    },

    Parameter { 
        index: Index::VelocidadeDeAvanco, 
        address: Address {
            word_address: 0x54,
            address_mask: AddressMask::Word,
        },
        data_type:  DataType::Velocity,
    },

    Parameter { 
        index: Index::VelocidadeDeRetorno, 
        address: Address {
            word_address: 0x55,
            address_mask: AddressMask::Word,
        },
        data_type:  DataType::Velocity,
    },
    
    //PARAMETROS DE IMPRESSAO

    Parameter { 
        index: Index::NumeroDeMensagensNoAvanco, 
        address: Address {
            word_address: 0x56,
            address_mask: AddressMask::ByteHigh,
        },
        data_type: DataType::Quantity,
    },

    Parameter { 
        index: Index::NumeroDeMensagensNoRetorno, 
        address: Address {
            word_address: 0x56,
            address_mask: AddressMask::ByteLow,
        },
        data_type: DataType::Quantity,
    },

    Parameter { 
        index: Index::PosicaoDaPrimeiraImpressaoNoAvanco, 
        address: Address {
            word_address: 0x57,
            address_mask: AddressMask::Word,
        },
        data_type: DataType::Displacement,
    },

    Parameter { 
        index: Index::PosicaoDaPrimeiraImpressaoNoRetorno, 
        address: Address {
            word_address: 0x58,
            address_mask: AddressMask::Word,
        },
        data_type: DataType::Displacement,
    },

    Parameter { 
        index: Index::PosicaoDaUltimaImpressaoNoAvanco, 
        address: Address {
            word_address: 0x59,
            address_mask: AddressMask::Word,
        },
        data_type: DataType::Displacement,
    },

    Parameter { 
        index: Index::PosicaoDaUltimaImpressaoNoRetorno, 
        address: Address {
            word_address: 0x5A,
            address_mask: AddressMask::Word,
        },
        data_type: DataType::Displacement,
    },

    Parameter { 
        index: Index::LogicaDeSinalDeReversao, 
        address: Address {
            word_address: 0x60,
            address_mask: AddressMask::Bit9,
        },
        data_type: DataType::Quantity,
    },

    //PARAMETROS DA IMPRESSORA

    Parameter { 
        index: Index::LogicaDeSinalDeImpressao, 
        address: Address {
            word_address: 0x60,
            address_mask: AddressMask::Bit8,
        },
        data_type: DataType::Choice(Choice::OpenedClosed),
    },

    Parameter { 
        index: Index::LarguraDoSinalDeImpressao, 
        address: Address {
            word_address: 0x5B,
            address_mask: AddressMask::Word,
        },
        data_type: DataType::Duration,
    },

    Parameter { 
        index: Index::ReversaoDeMensagemViaSerial, 
        address: Address {
            word_address: 0x60,
            address_mask: AddressMask::Bit11,
        },
        data_type: DataType::Choice(Choice::OnOff),
    },

    Parameter { 
        index: Index::SelecaoDeMensagemViaSerial, 
        address: Address {
            word_address: 0x60,
            address_mask: AddressMask::Bit10,
        },
        data_type: DataType::Choice(Choice::OnOff),
    },

    //PARAMETROS DE CICLO

    Parameter { 
        index: Index::TempoParaStartAutomatico, 
        address: Address {
            word_address: 0x5C,
            address_mask: AddressMask::Word,
        },
        data_type: DataType::Duration,
    },

    Parameter { 
        index: Index::TempoParaStartExterno, 
        address: Address {
            word_address: 0x5D,
            address_mask: AddressMask::Word,
        },
        data_type: DataType::Duration,
    },

    Parameter { 
        index: Index::StartAutomaticoNoAvanco, 
        address: Address {
            word_address: 0x60,
            address_mask: AddressMask::Bit0,
        },
        data_type: DataType::Choice(Choice::OnOff),
    },

    Parameter { 
        index: Index::StartAutomaticoNoRetorno, 
        address: Address {
            word_address: 0x60,
            address_mask: AddressMask::Bit1,
        },
        data_type: DataType::Choice(Choice::OnOff),
    },

    Parameter { 
        index: Index::ModoContinuoPassoAPasso, 
        address: Address {
            word_address: 0x60,
            address_mask: AddressMask::Bit15,
        },
        data_type: DataType::Choice(Choice::ContinumPassToPass),
    },

    //INTERTRAVAMENTO PARA DOIS EIXOS

    Parameter { 
        index: Index::AntecipacaoDaSaidaDeStart, 
        address: Address {
            word_address: 0x5E,
            address_mask: AddressMask::Word,
        },
        data_type: DataType::Displacement,
    },

    Parameter { 
        index: Index::SaidaDeStartNoAvanco, 
        address: Address {
            word_address: 0x60,
            address_mask: AddressMask::Bit2,
        },
        data_type: DataType::Choice(Choice::OnOff),
    },

    Parameter { 
        index: Index::SaidaDeStartNoRetorno, 
        address: Address {
            word_address: 0x60,
            address_mask: AddressMask::Bit3,
        },
        data_type: DataType::Choice(Choice::OnOff),
    },

    Parameter { 
        index: Index::EntradaDeStartEntreEixos, 
        address: Address {
            word_address: 0x60,
            address_mask: AddressMask::Bit6,
        },
        data_type: DataType::Choice(Choice::OnOff),
    },

    Parameter { 
        index: Index::ReferenciaPeloStartExterno, 
        address: Address {
            word_address: 0x60,
            address_mask: AddressMask::Bit7,
        },
        data_type: DataType::Choice(Choice::OnOff),
    },

    // CONFIGURACAO DE EIXO

    //
    //Numero do Canal
    //

    Parameter { 
        index: Index::NumeroDePulsosPorGiroDoMotor, 
        address: Address {
            word_address: 0x62,
            address_mask: AddressMask::Word,
        },
        data_type: DataType::Displacement, // better than Quantity?
    },

    Parameter { 
        index: Index::JanelaDeProtecaoParaOGiro, 
        address: Address {
            word_address: 0x61,
            address_mask: AddressMask::Word,
        },
        data_type:  DataType::Pulses, // better than Quantity?
    },

    //
    //Deslocamento/Giro do Motor 
    //

    Parameter { 
        index: Index::GiroComFuncaoDeProtecao, 
        address: Address {
            word_address: 0x60,
            address_mask: AddressMask::Bit2,
        },
        data_type: DataType::Choice(Choice::OnOff),
    },

    Parameter { 
        index: Index::GiroComFuncaoDeCorrecao, 
        address: Address {
            word_address: 0x60,
            address_mask: AddressMask::Bit13,
        },
        data_type: DataType::Choice(Choice::OnOff),
    },

    Parameter { 
        index: Index::LogicaDeStartExterno, 
        address: Address {
            word_address: 0x60,
            address_mask: AddressMask::Bit5,
        },
        data_type: DataType::Choice(Choice::OpenedClosed),
    },

    Parameter { 
        index: Index::ValorDaPosicaoDeReferencia, 
        address: Address {
            word_address: 0x63,
            address_mask: AddressMask::Word,
        },
        data_type: DataType::Displacement, // better than Quantity?
    },

    Parameter { 
        index: Index::VelocidadeDeReferencia, 
        address: Address {
            word_address: 0x65,
            address_mask: AddressMask::Word,
        },
        data_type: DataType::Velocity, // better than Quantity?
    },

    Parameter { 
        index: Index::AceleracaoDeReferencia, 
        address: Address {
            word_address: 0x64,
            address_mask: AddressMask::Word,
        },
        data_type: DataType::Acceleration, // better than Quantity?
    },

    Parameter { 
        index: Index::ReducaoDaCorrenteDeRepouso, 
        address: Address {
            word_address: 0x60,
            address_mask: AddressMask::Bit14,
        },
        data_type: DataType::Choice(Choice::OnOff),
    },

    Parameter { 
        index: Index::EntradaDeStartExterno, 
        address: Address {
            word_address: 0x60,
            address_mask: AddressMask::Bit4,
        },
        data_type: DataType::Choice(Choice::OnOff),
    },

];


pub fn development_entry_point() -> ! {
    lcd::lcd_initialize();
    lcd::print("oi");

    loop { }
}