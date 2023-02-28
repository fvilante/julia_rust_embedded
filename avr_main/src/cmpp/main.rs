use heapless::Deque;
use lib_1::protocol::{
    datalink::{
        datalink::{word16::Word16, DLError, Datalink},
        decoder::{Decoder, DecodingError},
        frame::Frame,
        prelude::StartByte,
        transact::{transact, DatalinkError},
    },
    transport::{
        channel::Channel,
        transport_layer::{
            self,
            cmpp_value::{self, MechanicalProperties},
            new_proposal::Displacement,
            TLError, TransportLayer,
        },
    },
};

use crate::{
    board::lcd,
    microcontroler::{
        delay::{delay_ms, delay_us},
        serial,
        timer::now,
    },
};

use super::datalink::concrete_serial::ConcreteSerialPort;
use lib_1::types::serial_connection::SerialConnection;

/// Helper to just clamp values
fn delay_us_helper(time_us: u64) {
    let time_us_clamped = time_us.clamp(u32::MIN as u64, u32::MAX as u64);
    delay_us(time_us_clamped)
}

fn test_cmpp() {
    let frame = Frame::make_master_block([0, 0x50, 0, 0].into());
    let connection = ConcreteSerialPort::new(2400);
    const timeout_us: u64 = 200 * 1000;
    let response = transact(frame, connection, timeout_us, delay_us_helper);
    match response {
        Ok(_response) => lcd::print("Response Ok"),
        Err(error) => {
            lcd::print("Response Err");
            match error {
                DatalinkError::DecodingError(error) => {
                    lcd::print("DecodingError");

                    match error {
                        DecodingError::InvalidStartByte(start_byte) => {
                            lcd::print("InvalidStartByte=");
                            lcd::print_u8_in_hex(start_byte)
                        }
                        DecodingError::BufferOverFlow => {
                            lcd::print("BufferOverFlow");
                        }
                        DecodingError::ExpectedEtxOrEscDupButFoundOtherThing(_) => {
                            lcd::print("ExpectedEtxOrEscDupButFoundOtherThing");
                        }
                        DecodingError::ChecksumIsEscButNotDuplicated(_) => {
                            lcd::print("ChecksumIsEscButNotDuplicated");
                        }
                        DecodingError::InvalidChecksum {
                            expected: _,
                            received: _,
                        } => {
                            lcd::print("InvalidChecksum");
                        }
                    }
                }
                DatalinkError::ReceptionTimeout { elapsed_time } => {
                    lcd::print("Timeout=");
                    let _e = elapsed_time / 1000; //to milisec
                }
            }
        }
    }
}

fn panic_error_message(error: TLError) -> () {
    match error {
        TLError::PacoteDeRetornoComErro(_) => {
            panic!("Pacote recebido com NACK")
        }
        TLError::DLError(datalink_error) => match datalink_error {
            DLError::InvalidChannel(_) => panic!("InvalidChannel"),
            DLError::SerialTransmissionTimeedOut(_) => {
                panic!("SerialTransmissionError")
            }
            DLError::DecodingError(_) => panic!("DecodingError"),
            DLError::Timeout(_) => panic!("Timeout"),
            DLError::SerialReceptionError => panic!("SerialReceptionError"),
            DLError::SlaveHasReturnedStartByteAsNeitherAckNorNack => {
                panic!("SlaveHasReturnedStartByteAsNeitherAckNorNack")
            }
            DLError::SlaveHasReturnedNack(_) => panic!("SlaveHasReturnedNack"),
        },
    }
}

pub fn development_entry_point() -> ! {
    lcd::lcd_initialize();
    lcd::print("Juca kifuri");

    let channel = Channel::from_u8(0).unwrap();
    let now = now;
    let timeout_ms = 1000; // TODO: Maybe in future be calculated as a function of the connection baud rate

    let baud_rate = 9600; // FIX: 2400 is not working, the problem seems to be in the register's port setup configuration
    let serial = serial::init(baud_rate);

    fn try_rx() -> Result<Option<u8>, ()> {
        Ok(serial::try_receive())
    }

    fn try_tx(byte: u8) -> Option<()> {
        serial::try_transmit(byte).ok()
    }

    let datalink = Datalink {
        channel,
        now,
        timeout_ms,
        try_rx,
        try_tx,
    };

    let mechanical_properties = MechanicalProperties {
        pulses_per_motor_revolution: 400,
        linear_displacement_per_tooth_belt: 828,
    };

    let transport = TransportLayer::new(datalink, mechanical_properties);

    lcd::clear();
    lcd::print("Lendo posicao inicial. ");

    let _status = transport
        .posicao_inicial()
        .set(Displacement(0x200))
        .map_err(panic_error_message)
        .unwrap();

    let value = transport
        .posicao_inicial()
        .get()
        .map_err(panic_error_message)
        .unwrap();

    lcd::print_u16_in_hex(value.0);

    //

    let _status = transport
        .posicao_inicial()
        .set(Displacement(0x100))
        .map_err(panic_error_message)
        .unwrap();

    let value = transport
        .posicao_inicial()
        .get()
        .map_err(panic_error_message)
        .unwrap();

    lcd::print_u16_in_hex(value.0);

    lcd::print("Feito.");
    loop {}
}
