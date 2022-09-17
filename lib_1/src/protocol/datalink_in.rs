// low-level driver for receiving data from the cmpp board

use super::datalink_base::{ 
    PacketBaseStructure, 
    ProtoControl,
    ProtoStates,
};


pub struct PacketIncommingService {
    //resultPack_: &mut PacketBaseStructure,
    temp_pack: PacketBaseStructure,
    state_: ProtoStates,
    check_sum: u8,
    duplicate_esc: bool,
} 

impl PacketIncommingService {

    pub fn new() -> Self {
        PacketIncommingService {
            //resultPack_: _p,
            temp_pack: PacketBaseStructure::new(),
            state_: ProtoStates::InitialEsc,
            check_sum: 0x00,
            duplicate_esc: false,
        }
    }

    #[allow(unreachable_code)]
    pub fn parse_next(&mut self, _byte: u8) -> (ProtoStates, Option<PacketBaseStructure>) {

        let is_not_final_or_initial_esc: bool = (self.state_ != ProtoStates::InitialEsc) && (self.state_ != ProtoStates::FinalEsc);
        let is_check_sum: bool = self.state_ != ProtoStates::Checksum;
        let is_esc: bool = _byte == (ProtoControl::ESC as u8);
        let must_duplicate_esc: bool = self.duplicate_esc;

        // //duplicate ESC and do earlier return
        if must_duplicate_esc && is_esc {
            self.duplicate_esc = false;
            return (self.state_, None)
        } 

        if is_not_final_or_initial_esc {
            if is_check_sum  {
                // do not compute checksum on control ESC's: initial esc, final esc or duplicated esc
                self.check_sum = self.check_sum + _byte;
            };
            if is_esc  {
                // duplicate ESC if current byte is ESC (but not a kind of control ESC byte but a Data ESC)
                self.duplicate_esc = true;
            };
        }

        match self.state_ {
        
            ProtoStates::InitialEsc => {
                if _byte == ProtoControl::ESC as u8 { 
                    self.state_ = ProtoStates::StartByte; 
                }   
            }
                  
            ProtoStates::StartByte => {
                // FIX: Consider also stx and nack as start byte
                if _byte == ProtoControl::ACK as u8 { 
                    self.state_ = ProtoStates::DirectionAndChannel; 
                }  
            }        
                
            ProtoStates::DirectionAndChannel => {
                self.temp_pack.direcao = _byte & 0b11000000;
                self.temp_pack.canal = _byte &   0b00111111;
                self.state_ = ProtoStates::Command
            }
             
            ProtoStates::Command => {
                self.temp_pack.cmd = _byte;
                self.state_ = ProtoStates::DataLow
            }
                
            ProtoStates::DataLow => {
                self.temp_pack.dado_low = _byte;
                self.state_ = ProtoStates::DataHigh  
            }
                
            ProtoStates::DataHigh => {
                self.temp_pack.dado_high = _byte;
                self.state_ = ProtoStates::FinalEsc
            }

            ProtoStates::FinalEsc => {
                if _byte == ProtoControl::ESC as u8 { 
                    self.state_ = ProtoStates::EtxByte; 
                } 
            }

            ProtoStates::EtxByte => {
                if _byte == ProtoControl::ETX as u8 { 
                    self.state_ = ProtoStates::Checksum; 
                }
            }

            ProtoStates::Checksum => {
                self.check_sum = !self.check_sum; // note: not sure "!" operator will work here
                //FIX: implement checksum check
                //if (checkSum_ == byte) {
                    //*resultPack_ = tempPack_;
                    self.state_ = ProtoStates::Sucessful;
                //}
            }
             
            ProtoStates::Sucessful => {
                //??
                !unreachable!()
            }
                
            ProtoStates::Error => {
                //FIX: not implemented
                !unimplemented!()
            }

                  
        };

        if self.state_ == ProtoStates::Checksum {
            (self.state_, Some(self.temp_pack))
        } else {
            (self.state_, None)
        }
        
    }

}


#[cfg(test)]
mod tests {
    use super::*;



    #[test]
    fn it_can_accept_a_valid_segment() {
        #[allow(arithmetic_overflow)]
        let direcao = 0xC0;
        let canal = 0x01;
        let cmd = 0x50;
        let dado_high = 0x61;
        let dado_low = 0x03;
        let valid_segment: [u8;9] = [0x1B, 0x06, direcao+canal, cmd, dado_low, dado_high, 0x1B, 0x03, 0x87];
        let mut parser = PacketIncommingService::new();
        let mut success: bool = false;
        for byte in valid_segment {
            let (_state, result) = parser.parse_next(byte);
            if let Some(output) = result {
                success = true;
                assert_eq!(output.canal, canal);
                assert_eq!(output.direcao, direcao);
                assert_eq!(output.cmd, cmd);
                assert_eq!(output.dado_high, dado_high);
                assert_eq!(output.dado_low, dado_low);
            };
        }
        assert_eq!(success, true);
    }
}