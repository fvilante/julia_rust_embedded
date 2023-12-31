use core::marker::PhantomData;

use crate::protocol::datalink::datalink::{
    word16::Word16, DLError, Datalink, PacodeDeRetornoDeSolicitacao, PacoteDeRetornoComErro,
    PacoteDeRetornoDeEnvio, Status,
};

use self::{
    cmpp_value::{Bit, MechanicalProperties},
    memory_map::{BitAddress, BitPosition, BytePosition, WordAddress},
    new_proposal::{
        Acceleration, ActivationState, Adimensional, AxisMode, BinaryManipulator, ByteManipulator,
        Displacement, FromCmpp, SignalLogic, Time, Velocity, WordManipulator,
    },
};

pub mod cmpp_value {
    use super::memory_map;

    #[derive(Clone, Copy)]
    pub struct MechanicalProperties {
        pub pulses_per_motor_revolution: u16, // NPM = Numero de pulsos por volta do motor
        // TODO: Improve doc for this "fixed point in disguise" calculation
        pub linear_displacement_per_tooth_belt_mult_by_100: u16, // unit: milimeters multiplied by 1000 (normally=508 centh of milimeters); PC=Passo da correia dentada
        pub number_of_tooths_of_motor_pulley: u8, // NDP = numero de dentes da polia motora
    }

    impl MechanicalProperties {
        /// TODO: Improve doc for this "fixed point in disguise" calculation
        pub fn get_pulses_per_milimeters_mult_by_100(&self) -> u16 {
            let npm = self.pulses_per_motor_revolution;
            let ndp = self.number_of_tooths_of_motor_pulley as u16;
            let pc = self.linear_displacement_per_tooth_belt_mult_by_100;
            let value: u32 = (npm as u32 / ndp as u32) * 100 * 100 / pc as u32;
            value as u16
        }
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

    impl From<bool> for Bit {
        fn from(value: bool) -> Self {
            if value {
                Bit(true)
            } else {
                Bit(false)
            }
        }
    }
}

pub mod memory_map {

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

    /// TODO: Probably this type should be moved to another place. But I'm not sure right now where.
    #[derive(Clone, Copy)]
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

/// Transport Layer Error
#[derive(Debug)]
pub enum TLError {
    PacoteDeRetornoComErro(PacoteDeRetornoComErro),
    DLError(DLError),
}

/// TODO: When possible refactor to abstract and generalize Manipulators. You shoul also
/// refactor the concept of bitwise manipulation used inside Word16 as a consequence. Se also
/// BitPosition type `todo` notes.
pub mod new_proposal {
    use core::marker::PhantomData;

    use crate::{
        protocol::datalink::datalink::{word16::Word16, Status},
        utils::cursor::Cursor,
    };

    use super::{
        cmpp_value::{Bit, MechanicalProperties},
        memory_map::{BitAddress, BitPosition, BytePosition, WordAddress},
        TLError, TransportLayer,
    };

    //  ///////////////////////////////////////////////////////////////////////////////////
    //
    //      DISPLACEMENT
    //
    //  ///////////////////////////////////////////////////////////////////////////////////

    pub struct Displacement(pub u16);

    impl From<u16> for Displacement {
        fn from(value: u16) -> Self {
            Displacement(value)
        }
    }

    impl FromCmpp<u16> for Displacement {
        fn from_cmpp(cmpp_value: u16, context: MechanicalProperties) -> Self {
            //TODO: Check if the division is truncating de fractional part instead of rounding it.
            const const_value: u32 = 512;
            let factor: u32 = context.get_pulses_per_milimeters_mult_by_100().into();
            let user_value = (((cmpp_value as u32 - const_value) * 100) / factor);
            Self(user_value as u16)
        }
    }

    impl ToCmpp<u16> for Displacement {
        fn to_cmpp(&self, context: MechanicalProperties) -> u16 {
            //TODO: Check if the division is truncating de fractional part instead of rounding it.
            const const_value: u32 = 512;
            let factor: u32 = context.get_pulses_per_milimeters_mult_by_100().into();
            let user_value = self.0 as u32;
            let cmpp_value = ((user_value * factor) / 100) + const_value;
            cmpp_value as u16
        }
    }

    //  ///////////////////////////////////////////////////////////////////////////////////
    //
    //      VELOCITY
    //
    //  ///////////////////////////////////////////////////////////////////////////////////

    pub struct Velocity(pub u16);

    impl From<u16> for Velocity {
        fn from(value: u16) -> Self {
            Velocity(value)
        }
    }

    impl FromCmpp<u16> for Velocity {
        fn from_cmpp(value: u16, context: MechanicalProperties) -> Self {
            const T: u32 = 1024; // See: CMPP's protocol documentation
                                 // reuse displacement's convertion algorithm
                                 // TODO: make this algorithm clearer
            let temp = Displacement::from_cmpp(value, context).0;
            let user_value: u32 = (temp as u32 * T) / 1000;
            Self(user_value as u16)
        }
    }

    impl ToCmpp<u16> for Velocity {
        fn to_cmpp(&self, context: MechanicalProperties) -> u16 {
            const T: u32 = 1024; // See: CMPP's protocol documentation
                                 // reuse displacement's convertion algorithm
                                 // TODO: make this algorithm clearer
            let temp = Displacement(self.0).to_cmpp(context);
            let cmpp_value: u32 = ((temp as u32 * 1000) / T);
            cmpp_value as u16
        }
    }

    //  ///////////////////////////////////////////////////////////////////////////////////
    //
    //      ACCELERATION
    //
    //  ///////////////////////////////////////////////////////////////////////////////////

    pub struct Acceleration(pub u16);

    impl From<u16> for Acceleration {
        fn from(value: u16) -> Self {
            Acceleration(value)
        }
    }

    impl FromCmpp<u16> for Acceleration {
        fn from_cmpp(value: u16, context: MechanicalProperties) -> Self {
            const T: u32 = 1024; // See: CMPP's protocol documentation
                                 // reuse displacement's convertion algorithm
                                 // TODO: make this algorithm clearer
            let temp = Displacement::from_cmpp(value, context).0;
            let user_value: u32 = (((temp as u32 * T) / 1000) * T) / 1000;
            Self(user_value as u16)
        }
    }

    impl ToCmpp<u16> for Acceleration {
        fn to_cmpp(&self, context: MechanicalProperties) -> u16 {
            const T: u32 = 1024; // See: CMPP's protocol documentation
                                 // reuse displacement's convertion algorithm
                                 // TODO: make this algorithm clearer
            let temp = Displacement(self.0).to_cmpp(context);
            let cmpp_value: u32 = (((temp as u32 * 1000) / T) * 1000) / T;
            cmpp_value as u16
        }
    }

    //  ///////////////////////////////////////////////////////////////////////////////////
    //
    //      TIME
    //
    //  ///////////////////////////////////////////////////////////////////////////////////

    pub struct Time(pub u16);

    impl From<u16> for Time {
        fn from(value: u16) -> Self {
            Time(value)
        }
    }

    impl FromCmpp<u16> for Time {
        fn from_cmpp(cmpp_value: u16, context: MechanicalProperties) -> Self {
            const T: u32 = 1024; // See: CMPP's protocol documentation
            let user_value: u32 = (cmpp_value as u32 * T) / 1000;
            Self(user_value as u16)
        }
    }

    impl ToCmpp<u16> for Time {
        fn to_cmpp(&self, context: MechanicalProperties) -> u16 {
            const T: u32 = 1024; // See: CMPP's protocol documentation
            let user_value = self.0;
            let cmpp_value: u32 = (user_value as u32 * 1000) / T;
            self.0
        }
    }

    //  ///////////////////////////////////////////////////////////////////////////////////
    //
    //      Adimensional16
    //
    //  ///////////////////////////////////////////////////////////////////////////////////

    pub struct Adimensional(pub u16);

    impl From<u16> for Adimensional {
        fn from(value: u16) -> Self {
            Adimensional(value)
        }
    }

    impl FromCmpp<u16> for Adimensional {
        ///TODO: Fake implementation, not performing physical convertion
        fn from_cmpp(value: u16, context: MechanicalProperties) -> Self {
            Self(value)
        }
    }

    impl ToCmpp<u16> for Adimensional {
        ///TODO: Fake implementation, not performing physical convertion
        fn to_cmpp(&self, context: MechanicalProperties) -> u16 {
            self.0
        }
    }

    impl FromCmpp<u8> for Adimensional {
        ///TODO: Fake implementation, not performing physical convertion
        fn from_cmpp(value: u8, context: MechanicalProperties) -> Self {
            Self(value as u16)
        }
    }

    impl ToCmpp<u8> for Adimensional {
        ///TODO: Fake implementation, not performing physical convertion
        fn to_cmpp(&self, context: MechanicalProperties) -> u8 {
            self.0 as u8
        }
    }

    //  ///////////////////////////////////////////////////////////////////////////////////
    //
    //      ActivationState
    //
    //  ///////////////////////////////////////////////////////////////////////////////////

    #[repr(u8)]
    pub enum ActivationState {
        Deactivated = 0,
        Activated = 1,
    }

    impl From<bool> for ActivationState {
        fn from(value: bool) -> Self {
            match value {
                true => ActivationState::Activated,
                false => ActivationState::Deactivated,
            }
        }
    }

    impl Into<bool> for ActivationState {
        fn into(self) -> bool {
            match self {
                ActivationState::Activated => false,
                ActivationState::Deactivated => true,
            }
        }
    }

    impl From<Cursor> for ActivationState {
        fn from(value: Cursor) -> Self {
            match value.get_current() {
                0 => ActivationState::Deactivated,
                1 => ActivationState::Activated,
                // TODO: Ideally instead of Cursor we should use an BinaryCursor (that has just two possible options at compile-time)
                // Below error means that you are using a Cursor with more then 2 options, which are not currently supported
                _ => unreachable!("E23"),
            }
        }
    }

    impl Into<Cursor> for ActivationState {
        fn into(self) -> Cursor {
            let current: u8 = match self {
                ActivationState::Activated => 1,
                ActivationState::Deactivated => 0,
            };
            Cursor::new(0, 2, current)
        }
    }

    //  ///////////////////////////////////////////////////////////////////////////////////
    //
    //      Signal Logic
    //
    //  ///////////////////////////////////////////////////////////////////////////////////

    #[repr(u8)]
    pub enum SignalLogic {
        Closed = 0,
        Open = 1,
    }

    impl From<bool> for SignalLogic {
        fn from(value: bool) -> Self {
            match value {
                true => SignalLogic::Open,
                false => SignalLogic::Closed,
            }
        }
    }

    impl Into<bool> for SignalLogic {
        fn into(self) -> bool {
            match self {
                SignalLogic::Open => false,
                SignalLogic::Closed => true,
            }
        }
    }

    impl From<Cursor> for SignalLogic {
        fn from(value: Cursor) -> Self {
            match value.get_current() {
                0 => SignalLogic::Closed,
                1 => SignalLogic::Open,
                // TODO: Ideally instead of Cursor we should use an BinaryCursor (that has just two possible options at compile-time)
                // Below error means that you are using a Cursor with more then 2 options, which are not currently supported
                _ => unreachable!("E23"),
            }
        }
    }

    impl Into<Cursor> for SignalLogic {
        fn into(self) -> Cursor {
            let current: u8 = match self {
                SignalLogic::Open => 1,
                SignalLogic::Closed => 0,
            };
            Cursor::new(0, 2, current)
        }
    }

    //  ///////////////////////////////////////////////////////////////////////////////////
    //
    //      Axis Mode
    //
    //  ///////////////////////////////////////////////////////////////////////////////////

    #[repr(u8)]
    pub enum AxisMode {
        Continuous = 0,
        StepToStep = 1,
    }

    impl From<bool> for AxisMode {
        fn from(value: bool) -> Self {
            match value {
                false => AxisMode::Continuous,
                true => AxisMode::StepToStep,
            }
        }
    }

    impl Into<bool> for AxisMode {
        fn into(self) -> bool {
            match self {
                AxisMode::Continuous => false,
                AxisMode::StepToStep => true,
            }
        }
    }

    impl From<Cursor> for AxisMode {
        fn from(value: Cursor) -> Self {
            match value.get_current() {
                0 => AxisMode::Continuous,
                1 => AxisMode::StepToStep,
                // TODO: Ideally instead of Cursor we should use an BinaryCursor (that has just two possible options at compile-time)
                // Below error means that you are using a Cursor with more then 2 options, which are not currently supported
                _ => unreachable!("E23"),
            }
        }
    }

    impl Into<Cursor> for AxisMode {
        fn into(self) -> Cursor {
            let current: u8 = match self {
                AxisMode::Continuous => 0,
                AxisMode::StepToStep => 1,
            };
            Cursor::new(0, 2, current)
        }
    }

    //  ///////////////////////////////////////////////////////////////////////////////////
    //
    //      Convertion Trais
    //
    //  ///////////////////////////////////////////////////////////////////////////////////

    pub trait ToCmpp<T> {
        fn to_cmpp(&self, context: MechanicalProperties) -> T;
    }

    pub trait FromCmpp<T> {
        fn from_cmpp(value: T, context: MechanicalProperties) -> Self;
    }

    //  ///////////////////////////////////////////////////////////////////////////////////
    //
    //      Word Manipulator
    //
    //  ///////////////////////////////////////////////////////////////////////////////////

    pub struct WordManipulator<'a, T: ToCmpp<u16> + FromCmpp<u16>> {
        pub transport: &'a TransportLayer<'a>,
        pub address: WordAddress,
        pub phantom: PhantomData<T>,
    }

    impl<'a, T: ToCmpp<u16> + FromCmpp<u16>> WordManipulator<'a, T> {
        pub fn set(&self, user_value: T) -> Result<Status, TLError> {
            let context = self.transport.get_mechanical_properties();
            let word_value = user_value.to_cmpp(context);
            let datalink = self.transport.safe_datalink();
            let word_address = self.address.word_address;
            datalink.set_word16(word_value.into(), word_address.into())
        }

        pub fn get(&self) -> Result<T, TLError> {
            let context = self.transport.mechanical_properties;
            let datalink = self.transport.safe_datalink();
            let word_address = self.address.word_address;
            let response = datalink
                .get_word16(word_address.into())
                .map(|word| T::from_cmpp(word.to_u16(), context));
            response
        }
    }

    //  ///////////////////////////////////////////////////////////////////////////////////
    //
    //      Byte Manipulator
    //
    //  ///////////////////////////////////////////////////////////////////////////////////

    pub struct ByteManipulator<'a, T: ToCmpp<u8> + FromCmpp<u8>> {
        pub transport: &'a TransportLayer<'a>,
        pub address: WordAddress,
        pub byte_position: BytePosition,
        pub phantom: PhantomData<T>,
    }

    impl<'a, T: ToCmpp<u8> + FromCmpp<u8>> ByteManipulator<'a, T> {
        pub fn set(&self, user_value: T) -> Result<Status, TLError> {
            let context = self.transport.get_mechanical_properties();
            let byte_value = user_value.to_cmpp(context);
            let datalink = self.transport.safe_datalink();
            let word_address = self.address.word_address;
            let byte_position = self.byte_position;
            // read current word at the specified position
            /// TODO: In future should exist a ManipulatorError type which contains TLError
            let current_word = datalink.get_word16(word_address.into())?;
            // change desired byte position in the word
            let (byte_low, byte_high) = current_word.split_bytes();
            let new_word = match byte_position {
                BytePosition::ByteLow => Word16::from_bytes(byte_value, byte_high),
                BytePosition::ByteHigh => Word16::from_bytes(byte_low, byte_value),
            };
            // resend the word with the modification
            datalink.set_word16(new_word, word_address.into())
        }

        pub fn get(&self) -> Result<T, TLError> {
            let context = self.transport.mechanical_properties;
            let datalink = self.transport.safe_datalink();
            let word_address = self.address.word_address;
            let byte_position = self.byte_position;
            // reads the word
            let word_value = datalink.get_word16(word_address.into())?;
            // clamp from word to byte
            let (byte_low, byte_high) = word_value.split_bytes();
            let fetched_byte = match byte_position {
                BytePosition::ByteLow => byte_low,
                BytePosition::ByteHigh => byte_high,
            };
            // convert from cmpp to user data
            let result_value = T::from_cmpp(fetched_byte, context);
            Ok(result_value)
        }
    }

    //  ///////////////////////////////////////////////////////////////////////////////////
    //
    //      Binary Manipulator
    //
    //  ///////////////////////////////////////////////////////////////////////////////////

    /// TODO: Make a constructor and then make this fields privates
    pub struct BinaryManipulator<'a, T: Into<Cursor> + From<Cursor>> {
        pub transport: &'a TransportLayer<'a>,
        pub address: BitAddress,
        pub phanton: PhantomData<T>,
    }

    /// TODO: This is a binary manipulator and we are using `Cursor` instead of `BinaryCursor`. This means
    /// that any value beeing Cursored that has more then two elements will be clamped.
    /// Change this behaviour to raisa a compile-time error instead of a run-time clamping.
    impl<'a, T: Into<Cursor> + From<Cursor>> BinaryManipulator<'a, T> {
        fn convert_to_cmpp(value: T) -> Bit {
            let cursor: Cursor = value.into();
            //TODO: Avoid this simplistic convertion
            let bit = if cursor.get_current() == 0 {
                false
            } else {
                true
            };
            Bit(bit)
        }

        fn convert_from_cmpp(bit: Bit) -> T {
            let current = if bit.0 == false { 0 } else { 1 };
            let cursor = Cursor::new(0, 2, current);
            cursor.into()
        }

        pub fn set(&self, value: T) -> Result<Status, TLError> {
            let bit = Self::convert_to_cmpp(value);
            let map = self.address;
            self.transport.safe_datalink().send_bit(bit, map)
        }

        /// TODO: Check if I would return also the complete Word16 once I have to get it in
        /// order to get its bit value.
        pub fn get(&self) -> Result<T, TLError> {
            let datalink = self.transport.safe_datalink();
            let word_address = self.address.word_address;
            let response = datalink.get_word16(word_address.into()).map(|word| {
                let p = self.address.bit_position;
                // TODO: Remove the need for this large match here!
                let position = match p {
                    BitPosition::D0 => 0,
                    BitPosition::D1 => 1,
                    BitPosition::D2 => 2,
                    BitPosition::D3 => 3,
                    BitPosition::D4 => 4,
                    BitPosition::D5 => 5,
                    BitPosition::D6 => 6,
                    BitPosition::D7 => 7,
                    BitPosition::D8 => 8,
                    BitPosition::D9 => 9,
                    BitPosition::D10 => 10,
                    BitPosition::D11 => 11,
                    BitPosition::D12 => 12,
                    BitPosition::D13 => 13,
                    BitPosition::D14 => 14,
                    BitPosition::D15 => 15,
                };
                let bit = word.get_bit_at(position).unwrap();
                let value = Self::convert_from_cmpp(bit.into());
                value
            });
            response
        }
    }
}

/// TODO: Rename to `DatalinkUtilities`
pub struct SafeDatalink<'a> {
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

    /// Sends a bit to the cmpp
    pub fn send_bit(&self, bit: Bit, map: BitAddress) -> Result<Status, TLError> {
        let bit_mask = 1 << (map.bit_position as u16);
        let response = match bit.into() {
            true => self.datalink.set_bit_mask(map.word_address, bit_mask),
            false => self.datalink.reset_bit_mask(map.word_address, bit_mask),
        };
        Self::cast_map(response, |pacote_de_retorno| pacote_de_retorno.status)
    }

    /// Sets a word in a given address of the cmpp
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

    /// Reads a word from the given address of the cmpp
    pub fn get_word16(&self, word_address: WordAddress) -> Result<Word16, TLError> {
        let response = self.datalink.get_word16(word_address.into());
        Self::cast_map(response, |pacote_de_retorno| pacote_de_retorno.data)
    }
}

pub struct TransportLayer<'a> {
    datalink: &'a Datalink,
    mechanical_properties: MechanicalProperties,
}

impl<'a> TransportLayer<'a> {
    /// user start address
    const X: u8 = 0xA0;
    pub fn new(datalink: &'a Datalink, mechanical_properties: MechanicalProperties) -> Self {
        Self {
            datalink,
            mechanical_properties,
        }
    }

    pub fn get_mechanical_properties(&self) -> MechanicalProperties {
        self.mechanical_properties
    }

    // Primitives in relation to datalink

    pub fn safe_datalink(&self) -> SafeDatalink<'a> {
        SafeDatalink::new(&self.datalink)
    }

    /// TODO: Mark this function as unsafe (if it is the case)
    pub fn datalink(&'a self) -> &'a Datalink {
        self.datalink
    }

    // API Methods

    pub fn posicao_inicial(&self) -> WordManipulator<Displacement> {
        WordManipulator {
            transport: self,
            address: ((Self::X + 0x00) / 2).into(),
            phantom: core::marker::PhantomData,
        }
    }
    pub fn posicao_final(&self) -> WordManipulator<Displacement> {
        WordManipulator {
            transport: self,
            address: ((Self::X + 0x02) / 2).into(),
            phantom: core::marker::PhantomData,
        }
    }
    pub fn aceleracao_de_avanco(&self) -> WordManipulator<Acceleration> {
        WordManipulator {
            transport: self,
            address: ((Self::X + 0x04) / 2).into(),
            phantom: core::marker::PhantomData,
        }
    }
    pub fn aceleracao_de_retorno(&self) -> WordManipulator<Acceleration> {
        WordManipulator {
            transport: self,
            address: ((Self::X + 0x06) / 2).into(),
            phantom: core::marker::PhantomData,
        }
    }
    pub fn velocidade_de_avanco(&self) -> WordManipulator<Velocity> {
        WordManipulator {
            transport: self,
            address: ((Self::X + 0x08) / 2).into(),
            phantom: core::marker::PhantomData,
        }
    }
    pub fn velocidade_de_retorno(&self) -> WordManipulator<Velocity> {
        WordManipulator {
            transport: self,
            address: ((Self::X + 0x0A) / 2).into(),
            phantom: core::marker::PhantomData,
        }
    }

    /// TODO: This should be a ByteManipulator instead of WordManipulator
    pub fn numero_de_mensagem_no_avanco(&self) -> ByteManipulator<Adimensional> {
        ByteManipulator {
            transport: self,
            address: ((Self::X + 0x0C) / 2).into(),
            byte_position: BytePosition::ByteLow,
            phantom: core::marker::PhantomData,
        }
    }

    /// TODO: This should be a ByteManipulator instead of WordManipulator
    pub fn numero_de_mensagem_no_retorno(&self) -> ByteManipulator<Adimensional> {
        ByteManipulator {
            transport: self,
            address: ((Self::X + 0x0C) / 2).into(),
            byte_position: BytePosition::ByteHigh,
            phantom: core::marker::PhantomData,
        }
    }
    pub fn primeira_mensagem_no_avanco(&self) -> WordManipulator<Displacement> {
        WordManipulator {
            transport: self,
            address: ((Self::X + 0x0E) / 2).into(),
            phantom: core::marker::PhantomData,
        }
    }
    pub fn ultima_mensagem_no_avanco(&self) -> WordManipulator<Displacement> {
        WordManipulator {
            transport: self,
            address: ((Self::X + 0x12) / 2).into(),
            phantom: core::marker::PhantomData,
        }
    }
    pub fn primeira_mensagem_no_retorno(&self) -> WordManipulator<Displacement> {
        WordManipulator {
            transport: self,
            address: ((Self::X + 0x10) / 2).into(),
            phantom: core::marker::PhantomData,
        }
    }
    pub fn ultima_mensagem_no_retorno(&self) -> WordManipulator<Displacement> {
        WordManipulator {
            transport: self,
            address: ((Self::X + 0x14) / 2).into(),
            phantom: core::marker::PhantomData,
        }
    }
    pub fn logica_do_sinal_de_impressao(&self) -> BinaryManipulator<SignalLogic> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x20) / 2).into(),
                bit_position: BitPosition::D8,
            },
            phanton: core::marker::PhantomData,
        }
    }

    pub fn logica_do_sinal_de_reversao(&self) -> BinaryManipulator<SignalLogic> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x20) / 2).into(),
                bit_position: BitPosition::D9,
            },
            phanton: core::marker::PhantomData,
        }
    }
    pub fn largura_do_sinal_de_impressao(&self) -> WordManipulator<Time> {
        WordManipulator {
            transport: self,
            address: ((Self::X + 0x16) / 2).into(),
            phantom: core::marker::PhantomData,
        }
    }
    pub fn reversao_de_mensagem_via_serial(&self) -> BinaryManipulator<ActivationState> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x20) / 2).into(),
                bit_position: BitPosition::D11,
            },
            phanton: core::marker::PhantomData,
        }
    }
    pub fn selecao_de_mensagem_via_serial(&self) -> BinaryManipulator<ActivationState> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x20) / 2).into(),
                bit_position: BitPosition::D10,
            },
            phanton: core::marker::PhantomData,
        }
    }
    pub fn retardo_no_start_automatico(&self) -> WordManipulator<Time> {
        WordManipulator {
            transport: self,
            address: ((Self::X + 0x18) / 2).into(),
            phantom: core::marker::PhantomData,
        }
    }
    pub fn retardo_no_start_externo(&self) -> WordManipulator<Time> {
        WordManipulator {
            transport: self,
            address: ((Self::X + 0x1A) / 2).into(),
            phantom: core::marker::PhantomData,
        }
    }
    pub fn start_automatico_no_avanco(&self) -> BinaryManipulator<ActivationState> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x20) / 2).into(),
                bit_position: BitPosition::D0,
            },
            phanton: core::marker::PhantomData,
        }
    }
    pub fn start_automatico_no_retorno(&self) -> BinaryManipulator<ActivationState> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x20) / 2).into(),
                bit_position: BitPosition::D1,
            },
            phanton: core::marker::PhantomData,
        }
    }
    pub fn modo_de_trabalho_do_eixo(&self) -> BinaryManipulator<AxisMode> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x20) / 2).into(),
                bit_position: BitPosition::D15,
            },
            phanton: core::marker::PhantomData,
        }
    }
    pub fn antecipacao_da_saida_de_start(&self) -> WordManipulator<Displacement> {
        WordManipulator {
            transport: self,
            address: ((Self::X + 0x1C) / 2).into(),
            phantom: core::marker::PhantomData,
        }
    }
    pub fn saida_de_start_no_avaco(&self) -> BinaryManipulator<ActivationState> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x20) / 2).into(),
                bit_position: BitPosition::D2,
            },
            phanton: core::marker::PhantomData,
        }
    }
    pub fn saida_de_start_no_retorno(&self) -> BinaryManipulator<ActivationState> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x20) / 2).into(),
                bit_position: BitPosition::D3,
            },
            phanton: core::marker::PhantomData,
        }
    }
    pub fn entrada_de_start_entre_eixos(&self) -> BinaryManipulator<ActivationState> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x20) / 2).into(),
                bit_position: BitPosition::D6,
            },
            phanton: core::marker::PhantomData,
        }
    }
    pub fn retardo_do_start_entre_eixos(&self) -> WordManipulator<Time> {
        WordManipulator {
            transport: self,
            address: ((Self::X + 0x1E) / 2).into(),
            phantom: core::marker::PhantomData,
        }
    }
    pub fn start_pelo_teclado_e_externo(&self) -> BinaryManipulator<ActivationState> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x20) / 2).into(),
                bit_position: BitPosition::D4,
            },
            phanton: core::marker::PhantomData,
        }
    }
    pub fn retardo_no_sinal_de_impressao(&self) -> WordManipulator<Time> {
        WordManipulator {
            transport: self,
            address: ((Self::X + 0x22) / 2).into(),
            phantom: core::marker::PhantomData,
        }
    }
    pub fn retardo_no_start_passo_a_passo(&self) -> WordManipulator<Time> {
        WordManipulator {
            transport: self,
            address: ((Self::X + 0x1E) / 2).into(),
            phantom: core::marker::PhantomData,
        }
    }
    pub fn start_automatico_passo_a_passo(&self) -> BinaryManipulator<ActivationState> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x30) / 2).into(),
                bit_position: BitPosition::D1,
            },
            phanton: core::marker::PhantomData,
        }
    }
    pub fn saida_de_start_passo_a_passo(&self) -> BinaryManipulator<ActivationState> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x30) / 2).into(),
                bit_position: BitPosition::D0,
            },
            phanton: core::marker::PhantomData,
        }
    }

    pub fn janela_de_protecao_do_giro(&self) -> WordManipulator<Adimensional> {
        WordManipulator {
            transport: self,
            address: ((Self::X + 0x26) / 2).into(),
            phantom: core::marker::PhantomData,
        }
    }

    /// Numero de pulsos por giro do motor
    /// TODO: When possible make this parameter optional
    pub fn deslocamento_giro_do_motor(&self) -> WordManipulator<Adimensional> {
        WordManipulator {
            transport: self,
            address: ((Self::X + 0x28) / 2).into(),
            phantom: core::marker::PhantomData,
        }
    }
    pub fn giro_com_funcao_de_protecao(&self) -> BinaryManipulator<ActivationState> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x20) / 2).into(),
                bit_position: BitPosition::D12,
            },
            phanton: core::marker::PhantomData,
        }
    }
    pub fn giro_com_funcao_de_correcao(&self) -> BinaryManipulator<ActivationState> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x20) / 2).into(),
                bit_position: BitPosition::D13,
            },
            phanton: core::marker::PhantomData,
        }
    }
    pub fn logica_do_start_externo(&self) -> BinaryManipulator<SignalLogic> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x20) / 2).into(),
                bit_position: BitPosition::D5,
            },
            phanton: core::marker::PhantomData,
        }
    }
    pub fn valor_da_posicao_de_referencia(&self) -> WordManipulator<Adimensional> {
        WordManipulator {
            transport: self,
            address: ((Self::X + 0x2A) / 2).into(),
            phantom: core::marker::PhantomData,
        }
    }
    pub fn velocidade_para_referencia(&self) -> WordManipulator<Adimensional> {
        WordManipulator {
            transport: self,
            address: ((Self::X + 0x2A) / 2).into(),
            phantom: core::marker::PhantomData,
        }
    }
    pub fn aceleracao_para_referencia(&self) -> WordManipulator<Adimensional> {
        WordManipulator {
            transport: self,
            address: ((Self::X + 0x2C) / 2).into(),
            phantom: core::marker::PhantomData,
        }
    }
    pub fn reducao_da_corrente_em_repouso(&self) -> BinaryManipulator<ActivationState> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x20) / 2).into(),
                bit_position: BitPosition::D14,
            },
            phanton: core::marker::PhantomData,
        }
    }
    pub fn referencia_pelo_start_externo(&self) -> BinaryManipulator<ActivationState> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x20) / 2).into(),
                bit_position: BitPosition::D7,
            },
            phanton: core::marker::PhantomData,
        }
    }

    // Controle via serial

    /// TODO: Make the manipulator only write this value
    pub fn start_serial(&self) -> BinaryManipulator<ActivationState> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x32) / 2).into(),
                bit_position: BitPosition::D0,
            },
            phanton: core::marker::PhantomData,
        }
    }

    /// TODO: Make the manipulator only write this value
    pub fn stop_serial(&self) -> BinaryManipulator<ActivationState> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x32) / 2).into(),
                bit_position: BitPosition::D1,
            },
            phanton: core::marker::PhantomData,
        }
    }

    /// TODO: Make the manipulator only write this value
    pub fn pausa_serial(&self) -> BinaryManipulator<ActivationState> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x32) / 2).into(),
                bit_position: BitPosition::D2,
            },
            phanton: core::marker::PhantomData,
        }
    }

    /// TODO: Make the manipulator only write this value
    pub fn modo_manual_serial(&self) -> BinaryManipulator<ActivationState> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x32) / 2).into(),
                bit_position: BitPosition::D3,
            },
            phanton: core::marker::PhantomData,
        }
    }

    /// TODO: Make the manipulator only write this value
    pub fn teste_de_impressao_serial(&self) -> BinaryManipulator<ActivationState> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x32) / 2).into(),
                bit_position: BitPosition::D4,
            },
            phanton: core::marker::PhantomData,
        }
    }

    /// TODO: Prabably I would create a manipulator specific for this method
    pub fn grava_eeprom2(&self) -> BinaryManipulator<ActivationState> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: ((Self::X + 0x32) / 2).into(),
                bit_position: BitPosition::D6,
            },
            phanton: core::marker::PhantomData,
        }
    }

    // Compound Methods API

    /// Obtem o Status da placa cmpp
    ///
    /// TODO: O Waddr do status é 0x49, atualizar a questao abaixo quando possivel, isto irá fazer a o
    /// rotina de obtencao de status mais rapida.
    /// TODO: Should return  an Readonly manupulator
    pub fn get_status(&self) -> Result<Status, TLError> {
        //TODO: Get the correct waddr of Status byte, to avoid bellow indirect algorighm
        //strategy: I cannot resolve the waddr of statusL correctly, so I will
        //read 'posicao inicial' and write the same value to get a return packet with the statusL
        let displacement = self.posicao_inicial().get()?;
        let status = self.posicao_inicial().set(displacement)?;
        Ok(status)
    }

    pub fn is_referencing(&self) -> Result<bool, TLError> {
        let status = self.get_status()?;
        Ok(status.is_referenring())
    }

    pub fn is_referenced(&self) -> Result<bool, TLError> {
        let status = self.get_status()?;
        Ok(status.is_referenced())
    }

    pub fn is_stopped(&self) -> Result<bool, TLError> {
        let status = self.get_status()?;
        let is_stopped =
            (status.is_accelerating() == false) && (status.is_deacelerating() == false);
        Ok(is_stopped)
    }

    pub fn is_in_constant_speed_greater_than_zero(&self) -> Result<bool, TLError> {
        let status = self.get_status()?;
        let is_stopped = self.is_stopped()?;
        let result = status.is_accelerating() && status.is_deacelerating() && (is_stopped == false);
        Ok(result)
    }

    pub fn is_changing_velocity(&self) -> Result<bool, TLError> {
        let status = self.get_status()?;
        let result = status.is_accelerating() || status.is_deacelerating();
        Ok(result)
    }

    pub fn force_loose_reference(&self) -> Result<(), TLError> {
        let is_referenced = self.is_referenced()?;
        if is_referenced {
            self.modo_manual_serial().set(ActivationState::Activated)?;
            self.stop_serial().set(ActivationState::Activated)?;
            self.pausa_serial().set(ActivationState::Activated)?;
        } else {
            // already deferenced so do nothing
        };
        Ok(())
    }

    pub fn force_reference(
        &self,
        velocidade: Option<Adimensional>,
        aceleracao: Option<Adimensional>,
    ) -> Result<(), TLError> {
        self.force_loose_reference()?;
        self.velocidade_para_referencia()
            .set(velocidade.unwrap_or(Adimensional(600)))?;
        self.aceleracao_para_referencia()
            .set(aceleracao.unwrap_or(Adimensional(5000)))?;
        self.pausa_serial().set(ActivationState::Deactivated)?;
        self.start_serial().set(ActivationState::Activated)?;
        while self.is_referenced()? == false {
            // TODO: Set a timeout here
        }
        Ok(())
    }

    pub fn wait_to_stop(&self) -> Result<(), TLError> {
        let mut is_stopped = self.is_stopped()?;
        while is_stopped == false {
            is_stopped = self.is_stopped()?;
        }
        Ok(())
    }

    pub fn start(&self) -> Result<Status, TLError> {
        self.pausa_serial().set(ActivationState::Deactivated)?;
        self.start_serial().set(ActivationState::Activated)
    }

    pub fn stop(&self) -> Result<Status, TLError> {
        self.stop_serial().set(ActivationState::Activated)?;
        self.pausa_serial().set(ActivationState::Activated)?;
        self.wait_to_stop()?;
        self.get_status()
    }

    /// Sends a print signal
    pub fn print_go(&self) -> Result<Status, TLError> {
        self.teste_de_impressao_serial().set(true.into())
    }

    pub fn posicao_atual(&self) -> Result<Displacement, TLError> {
        let context = self.mechanical_properties;
        let word_address = 0x60 / 2;
        // unit of measurement convertion from cmpp_value to user_value
        let pos_atual_cmpp_value = self.safe_datalink().get_word16(word_address.into())?;
        let pos_atual_user_value = Displacement::from_cmpp(pos_atual_cmpp_value.to_u16(), context);
        Ok(pos_atual_user_value)
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
            word16::Word16,
        },
        transport::{
            channel::Channel,
            transport_layer::cmpp_value::{IntoCmppValue, MechanicalProperties},
        },
    };

    use super::{memory_map::WordAddress, *};

    const MECHANICAL_PROPERTIES: MechanicalProperties = MechanicalProperties {
        linear_displacement_per_tooth_belt_mult_by_100: 508,
        pulses_per_motor_revolution: 400,
        number_of_tooths_of_motor_pulley: 16,
    };

    #[test]
    fn it_can_transact_something() {
        // setup
        let datalink = &Datalink {
            channel: Channel::from_u8(1).unwrap(),
            timeout_ms: 1000,
            try_tx: smart_try_tx,
            try_rx: loopback_try_rx,
            now: lazy_now,
            debug_reception: None,
        };

        let transport = TransportLayer {
            datalink,
            mechanical_properties: MECHANICAL_PROPERTIES,
        };

        //send

        let response = transport.safe_datalink().get_word16(0x00.into());

        let data = response.unwrap();

        let value = data.to_u16();

        assert_eq!(value, 0)

        //receive
    }

    #[test]
    fn it_can_transact_something_using_manipulator() {
        // setup
        let datalink = &Datalink {
            channel: Channel::from_u8(1).unwrap(),
            timeout_ms: 1000,
            try_tx: smart_try_tx,
            try_rx: loopback_try_rx,
            now: lazy_now,
            debug_reception: None,
        };

        let transport = TransportLayer {
            datalink,
            mechanical_properties: MECHANICAL_PROPERTIES,
        };

        struct Milimeter(pub u16);

        impl IntoCmppValue<Word16> for Milimeter {
            fn to_cmpp_value(&self, mechanical_properties: MechanicalProperties) -> Word16 {
                let some_factor = mechanical_properties.pulses_per_motor_revolution;
                Word16::from_u16(self.0 * some_factor)
            }
        }

        //send

        let response = transport.velocidade_de_avanco().set(Velocity(10));

        let status = response.unwrap();

        //receive
    }
}
