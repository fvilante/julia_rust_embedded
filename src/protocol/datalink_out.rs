// low-level driver for transmiting data from the cmpp board

use crate::board::lcd;

use super::datalink_base::{StartByte, PacketBaseStructure, ProtoStates, ProtoControl};

struct PacketOutgoingService {
    startByte_: StartByte,
    basePack_: PacketBaseStructure,
    duplicateESC_: bool,
    state_: ProtoStates,
    checkSum_: u8,
}

impl PacketOutgoingService {

    pub fn new(startByte: StartByte, p: PacketBaseStructure) -> Self {
        Self {
            startByte_: StartByte::STX,
            basePack_: p,
            duplicateESC_: false,
            state_: ProtoStates::INITIAL_ESC,
            checkSum_: 0x00,
        }
    }

    pub fn readNextByte(&mut self) -> Option<u8> {
        //if Has Finished return None, otherwise return the next byte.    
        if self.state_ == ProtoStates::SUCESSFUL {
            return None
        };

        if self.duplicateESC_ { 
            self.duplicateESC_ = false; 
            // earlier return on duplicated esc so it is not considered in checksum calculation
            return Some(ProtoControl::ESC as u8) 
        };

        let mut nextState: ProtoStates = ProtoStates::ERROR;
        let mut result: u8 = ProtoControl::ESC as u8;

        match self.state_ {

            ProtoStates::INITIAL_ESC => {
                result = ProtoControl::ESC as u8;
                nextState = ProtoStates::START_BYTE ;
            }

            ProtoStates::START_BYTE => {
                result = self.startByte_ as u8;
                nextState = ProtoStates::DIRECTION_AND_CHANNEL ;
            }

            ProtoStates::DIRECTION_AND_CHANNEL => {
                let direcao = self.basePack_.direcao;
                let canal = self.basePack_.canal;
                let direcao_canal = direcao + canal;
                result = direcao_canal;
                nextState = ProtoStates::COMMAND ;
            }

            ProtoStates::COMMAND => {
                result = self.basePack_.cmd;
                nextState = ProtoStates::DATA_LOW ;
            }

            ProtoStates::DATA_LOW => {
                result = self.basePack_.dadoLow;
                nextState = ProtoStates::DATA_HIGH ;
            }

            ProtoStates::DATA_HIGH => {
                result = self.basePack_.dadoHigh;
                nextState = ProtoStates::FINAL_ESC ;
            }

            ProtoStates::FINAL_ESC => {
                result = ProtoControl::ESC as u8;
                nextState = ProtoStates::ETX_BYTE ;
            }

            ProtoStates::ETX_BYTE => {
                result = ProtoControl::ETX as u8;
                nextState = ProtoStates::CHECKSUM ;
            }

            ProtoStates::CHECKSUM => {
                result = 0x00 - self.checkSum_; //checksum two's-complement 
                nextState = ProtoStates::SUCESSFUL ;
            }
            
            
            _ => {
                #[allow(unreachable_code)]
                !unreachable!()
            }      
        }

        //calculates checksum and esc_dup

        let isNotFinalOrInitialEsc: bool = self.state_ != ProtoStates::INITIAL_ESC && self.state_ != ProtoStates::FINAL_ESC;
        let isCheckSum: bool  = self.state_ != ProtoStates::CHECKSUM;
        let isEsc: bool = result == ProtoControl::ESC as u8;
        if isNotFinalOrInitialEsc {
            // do not compute checksum on control ESC's: initial esc, final esc or duplicated esc
            if isCheckSum  {
                self.checkSum_ = self.checkSum_ + result;
            };
            // duplicate ESC if current byte is ESC but it is not a kind of control ESC byte but a Data ESC
            if isEsc  {
                self.duplicateESC_ = true;
            };
        }

        // update state
        self.state_ = nextState;
        
        Some(result)
    }

}



pub fn development_entry_point() -> ! {

    lcd::lcd_initialize();
    lcd::clear();
    lcd::print("Running  ");

    
    let mut frame = PacketBaseStructure::new();
    frame.canal = 0b11000000;
    frame.cmd = 0x77;
    frame.dadoLow = 0x78;
    frame.dadoHigh = 0x12;

    let mut parser = PacketOutgoingService::new(StartByte::STX, frame);

    while let Some(byte) = parser.readNextByte() {
        lcd::print_u8_in_hex(byte);
    }
    
    
    loop { }

}