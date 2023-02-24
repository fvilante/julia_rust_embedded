use crate::protocol::datalink::datalink::{
    DLError, Datalink, PacodeDeRetornoDeSolicitacao, PacoteDeRetornoComErro,
    PacoteDeRetornoDeEnvio, Status,
};

use self::{
    cmpp_value::{Bit, MechanicalProperties, Word16},
    manipulator::WordSetter,
    memory_map::{BitAddress, WordAddress},
};

mod cmpp_value {
    use super::memory_map;

    #[derive(Clone, Copy)]
    pub struct MechanicalProperties {
        pub pulses_per_motor_revolution: u16,
        pub linear_displacement_per_tooth_belt: u16,
    }

    pub trait IntoCmppValue<T> {
        fn to_cmpp_value(&self, properties: MechanicalProperties) -> T;
    }

    #[derive(Copy, Clone)]
    pub struct Bit(pub bool);

    impl Into<bool> for Bit {
        fn into(self) -> bool {
            self.0
        }
    }

    #[derive(Copy, Clone)]
    pub struct Word16(pub u16);

    impl Into<u16> for Word16 {
        fn into(self) -> u16 {
            self.0
        }
    }

    pub struct Byte(pub u8);
}

mod memory_map {

    #[derive(Copy, Clone)]
    /// Also known as `cmpp command`
    pub struct WordAddress {
        pub word_address: u8,
    }

    impl WordAddress {
        pub fn new(word_address: u8) -> Self {
            Self { word_address }
        }
    }

    impl From<u8> for WordAddress {
        fn from(value: u8) -> Self {
            Self::new(value)
        }
    }

    impl Into<u8> for WordAddress {
        fn into(self) -> u8 {
            self.word_address
        }
    }

    pub enum BytePosition {
        ByteLow,
        ByteHigh,
    }

    pub struct ByteAddress {
        pub word_address: u8,
        pub byte_position: BytePosition,
    }

    #[derive(Copy, Clone)]
    /// TODO: Place it in lib1::Utils (there are other places where this similar
    /// kind of construction is used (search for 'Bit' like in IOExpander::Bit, etc). Put every application of
    /// this concept in a unique place and reuse it)
    pub enum BitPosition {
        D0 = 0,
        D1 = 1,
        D2 = 2,
        D3 = 3,
        D4 = 4,
        D5 = 5,
        D6 = 6,
        D7 = 7,
        D8 = 8,
        D9 = 9,
        D10 = 10,
        D11 = 11,
        D12 = 12,
        D13 = 13,
        D14 = 14,
        D15 = 15,
    }

    #[derive(Copy, Clone)]
    pub struct BitAddress {
        /// TODO: Change to WordAddress type
        pub word_address: u8,
        /// value between 0..16 (inclusive, exclusive)
        pub bit_position: BitPosition,
    }
}

#[derive(Debug)]
pub enum TLError {
    PacoteDeRetornoComErro(PacoteDeRetornoComErro),
    DLError(DLError),
}

pub mod manipulator {
    use crate::protocol::datalink::datalink::Status;

    use super::{
        cmpp_value::{self, Bit, IntoCmppValue, Word16},
        memory_map::{self, BitAddress, WordAddress},
        TLError, TransportLayer,
    };

    pub struct WordSetter<'a> {
        pub transport_layer: &'a TransportLayer,
        pub memory_map: WordAddress,
    }

    impl<'a> WordSetter<'a> {
        pub fn set<T>(&self, value: T) -> Result<Status, TLError>
        where
            T: IntoCmppValue<Word16>,
        {
            let properties = self.transport_layer.get_mechanical_properties();
            let cmpp_value = value.to_cmpp_value(properties);
            let word_address = self.memory_map;
            self.transport_layer
                .safe_datalink()
                .set_word16(cmpp_value, word_address)
        }
    }

    pub struct BitSetter<'a> {
        transport_layer: &'a TransportLayer,
        memory_map: BitAddress,
    }

    impl<'a> BitSetter<'a> {
        pub fn set<T>(&self, value: T) -> Result<Status, TLError>
        where
            T: IntoCmppValue<Bit>,
        {
            let properties = self.transport_layer.get_mechanical_properties();
            let cmpp_value = value.to_cmpp_value(properties);
            let word_address = self.memory_map;
            self.transport_layer
                .safe_datalink()
                .send_bit(cmpp_value, word_address)
        }
    }
}

struct SafeDatalink<'a> {
    datalink: &'a Datalink,
}

impl<'a> SafeDatalink<'a> {
    fn new(datalink: &'a Datalink) -> Self {
        Self { datalink }
    }

    /// Helper function to cast values
    fn cast_map<A, B>(
        response: Result<Result<A, PacoteDeRetornoComErro>, DLError>,
        f: fn(A) -> B,
    ) -> Result<B, TLError> {
        match response {
            Ok(Ok(pacote_de_retorno)) => {
                let value = f(pacote_de_retorno);
                Ok(value)
            }
            Ok(Err(pacote_de_retorno_com_erro)) => {
                Err(TLError::PacoteDeRetornoComErro(pacote_de_retorno_com_erro))
            }
            Err(datalink_error) => Err(TLError::DLError(datalink_error)),
        }
    }

    pub fn send_bit(&self, bit: Bit, map: BitAddress) -> Result<Status, TLError> {
        let bit_mask = 1 << (map.bit_position as u16);
        let response = match bit.into() {
            true => self.datalink.set_bit_mask(map.word_address, bit_mask),
            false => self.datalink.reset_bit_mask(map.word_address, bit_mask),
        };
        Self::cast_map(response, |pacote_de_retorno| pacote_de_retorno.status)
    }

    pub fn set_word16(
        &self,
        word_value: Word16,
        word_address: WordAddress,
    ) -> Result<Status, TLError> {
        let response = self
            .datalink
            .set_word16(word_address.into(), word_value.into());
        Self::cast_map(response, |pacote_de_retorno| pacote_de_retorno.status)
    }

    pub fn get_word16(&self, word_address: WordAddress) -> Result<Word16, TLError> {
        let response = self.datalink.get_word16(word_address.into());
        Self::cast_map(response, |pacote_de_retorno| {
            cmpp_value::Word16(pacote_de_retorno.data.to_u16())
        })
    }
}

pub struct TransportLayer {
    datalink: Datalink,
    mechanical_properties: MechanicalProperties,
}

impl TransportLayer {
    fn get_mechanical_properties(&self) -> MechanicalProperties {
        self.mechanical_properties
    }

    // Primitives in relation to datalink

    fn safe_datalink<'a>(&'a self) -> SafeDatalink<'a> {
        SafeDatalink::new(&self.datalink)
    }

    fn velocidade_de_avanco<'a>(&'a self) -> WordSetter<'a> {
        WordSetter {
            transport_layer: &self,
            memory_map: WordAddress { word_address: 0x50 },
        }
    }
}

///////////////////////////////////

pub mod investigation_of_pattern {

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
        assert_eq!(u.do_some_work_and_return_number(), 100);
    }
}

//////////////////////////////////////////////////////
// TESTS
/////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    use crate::protocol::{
        datalink::datalink::{
            emulated::{lazy_now, loopback_try_rx, smart_try_tx},
            Channel,
        },
        transport::transport_layer::cmpp_value::{IntoCmppValue, MechanicalProperties},
    };

    use super::{memory_map::WordAddress, *};

    const MECHANICAL_PROPERTIES: MechanicalProperties = MechanicalProperties {
        linear_displacement_per_tooth_belt: 1234,
        pulses_per_motor_revolution: 2345,
    };

    #[test]
    fn it_can_transact_something() {
        //temp
        use super::investigation_of_pattern::main;
        main();

        // setup
        let datalink = Datalink {
            channel: Channel::from_u8(1).unwrap(),
            timeout_ms: 1000,
            try_tx: smart_try_tx,
            try_rx: loopback_try_rx,
            now: lazy_now,
        };

        let transport = TransportLayer {
            datalink,
            mechanical_properties: MECHANICAL_PROPERTIES,
        };

        //send

        let response = transport.safe_datalink().get_word16(0x00.into());

        let data = response.unwrap();

        let value = data.0;

        assert_eq!(value, 0)

        //receive
    }

    #[test]
    fn it_can_transact_something_using_manipulator() {
        // setup
        let datalink = Datalink {
            channel: Channel::from_u8(1).unwrap(),
            timeout_ms: 1000,
            try_tx: smart_try_tx,
            try_rx: loopback_try_rx,
            now: lazy_now,
        };

        let transport = TransportLayer {
            datalink,
            mechanical_properties: MECHANICAL_PROPERTIES,
        };

        struct Milimeter(pub u16);

        impl IntoCmppValue<Word16> for Milimeter {
            fn to_cmpp_value(&self, mechanical_properties: MechanicalProperties) -> Word16 {
                let some_factor = mechanical_properties.pulses_per_motor_revolution;
                Word16(self.0 * some_factor)
            }
        }

        //send

        let response = transport.velocidade_de_avanco().set(Milimeter(10));

        let status = response.unwrap();

        //receive
    }
}
