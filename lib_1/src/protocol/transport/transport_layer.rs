use crate::protocol::datalink::datalink::{
    word16::Word16, DLError, Datalink, PacodeDeRetornoDeSolicitacao, PacoteDeRetornoComErro,
    PacoteDeRetornoDeEnvio, Status,
};

use self::{
    cmpp_value::{Bit, MechanicalProperties},
    manipulator::WordSetter,
    memory_map::{BitAddress, BitPosition, WordAddress},
    new_proposal::{
        AccelerationManipulator, AdimensionalManipulator, BinaryManipulator,
        DisplacementManipulator, VelocityManipulator, __ActivationState, __TempManipulator,
    },
};

pub mod cmpp_value {
    use super::memory_map;

    #[derive(Clone, Copy)]
    pub struct MechanicalProperties {
        pub pulses_per_motor_revolution: u16,
        pub linear_displacement_per_tooth_belt: u16, // unit centh of milimeter
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

/// Transport Layer Error
#[derive(Debug)]
pub enum TLError {
    PacoteDeRetornoComErro(PacoteDeRetornoComErro),
    DLError(DLError),
}

pub mod manipulator {
    use crate::protocol::datalink::datalink::{word16::Word16, Status};

    use super::{
        cmpp_value::{self, Bit, IntoCmppValue},
        memory_map::{self, BitAddress, WordAddress},
        TLError, TransportLayer,
    };

    pub struct WordSetter<'a> {
        pub transport_layer: &'a TransportLayer<'a>,
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
        transport_layer: &'a TransportLayer<'a>,
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
        memory_map::{BitAddress, BitPosition, WordAddress},
        TLError, TransportLayer,
    };

    //  ///////////////////////////////////////////////////////////////////////////////////
    //
    //      DISPLACEMENT
    //
    //  ///////////////////////////////////////////////////////////////////////////////////

    pub struct Displacement(pub u16);

    pub struct DisplacementManipulator<'a> {
        pub transport: &'a TransportLayer<'a>,
        pub address: WordAddress,
    }

    impl From<u16> for Displacement {
        fn from(value: u16) -> Self {
            Displacement(value)
        }
    }

    impl<'a> DisplacementManipulator<'a> {
        // TODO: Not implemented yet, just a fake implementation
        fn convert_to_cmpp(d: Displacement, _p: MechanicalProperties) -> Word16 {
            let value = d.0;
            let new_value = d.0 * 1;
            new_value.into()
        }

        // TODO: Not implemented yet, just a fake implementation
        fn convert_from_cmpp(word: Word16, _p: MechanicalProperties) -> Displacement {
            let new_value = Displacement(word.into());
            new_value
        }

        pub fn set(&self, value: Displacement) -> Result<Status, TLError> {
            let properties = self.transport.mechanical_properties;
            let word_value = Self::convert_to_cmpp(value, properties);
            let datalink = self.transport.safe_datalink();
            let word_address = self.address.word_address;
            datalink.set_word16(word_value, word_address.into())
        }

        pub fn get(&self) -> Result<Displacement, TLError> {
            let properties = self.transport.mechanical_properties;
            let datalink = self.transport.safe_datalink();
            let word_address = self.address.word_address;
            let response = datalink
                .get_word16(word_address.into())
                .map(|word| Self::convert_from_cmpp(word, properties));
            response
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

    pub struct VelocityManipulator<'a> {
        pub transport: &'a TransportLayer<'a>,
        pub address: WordAddress,
    }

    impl<'a> VelocityManipulator<'a> {
        // TODO: Not implemented yet, just a fake implementation
        fn convert_to_cmpp(d: Velocity, _p: MechanicalProperties) -> Word16 {
            let value = d.0;
            let new_value = d.0 * 1;
            new_value.into()
        }

        // TODO: Not implemented yet, just a fake implementation
        fn convert_from_cmpp(word: Word16, _p: MechanicalProperties) -> Velocity {
            let new_value = Velocity(word.into());
            new_value
        }

        pub fn set(&self, value: Velocity) -> Result<Status, TLError> {
            let properties = self.transport.mechanical_properties;
            let word_value = Self::convert_to_cmpp(value, properties);
            let datalink = self.transport.safe_datalink();
            let word_address = self.address.word_address;
            datalink.set_word16(word_value.into(), word_address.into())
        }

        pub fn get(&self) -> Result<Velocity, TLError> {
            let properties = self.transport.mechanical_properties;
            let datalink = self.transport.safe_datalink();
            let word_address = self.address.word_address;
            let response = datalink
                .get_word16(word_address.into())
                .map(|word| Self::convert_from_cmpp(word, properties));
            response
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

    pub struct AccelerationManipulator<'a> {
        pub transport: &'a TransportLayer<'a>,
        pub address: WordAddress,
    }

    impl<'a> AccelerationManipulator<'a> {
        // TODO: Not implemented yet, just a fake implementation
        fn convert_to_cmpp(d: Acceleration, _p: MechanicalProperties) -> Word16 {
            let value = d.0;
            let new_value = d.0 * 1;
            new_value.into()
        }

        // TODO: Not implemented yet, just a fake implementation
        fn convert_from_cmpp(word: Word16, _p: MechanicalProperties) -> Acceleration {
            let new_value = Acceleration(word.into());
            new_value
        }

        pub fn set(&self, value: Acceleration) -> Result<Status, TLError> {
            let properties = self.transport.mechanical_properties;
            let word_value = Self::convert_to_cmpp(value, properties);
            let datalink = self.transport.safe_datalink();
            let word_address = self.address.word_address;
            datalink.set_word16(word_value.into(), word_address.into())
        }

        pub fn get(&self) -> Result<Acceleration, TLError> {
            let properties = self.transport.mechanical_properties;
            let datalink = self.transport.safe_datalink();
            let word_address = self.address.word_address;
            let response = datalink
                .get_word16(word_address.into())
                .map(|word| Self::convert_from_cmpp(word, properties));
            response
        }
    }

    //  ///////////////////////////////////////////////////////////////////////////////////
    //
    //      TEMP
    //
    //  ///////////////////////////////////////////////////////////////////////////////////

    pub struct __Temp(pub u16);

    impl From<Cursor> for __Temp {
        fn from(value: Cursor) -> Self {
            __Temp(0)
        }
    }

    impl From<u16> for __Temp {
        fn from(value: u16) -> Self {
            __Temp(0)
        }
    }

    pub struct __TempManipulator<'a> {
        pub transport: &'a TransportLayer<'a>,
        pub address: WordAddress,
    }

    impl<'a> __TempManipulator<'a> {
        // TODO: Not implemented yet, just a fake implementation
        fn convert_to_cmpp(d: __Temp, _p: MechanicalProperties) -> Word16 {
            let value = d.0;
            let new_value = d.0 * 1;
            new_value.into()
        }

        // TODO: Not implemented yet, just a fake implementation
        fn convert_from_cmpp(word: Word16, _p: MechanicalProperties) -> __Temp {
            let new_value = __Temp(word.into());
            new_value
        }

        pub fn set(&self, value: __Temp) -> Result<Status, TLError> {
            let properties = self.transport.mechanical_properties;
            let word_value = Self::convert_to_cmpp(value, properties);
            let datalink = self.transport.safe_datalink();
            let word_address = self.address.word_address;
            datalink.set_word16(word_value, word_address.into())
        }

        pub fn get(&self) -> Result<__Temp, TLError> {
            let properties = self.transport.mechanical_properties;
            let datalink = self.transport.safe_datalink();
            let word_address = self.address.word_address;
            let response = datalink
                .get_word16(word_address.into())
                .map(|word| Self::convert_from_cmpp(word, properties));
            response
        }
    }

    //  ///////////////////////////////////////////////////////////////////////////////////
    //
    //      Adimensional
    //
    //  ///////////////////////////////////////////////////////////////////////////////////

    pub struct Adimensional(pub u16);

    impl From<u16> for Adimensional {
        fn from(value: u16) -> Self {
            Adimensional(value)
        }
    }

    pub struct AdimensionalManipulator<'a> {
        pub transport: &'a TransportLayer<'a>,
        pub address: WordAddress,
    }

    impl<'a> AdimensionalManipulator<'a> {
        // TODO: Not implemented yet, just a fake implementation
        fn convert_to_cmpp(d: Adimensional, _p: MechanicalProperties) -> Word16 {
            let value = d.0;
            let new_value = d.0 * 1;
            new_value.into()
        }

        // TODO: Not implemented yet, just a fake implementation
        fn convert_from_cmpp(word: Word16, _p: MechanicalProperties) -> Adimensional {
            let new_value = Adimensional(word.into());
            new_value
        }

        pub fn set(&self, value: Adimensional) -> Result<Status, TLError> {
            let properties = self.transport.mechanical_properties;
            let word_value = Self::convert_to_cmpp(value, properties);
            let datalink = self.transport.safe_datalink();
            let word_address = self.address.word_address;
            datalink.set_word16(word_value.into(), word_address.into())
        }

        pub fn get(&self) -> Result<Adimensional, TLError> {
            let properties = self.transport.mechanical_properties;
            let datalink = self.transport.safe_datalink();
            let word_address = self.address.word_address;
            let response = datalink
                .get_word16(word_address.into())
                .map(|word| Self::convert_from_cmpp(word, properties));
            response
        }
    }

    //  ///////////////////////////////////////////////////////////////////////////////////
    //
    //      ActivationState
    //
    //  ///////////////////////////////////////////////////////////////////////////////////

    #[repr(u8)]
    pub enum __ActivationState {
        Activated = 1,
        Deactivated = 0,
    }

    impl From<bool> for __ActivationState {
        fn from(value: bool) -> Self {
            match value {
                true => __ActivationState::Activated,
                false => __ActivationState::Deactivated,
            }
        }
    }

    impl Into<bool> for __ActivationState {
        fn into(self) -> bool {
            match self {
                __ActivationState::Activated => false,
                __ActivationState::Deactivated => true,
            }
        }
    }

    impl From<Cursor> for __ActivationState {
        fn from(value: Cursor) -> Self {
            match value.get_current() {
                0 => __ActivationState::Deactivated,
                1 => __ActivationState::Activated,
                // TODO: Ideally instead of Cursor we should use an BinaryCursor (that has just two possible options at compile-time)
                // Below error means that you are using a Cursor with more then 2 options, which are not currently supported
                _ => unreachable!("E23"),
            }
        }
    }

    impl Into<Cursor> for __ActivationState {
        fn into(self) -> Cursor {
            let current: u8 = match self {
                __ActivationState::Activated => 1,
                __ActivationState::Deactivated => 0,
            };
            Cursor::new(0, 2, current)
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
        Self::cast_map(response, |pacote_de_retorno| pacote_de_retorno.data)
    }
}

pub struct TransportLayer<'a> {
    datalink: &'a Datalink,
    mechanical_properties: MechanicalProperties,
}

impl<'a> TransportLayer<'a> {
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

    // API Methods

    pub fn posicao_inicial(&self) -> DisplacementManipulator {
        DisplacementManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn posicao_final(&self) -> DisplacementManipulator {
        DisplacementManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn aceleracao_de_avanco(&self) -> AccelerationManipulator {
        AccelerationManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn aceleracao_de_retorno(&self) -> AccelerationManipulator {
        AccelerationManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn velocidade_de_avanco(&self) -> VelocityManipulator {
        VelocityManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn velocidade_de_retorno(&self) -> VelocityManipulator {
        VelocityManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn numero_de_mensagem_no_avanco(&self) -> AdimensionalManipulator {
        AdimensionalManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn numero_de_mensagem_no_retorno(&self) -> AdimensionalManipulator {
        AdimensionalManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn primeira_mensagem_no_avanco(&self) -> DisplacementManipulator {
        DisplacementManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn ultima_mensagem_no_avanco(&self) -> DisplacementManipulator {
        DisplacementManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn primeira_mensagem_no_retorno(&self) -> DisplacementManipulator {
        DisplacementManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn ultima_mensagem_no_retorno(&self) -> DisplacementManipulator {
        DisplacementManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn logica_do_sinal_de_impressao(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn largura_do_sinal_de_impressao(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn reversao_de_mensagem_via_serial(&'a self) -> BinaryManipulator<'a, __ActivationState> {
        BinaryManipulator {
            transport: self,
            address: BitAddress {
                word_address: 0xFF.into(),
                bit_position: BitPosition::D12,
            },
            phanton: core::marker::PhantomData,
        }
    }
    pub fn selecao_de_mensagem_via_serial(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn retardo_no_start_automatico(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn retardo_no_start_externo(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn start_automatico_no_avanco(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn start_automatico_no_retorno(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn modo_de_trabalho_do_eixo(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn antecipacao_da_saida_de_start(&self) -> DisplacementManipulator {
        DisplacementManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn saida_de_start_no_avaco(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn saida_de_start_no_retorno(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn entrada_de_start_entre_eixos(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn retardo_do_start_entre_eixos(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn start_pelo_teclado_e_externo(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn retardo_no_sinal_de_impressao(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn retardo_no_start_passo_a_passo(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn start_automatico_passo_a_passo(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn saida_de_start_passo_a_passo(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn numero_do_canal(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn numero_de_pulso_do_giro(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn janela_de_protecao_do_giro(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn deslocamento_giro_do_motor(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn giro_com_funcao_de_protecao(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn giro_com_funcao_de_correcao(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn logica_do_start_externo(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn valor_da_posicao_de_referencia(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn velocidade_para_referencia(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn aceleracao_para_referencia(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn reducao_da_corrente_em_repouso(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn referencia_pelo_start_externo(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
    pub fn modo_turbo(&self) -> __TempManipulator {
        __TempManipulator {
            transport: self,
            address: 0xFF.into(),
        }
    }
}

///////////////////////////////////

pub mod investigation_of_pattern {}

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
        linear_displacement_per_tooth_belt: 1234,
        pulses_per_motor_revolution: 2345,
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

        let response = transport.__velocidade_de_avanco().set(Milimeter(10));

        let status = response.unwrap();

        //receive
    }
}
