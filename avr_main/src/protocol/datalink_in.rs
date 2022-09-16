// low-level driver for receiving data from the cmpp board
use crate::{board::lcd, microcontroler::delay::delay_ms};

use super::datalink_base::{ 
    PacketBaseStructure, 
    ProtoControl,
    ProtoStates,
};


pub struct PacketIncommingService {
    //resultPack_: &mut PacketBaseStructure,
    tempPack_: PacketBaseStructure,
    state_: ProtoStates,
    checkSum_: u8,
    duplicateEsc_: bool,
} 

impl PacketIncommingService {

    pub fn new() -> Self {
        PacketIncommingService {
            //resultPack_: _p,
            tempPack_: PacketBaseStructure::new(),
            state_: ProtoStates::INITIAL_ESC,
            checkSum_: 0x00,
            duplicateEsc_: false,
        }
    }

    #[allow(unreachable_code)]
    pub fn processNextByte(&mut self, _byte: u8) -> (ProtoStates, Option<PacketBaseStructure>) {

        let isNotFinalOrInitialEsc: bool = (self.state_ != ProtoStates::INITIAL_ESC) && (self.state_ != ProtoStates::FINAL_ESC);
        let isCheckSum: bool = self.state_ != ProtoStates::CHECKSUM;
        let isEsc: bool = _byte == (ProtoControl::ESC as u8);
        let mustDuplicateEsc: bool = self.duplicateEsc_;

        // //duplicate ESC and do earlier return
        if mustDuplicateEsc && isEsc {
            self.duplicateEsc_ = false;
            return (self.state_, None)
        } 

        if isNotFinalOrInitialEsc {
            if isCheckSum  {
                // do not compute checksum on control ESC's: initial esc, final esc or duplicated esc
                self.checkSum_ = self.checkSum_ + _byte;
            };
            if isEsc  {
                // duplicate ESC if current byte is ESC (but not a kind of control ESC byte but a Data ESC)
                self.duplicateEsc_ = true;
            };
        }

        match self.state_ {
        
            ProtoStates::INITIAL_ESC => {
                if _byte == ProtoControl::ESC as u8 { 
                    self.state_ = ProtoStates::START_BYTE; 
                }   
            }
                  
            ProtoStates::START_BYTE => {
                // FIX: Consider also stx and nack as start byte
                if _byte == ProtoControl::ACK as u8 { 
                    self.state_ = ProtoStates::DIRECTION_AND_CHANNEL; 
                }  
            }        
                
            ProtoStates::DIRECTION_AND_CHANNEL => {
                self.tempPack_.direcao = _byte & 0b11000000;
                self.tempPack_.canal = _byte &   0b00111111;
                self.state_ = ProtoStates::COMMAND
            }
             
            ProtoStates::COMMAND => {
                self.tempPack_.cmd = _byte;
                self.state_ = ProtoStates::DATA_LOW
            }
                
            ProtoStates::DATA_LOW => {
                self.tempPack_.dadoLow = _byte;
                self.state_ = ProtoStates::DATA_HIGH  
            }
                
            ProtoStates::DATA_HIGH => {
                self.tempPack_.dadoHigh = _byte;
                self.state_ = ProtoStates::FINAL_ESC
            }

            ProtoStates::FINAL_ESC => {
                if _byte == ProtoControl::ESC as u8 { 
                    self.state_ = ProtoStates::ETX_BYTE; 
                } 
            }

            ProtoStates::ETX_BYTE => {
                if _byte == ProtoControl::ETX as u8 { 
                    self.state_ = ProtoStates::CHECKSUM; 
                }
            }

            ProtoStates::CHECKSUM => {
                self.checkSum_ = !self.checkSum_; // note: not sure "!" operator will work here
                //FIX: implement checksum check
                //if (checkSum_ == byte) {
                    //*resultPack_ = tempPack_;
                    self.state_ = ProtoStates::SUCESSFUL;
                //}
            }
             
            ProtoStates::SUCESSFUL => {
                //??
                !unreachable!()
            }
                
            ProtoStates::ERROR => {
                //FIX: not implemented
                !unimplemented!()
            }
                
            _ => { 
                // should never happen
                !unreachable!()
            }
                  
        };

        if self.state_ == ProtoStates::CHECKSUM {
            (self.state_, Some(self.tempPack_))
        } else {
            (self.state_, None)
        }
        
    }

}





pub fn development_entry_point() -> ! {

    lcd::lcd_initialize();
    lcd::clear();
    lcd::print("Running  ");

    let mut parser = PacketIncommingService::new();
    
    #[allow(arithmetic_overflow)]
    let frame: [u8;12] = [0x1B, 0x06, 0xC1, 0x50, 0x61, 0x02, 0x1B, 0x03, 0x87,0x00,0x00,0x00];

   
    for byte in frame {
        let (state, output) = parser.processNextByte(byte);
        //lcd::print(protoStates_toString(state));
        //lcd::print("; ");
        if let Some(res) = output {
            lcd::clear();
            lcd::print("Juca");
            lcd::print_u8_in_hex(res.direcao);
            lcd::print_u8_in_hex(res.canal);
            lcd::print_u8_in_hex(res.cmd);
            lcd::print_u8_in_hex(res.dadoHigh);
            lcd::print_u8_in_hex(res.dadoLow);
        }
        //delay_ms(2000);
    }
    loop { }
}