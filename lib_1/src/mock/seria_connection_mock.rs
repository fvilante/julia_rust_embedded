use crate::{types::serial_connection::SerialConnection};
use super::ring_buffer::{RingBuffer};

const BUFFER_SIZE: usize = 128 ;

static mut BUFFER: RingBuffer<u8, BUFFER_SIZE> = RingBuffer::new([0x00 as u8; BUFFER_SIZE]);

pub struct MockedSerialConnection {
    baud_rate: u32,
}

impl SerialConnection for MockedSerialConnection {
    fn new(baud_rate: u32) -> Self {
        Self {
            baud_rate,
        }
    }

    fn transmit(&self, byte: u8) {
        unsafe { BUFFER.write(byte).unwrap(); };
    }

    fn ready_to_receive(&self ) -> bool {
        unsafe { BUFFER.is_empty() == false }
    }

    fn receive(&self ) -> u8 {
        unsafe { BUFFER.read().unwrap() }
    }

}

pub fn add(left: u8, right: u8) -> u8 {
    left + right + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_writes_and_reads_one_byte() {
        let serial = MockedSerialConnection::new(9600);
        let probe = 0x77;
        serial.transmit(probe);
        let read = serial.receive();
        assert_eq!(read, probe);
    }

    fn it_writes_and_reads_a_bunch_of_bytes() {
        let serial = MockedSerialConnection::new(9600);
        for i in 0..BUFFER_SIZE*2 {
            let probe: u8 = i.try_into().unwrap();
            serial.transmit(probe);
            let read = serial.receive();
            assert_eq!(read, probe);
        };
    }
}
