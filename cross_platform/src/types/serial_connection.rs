pub trait SerialConnection {
    fn new(baud_rate: u32) -> Self;
    fn transmit(&self, byte: u8);
    fn ready_to_receive(&self) -> bool;
    fn receive(&self) -> u8;
}
