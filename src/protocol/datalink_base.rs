
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
type Dado = u16;
type DadoLow = u8;
type DadoHigh = u8;

#[derive(Copy,Clone)]
pub struct PacketBaseStructure {    
    //Main data structure of a packet of protocol
    // dataSegment = [ cmd + (direcao/canal) + dadoHigh + dadoLow ]
    pub direcao: Direcao,
    pub canal: Canal,
    pub cmd: Cmd,
    pub dadoHigh: DadoHigh,
    pub dadoLow: DadoLow,
}


impl PacketBaseStructure {
    pub fn new() -> Self {
        Self {
            direcao: 0x00,
            canal: 0x00,
            cmd: 0x00, 
            dadoHigh: 0x00,
            dadoLow: 0x00,
        }
    }
}


#[allow(non_camel_case_types)]
#[derive(PartialEq, Clone, Copy)]
pub enum ProtoStates {
    //States of protocol posijet1 
    INITIAL_ESC = 0,
    START_BYTE,
    DIRECTION_AND_CHANNEL,
    COMMAND,
    DATA_LOW,
    DATA_HIGH,
    FINAL_ESC,
    ETX_BYTE,
    CHECKSUM,
    SUCESSFUL,
    ERROR, //fix: not implmented yet
}

pub fn protoStates_toString(state: ProtoStates) -> &'static str {
    match state {
         ProtoStates::INITIAL_ESC =>    "ESA", 
         ProtoStates::START_BYTE => "STB", 
         ProtoStates::DIRECTION_AND_CHANNEL =>  "D&C", 
         ProtoStates::COMMAND =>    "CMD", 
         ProtoStates::DATA_LOW =>   "DL", 
         ProtoStates::DATA_HIGH =>  "DH", 
         ProtoStates::FINAL_ESC =>  "ESB", 
         ProtoStates::ETX_BYTE =>   "ETX", 
         ProtoStates::CHECKSUM =>   "CHK", 
         ProtoStates::SUCESSFUL =>  "SUCESSFUL", 
         ProtoStates::ERROR =>  "ERROR", 
    }
}