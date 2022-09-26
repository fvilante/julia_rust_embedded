use crate::microcontroler::serial;
use crate::lib_1::types::serial_connection::SerialConnection;


pub struct ConcreteSerialPort {
    baud_rate: u32,
}

impl SerialConnection for ConcreteSerialPort {

    fn new(baud_rate: u32) -> Self {
        serial::init(baud_rate);
        Self { 
            baud_rate,
        }
    }

    fn transmit(&self, byte: u8) {
        serial::transmit(byte)
    }
    fn ready_to_receive(&self) -> bool {
        serial::ready_to_receive()
    }
    fn receive(&self) -> u8 {
        serial::receive()
    }

}
