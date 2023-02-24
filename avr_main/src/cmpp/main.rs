use heapless::Deque;
use lib_1::protocol::datalink::{
    decoder::{Decoder, DecodingError},
    frame::Frame,
    prelude::StartByte,
    transact::{transact, DatalinkError},
};

use crate::{
    board::lcd,
    microcontroler::{delay::delay_us, serial},
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

fn teste_dequeue() {
    let mut queue: Deque<u8, 3> = Deque::new();

    //queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);
    lcd::print_u8_in_hex(queue.pop_front().unwrap());
    match queue.push_back(4) {
        Ok(_) => lcd::print("4o elemento OK"),
        Err(_) => lcd::print("4o elemento PROBLEMA"),
    }
    // queue.push_front(20);
    lcd::print_u8_in_hex(queue.pop_front().unwrap());
    lcd::print_u8_in_hex(queue.pop_front().unwrap());
    lcd::print_u8_in_hex(queue.pop_front().unwrap());
}

pub fn development_entry_point() -> ! {
    /// Here is how to use enum_dispatch
    ///
    /// reference: https://crates.io/crates/enum_dispatch
    use enum_dispatch::enum_dispatch;

    #[enum_dispatch]
    trait MyBehaviour {
        fn get_age(&self) -> u8;
    }

    #[enum_dispatch]
    trait Widget {
        fn get_widget(&self) -> u8;
    }

    #[derive(Clone, Copy)]
    struct Foo {}

    impl Foo {
        fn new() -> Self {
            Self {}
        }

        fn non_trait_method(&self) -> u8 {
            222
        }
    }

    impl MyBehaviour for Foo {
        fn get_age(&self) -> u8 {
            8
        }
    }

    impl Widget for Foo {
        fn get_widget(&self) -> u8 {
            0
        }
    }

    #[derive(Clone, Copy)]
    struct Bar;

    impl MyBehaviour for Bar {
        fn get_age(&self) -> u8 {
            77
        }
    }

    impl Widget for Bar {
        fn get_widget(&self) -> u8 {
            1
        }
    }

    #[derive(Clone, Copy)]
    struct Juca;

    impl Juca {
        fn non_trait_method(&self) -> u8 {
            222
        }
    }

    impl MyBehaviour for Juca {
        fn get_age(&self) -> u8 {
            166
        }
    }

    impl Widget for Juca {
        fn get_widget(&self) -> u8 {
            2
        }
    }

    #[enum_dispatch(OthersEnum, MyBehaviourEnum)]
    trait Common {
        fn do_some_work_and_return_number(&self) -> u16;
    }

    struct Nego;

    impl Nego {
        fn get_behaviour(&self) -> MyBehaviourEnum {
            Juca.into()
        }
    }

    struct Kiss;

    impl Common for Foo {
        fn do_some_work_and_return_number(&self) -> u16 {
            100
        }
    }

    impl Common for Bar {
        fn do_some_work_and_return_number(&self) -> u16 {
            101
        }
    }

    impl Common for Juca {
        fn do_some_work_and_return_number(&self) -> u16 {
            102
        }
    }

    impl Common for Kiss {
        fn do_some_work_and_return_number(&self) -> u16 {
            103
        }
    }

    impl Common for Nego {
        fn do_some_work_and_return_number(&self) -> u16 {
            104
        }
    }

    #[derive(Clone, Copy)]
    #[enum_dispatch(MyBehaviour, Widget)]
    enum MyBehaviourEnum {
        Foo,  //8
        Bar,  //77
        Juca, //166
    }

    enum OthersEnum {
        Kiss,
        Nego,
    }

    pub fn main() {
        lcd::lcd_initialize();
        lcd::print("Pensando o que?");

        let foo = Foo {};
        let bar = Bar;
        let juca = Juca;
        let array: [(MyBehaviourEnum, u8, u8); 10] = [
            (foo.into(), 8, 0),
            (bar.into(), 77, 1),
            (foo.into(), 8, 0),
            (bar.into(), 77, 1),
            (juca.into(), 166, 2),
            (bar.into(), 77, 1),
            (foo.into(), 8, 0),
            (juca.into(), 166, 2),
            (bar.into(), 77, 1),
            (bar.into(), 77, 1),
        ];
        for (each, expected_age, expected_widget) in array {
            assert_eq!(each.get_age(), expected_age);
            assert_eq!(each.get_widget(), expected_widget);
        }

        // explicit cast necessary to get the enum
        let p: MyBehaviourEnum = juca.into();
        let a = p.get_age();
        assert_eq!(a, 166);

        // uncast (use try_into)
        let u: Result<Juca, _> = p.try_into();
        assert_eq!(u.unwrap().non_trait_method(), 222);

        // common trait

        let u: MyBehaviourEnum = foo.into();
        lcd::print("-------------------------->");
        lcd::print_u16_in_hex(u.do_some_work_and_return_number());

        assert_eq!(u.do_some_work_and_return_number(), 100);
    }

    lcd::lcd_initialize();
    lcd::print("Juca kifuri");

    main();

    let frame = Frame::new(StartByte::STX, [0, 0x50, 0, 0].into());

    serial::init(9600);

    lcd::print("(A);");
    for byte in frame.encode() {
        loop {
            if let Ok(_) = serial::try_transmit(byte) {
                break;
            }
        }
    }
    lcd::print("(B);");

    let _count: u16 = 0;

    let mut decoder = Decoder::new();

    loop {
        if let Some(byte) = serial::try_receive() {
            match decoder.parse_next(byte) {
                Ok(res) => {
                    match res {
                        Some(_frame) => {
                            lcd::print("Success");
                        }
                        None => {
                            // processing input
                        }
                    }
                }
                Err(_error) => {
                    lcd::print("Error");
                }
            }
        }
    }

    //crate::microcontroler::serial::development_entry_point();

    //test_cmpp();

    loop {}
}
