use crate::protocol::datalink::datalink::{
    DLError, Datalink, PacodeDeRetornoDeSolicitacao, PacoteDeRetornoComErro,
    PacoteDeRetornoDeEnvio, Status,
};

mod cmpp_value {
    use super::memory_map;

    pub trait ToCmpp<N> {
        fn to_cmpp_value(&self) -> N;
    }

    pub trait FromCmpp {
        fn from_cmpp_value(&self, cmpp_value: AnyCmppValue) -> Self;
    }

    pub enum AnyCmppValue {
        Bit(Bit),
        Word16(Word16),
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
}

mod memory_map {

    #[derive(Copy, Clone)]
    /// Also known as `cmpp command`
    /// TODO: Change name to WordAddress
    pub struct Word16 {
        pub word_address: u8,
    }

    impl Word16 {
        pub fn new(word_address: u8) -> Self {
            Self { word_address }
        }
    }

    impl From<u8> for Word16 {
        fn from(value: u8) -> Self {
            Self::new(value)
        }
    }

    impl Into<u8> for Word16 {
        fn into(self) -> u8 {
            self.word_address
        }
    }

    #[derive(Copy, Clone)]
    /// TODO: Change name to BitPosition, place it in lib1::Utils (there are other places where this similar
    /// kind of construction is used (search for 'Bit' like in IOExpander::Bit, etc). Put every application of
    /// this concept in a unique place and reuse it)
    pub enum BitAddress {
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
    ///TODO: Change Bit to `BitAddress` and bit_address to `BitPosition`. And then avoid name colision and the
    /// need to qualify namespaces `memory_map` and `cmpp_value`.
    pub struct Bit {
        /// TODO: Change to WordAddress type
        pub word_address: u8,
        /// value between 0..16 (inclusive, exclusive)
        pub bit_address: BitAddress,
    }
}

#[derive(Debug)]
pub enum TLError {
    PacoteDeRetornoComErro(PacoteDeRetornoComErro),
    DLError(DLError),
}

mod manipulator {
    use crate::protocol::datalink::datalink::Status;

    use super::{
        cmpp_value::{self, ToCmpp},
        memory_map, TLError, TransportLayer,
    };

    struct WordSetter<'a> {
        transport_layer: &'a TransportLayer,
        memory_map: memory_map::Word16,
    }

    impl<'a> WordSetter<'a> {
        pub fn set<T: ToCmpp<cmpp_value::Word16>>(&self, value: T) -> Result<Status, TLError> {
            let cmpp_value = value.to_cmpp_value();
            let word_address = self.memory_map;
            self.transport_layer
                .safe_datalink()
                .set_word16(cmpp_value, word_address)
        }
    }

    struct BitSetter<'a> {
        transport_layer: &'a TransportLayer,
        memory_map: memory_map::Bit,
    }

    impl<'a> BitSetter<'a> {
        pub fn set<T: ToCmpp<cmpp_value::Bit>>(&self, value: T) -> Result<Status, TLError> {
            let cmpp_value = value.to_cmpp_value();
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

    pub fn send_bit(&self, bit: cmpp_value::Bit, map: memory_map::Bit) -> Result<Status, TLError> {
        let bit_mask = 1 << (map.bit_address as u16);
        let response = match bit.into() {
            true => self.datalink.set_bit_mask(map.word_address, bit_mask),
            false => self.datalink.reset_bit_mask(map.word_address, bit_mask),
        };
        Self::cast_map(response, |pacote_de_retorno| pacote_de_retorno.status)
    }

    pub fn set_word16(
        &self,
        word_value: cmpp_value::Word16,
        word_address: memory_map::Word16,
    ) -> Result<Status, TLError> {
        let response = self
            .datalink
            .set_word16(word_address.into(), word_value.into());
        Self::cast_map(response, |pacote_de_retorno| pacote_de_retorno.status)
    }

    pub fn get_word16(
        &self,
        word_address: memory_map::Word16,
    ) -> Result<cmpp_value::Word16, TLError> {
        let response = self.datalink.get_word16(word_address.into());
        Self::cast_map(response, |pacote_de_retorno| {
            cmpp_value::Word16(pacote_de_retorno.data.to_u16())
        })
    }
}

struct TransportLayer {
    datalink: Datalink,
}

impl TransportLayer {
    // Primitives in relation to datalink

    fn safe_datalink<'a>(&'a self) -> SafeDatalink<'a> {
        SafeDatalink::new(&self.datalink)
    }
}

//////////////////////////////////////////////////////
// TESTS
/////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    use crate::protocol::datalink::datalink::{
        emulated::{lazy_now, loopback_try_rx, smart_try_tx},
        Channel,
    };

    use super::{memory_map::Word16, *};

    #[test]
    fn it_can_transact_something() {
        // setup
        let datalink = Datalink {
            channel: Channel::from_u8(1).unwrap(),
            timeout_ms: 1000,
            try_tx: smart_try_tx,
            try_rx: loopback_try_rx,
            now: lazy_now,
        };

        let transport = TransportLayer { datalink };

        //send

        let response = transport.safe_datalink().get_word16(0x00.into());

        let data = response.unwrap();

        let value = data.0;

        assert_eq!(value, 0)

        //receive
    }
}
