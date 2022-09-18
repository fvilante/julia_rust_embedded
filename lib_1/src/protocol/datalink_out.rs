// low-level driver for transmiting data from the cmpp board

use super::datalink_base::{StartByte, PacketBaseStructure, ProtoStates, ProtoControl};

pub struct PacketOutgoingService {
    start_byte: StartByte,
    base_pack: PacketBaseStructure,
    duplicate_esc: bool,
    state: ProtoStates,
    check_sum: u8,
}

impl PacketOutgoingService {

    pub fn new(start_byte: StartByte, base_pack: PacketBaseStructure) -> Self {
        Self {
            start_byte,
            base_pack,
            duplicate_esc: false,
            state: ProtoStates::InitialEsc,
            check_sum: 0x00,
        }
    }

    pub fn read_next(&mut self) -> Option<u8> {
        //if Has Finished return None, otherwise return the next byte.    
        if self.state == ProtoStates::Sucessful {
            return None
        };

        if self.duplicate_esc { 
            self.duplicate_esc = false; 
            // earlier return on duplicated esc so it is not considered in checksum calculation
            return Some(ProtoControl::ESC as u8) 
        };

        let next_state: ProtoStates;
        let  result: u8;

        match self.state {

            ProtoStates::InitialEsc => {
                result = ProtoControl::ESC as u8;
                next_state = ProtoStates::StartByte ;
            }

            ProtoStates::StartByte => {
                result = self.start_byte as u8;
                next_state = ProtoStates::DirectionAndChannel ;
            }

            ProtoStates::DirectionAndChannel => {
                let direcao = self.base_pack.direcao;
                let canal = self.base_pack.canal;
                let direcao_canal = direcao + canal;
                result = direcao_canal;
                next_state = ProtoStates::Command ;
            }

            ProtoStates::Command => {
                result = self.base_pack.cmd;
                next_state = ProtoStates::DataLow ;
            }

            ProtoStates::DataLow => {
                result = self.base_pack.dado_low;
                next_state = ProtoStates::DataHigh ;
            }

            ProtoStates::DataHigh => {
                result = self.base_pack.dado_high;
                next_state = ProtoStates::FinalEsc ;
            }

            ProtoStates::FinalEsc => {
                result = ProtoControl::ESC as u8;
                next_state = ProtoStates::EtxByte ;
            }

            ProtoStates::EtxByte => {
                result = ProtoControl::ETX as u8;
                next_state = ProtoStates::Checksum ;
            }

            ProtoStates::Checksum => {
                result = 0x00 - self.check_sum; //checksum two's-complement 
                next_state = ProtoStates::Sucessful ;
            }
            
            
            _ => {
                #[allow(unreachable_code)]
                !unreachable!()
            }      
        }

        //calculates checksum and esc_dup

        let is_not_final_or_initial_esc: bool = self.state != ProtoStates::InitialEsc && self.state != ProtoStates::FinalEsc;
        let is_check_sum: bool  = self.state != ProtoStates::Checksum;
        let is_esc: bool = result == ProtoControl::ESC as u8;
        if is_not_final_or_initial_esc {
            // do not compute checksum on control ESC's: initial esc, final esc or duplicated esc
            if is_check_sum  {
                self.check_sum = self.check_sum + result;
            };
            // duplicate ESC if current byte is ESC but it is not a kind of control ESC byte but a Data ESC
            if is_esc  {
                self.duplicate_esc = true;
            };
        }

        // update state
        self.state = next_state;
        
        Some(result)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_can_create_a_valid_segment() {
        let direcao = 0xC0;
        let canal = 0x01;
        let cmd = 0x50;
        let dado_high = 0x61;
        let dado_low = 0x03;
        let valid_segment: [u8;9] = [0x1B, 0x06, direcao+canal, cmd, dado_low, dado_high, 0x1B, 0x03, 0x87];
        let valid_frame = PacketBaseStructure{
            direcao,
            canal,
            cmd,
            dado_high,
            dado_low,
        };
        let mut parser = PacketOutgoingService::new(StartByte::ACK,valid_frame);
        for byte_expected in valid_segment {
            let byte_emited = parser.read_next();
            assert_eq!(byte_expected, byte_emited.unwrap());
        }
    }

    /* 
    #[test]
    fn it_can_create_a_valid_segment() {
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

    */
}