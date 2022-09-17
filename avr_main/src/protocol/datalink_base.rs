
// Protocol control characters
#[derive(PartialEq)]
pub enum ProtoControl {
    ESC = 0x1B,
    STX = 0x02,
    ACK = 0x06,
    ETX = 0x03,
    NACK = 0x15,
}

pub enum StartByte {
    STX = 0x02, 
    ACK = 0x06,
    NACK = 0x15,
}

// FIX: Improve typing cast in the future
type Direcao = u8; // just 2 bits most significatives are used
type Canal = u8; 
type Cmd = u8; // just 6 bits least significatives are used 
type DadoLow = u8;
type DadoHigh = u8;

#[derive(Copy,Clone)]
pub struct PacketBaseStructure {    
    //Main data structure of a packet of protocol
    // dataSegment = [ cmd + (direcao/canal) + dadoHigh + dadoLow ]
    pub direcao: Direcao,
    pub canal: Canal,
    pub cmd: Cmd,
    pub dado_high: DadoHigh,
    pub dado_low: DadoLow,
}


impl PacketBaseStructure {
    pub fn new() -> Self {
        Self {
            direcao: 0x00,
            canal: 0x00,
            cmd: 0x00, 
            dado_high: 0x00,
            dado_low: 0x00,
        }
    }
}



#[derive(PartialEq, Clone, Copy)]
pub enum ProtoStates {
    //States of protocol posijet1 
    InitialEsc = 0,
    StartByte,
    DirectionAndChannel,
    Command,
    DataLow,
    DataHigh,
    FinalEsc,
    EtxByte,
    Checksum,
    Sucessful,
    Error, //fix: not implmented yet
}

pub fn proto_states_to_string(state: ProtoStates) -> &'static str {
    match state {
         ProtoStates::InitialEsc =>    "ESA", 
         ProtoStates::StartByte => "STB", 
         ProtoStates::DirectionAndChannel =>  "D&C", 
         ProtoStates::Command =>    "CMD", 
         ProtoStates::DataLow =>   "DL", 
         ProtoStates::DataHigh =>  "DH", 
         ProtoStates::FinalEsc =>  "ESB", 
         ProtoStates::EtxByte =>   "ETX", 
         ProtoStates::Checksum =>   "CHK", 
         ProtoStates::Sucessful =>  "SUCESSFUL", 
         ProtoStates::Error =>  "ERROR", 
    }
}