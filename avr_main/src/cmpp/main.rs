use lib_1::protocol::{
    datalink::datalink::Datalink,
    transport::{
        channel::Channel,
        transport_layer::{
            cmpp_value::MechanicalProperties, new_proposal::Displacement, TLError, TransportLayer,
        },
    },
};

use crate::{
    board::lcd,
    microcontroler::{delay::delay_ms, serial, timer},
};

fn panic_error_message(_error: TLError) -> () {
    panic!("TLErr");
    //match error {
    //    TLError::PacoteDeRetornoComErro(_) => {
    //        panic!("Pacote recebido com NACK")
    //    }
    //    TLError::DLError(datalink_error) => match datalink_error {
    //        DLError::InvalidChannel(_) => panic!("InvalidChannel"),
    //        DLError::SerialTransmissionTimeedOut(_) => {
    //            panic!("SerialTransmissionError")
    //        }
    //        DLError::DecodingError(_) => panic!("DecodingError"),
    //        DLError::Timeout(_) => panic!("Timeout"),
    //        DLError::SerialReceptionError => panic!("SerialReceptionError"),
    //        DLError::SlaveHasReturnedStartByteAsNeitherAckNorNack => {
    //            panic!("SlaveHasReturnedStartByteAsNeitherAckNorNack")
    //        }
    //        DLError::SlaveHasReturnedNack(_) => panic!("SlaveHasReturnedNack"),
    //    },
    //}
}

pub fn development_entry_point() {
    lcd::lcd_initialize();

    lcd::clear();
    lcd::print("Ini");

    let channel = Channel::from_u8(0).unwrap();
    fn now() -> u16 {
        timer::now() as u16
    }
    const TIMEOUT_MS: u16 = 1000; // TODO: Maybe in future be calculated as a function of the connection baud rate

    const BAUD_RATE: u32 = 9600; // FIX: 2400 is not working, the problem seems to be in the register's port setup configuration
    let _serial = serial::init(BAUD_RATE);

    fn try_rx() -> Result<Option<u8>, ()> {
        Ok(serial::try_receive())
    }

    fn try_tx(byte: u8) -> Option<()> {
        serial::try_transmit(byte).ok()
    }

    let datalink = &Datalink {
        channel,
        now,
        timeout_ms: TIMEOUT_MS,
        try_rx,
        try_tx,
        debug_reception: None,
    };

    let mechanical_properties = MechanicalProperties {
        pulses_per_motor_revolution: 400,
        linear_displacement_per_tooth_belt_mult_by_100: 508,
        number_of_tooths_of_motor_pulley: 16,
    };

    let transport = TransportLayer::new(datalink, mechanical_properties);

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
    lcd::print("Fim");

    delay_ms(4000);
}
