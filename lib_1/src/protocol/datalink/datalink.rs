///////////////////////////////////////////////////////////

use crate::utils::common::{get_bit_at, get_bit_at_as_bool, reset_bit_at, word_to_byte};

use super::{
    decoder::{Decoder, DecodingError},
    encoder::Encoder,
    frame::{Frame, Payload, SlaveFrame, SlaveFrameNack},
    prelude::{SlaveStartByte, StartByte},
};

use super::super::transport::channel::Channel;

/// DatalinkError.
///
/// TODO: Resolve conflict naming with other struct named 'DataLinkError', this here
/// is intended to be more generic
#[derive(Debug)]
pub enum DLError {
    /// Channel number must be between range 0..64 (inclusive, exclusive)
    InvalidChannel(u8),
    /// Low level (byte-level), serial TX function failed
    SerialTransmissionError,
    /// The incomming byte stream from slave does not obey the right syntax of the protocol or the checksum is wrong
    DecodingError(DecodingError),
    /// Timeout error (cannot wait response forever!)
    Timeout(u16),
    /// Low level (byte-level), serial RX function failed
    SerialReceptionError,
    /// If slave returns STX (which according to protocol v1 spec is not allowed, just
    /// Masters may use STX as start byte)
    SlaveHasReturnedStartByteAsNeitherAckNorNack,
    SlaveHasReturnedNack(SlaveFrame),
}

impl From<SlaveFrameNack> for DLError {
    fn from(frame: SlaveFrameNack) -> Self {
        let slave_frame = frame.0;
        DLError::SlaveHasReturnedNack(slave_frame)
    }
}

///////////////////////////////////////////////////////////

const BIT_0: u8 = 0;
const BIT_1: u8 = 1;
const BIT_2: u8 = 2;
const BIT_3: u8 = 3;
const BIT_4: u8 = 4;
const BIT_5: u8 = 5;
const BIT_6: u8 = 6;
const BIT_7: u8 = 7;

/// Status of the cmpp. For more details see the cmpp v1 protocol specification;
///
/// NOTE: Byte High is not present because according to spec it is "Reserved
/// for special applications".
#[derive(Copy, Clone, Debug)]
pub struct Status {
    low: u8,
    //high: None,
}

impl Status {
    fn new_from_raw(byte_low: u8) -> Self {
        Self { low: byte_low }
    }

    /// If cmpp is referenced
    pub fn is_referenced(&self) -> bool {
        get_bit_at_as_bool(self.low, BIT_0)
    }

    /// If last position has been reached
    pub fn last_position_was_reached(&self) -> bool {
        get_bit_at_as_bool(self.low, BIT_1)
    }

    /// If cmpp is in the proccess of referecing the axis
    pub fn is_referenring(&self) -> bool {
        get_bit_at_as_bool(self.low, BIT_2)
    }

    /// if direction of the axis is forward
    pub fn is_positive_moving_direction(&self) -> bool {
        get_bit_at_as_bool(self.low, BIT_3)
    }

    /// if motor is accelerating
    pub fn is_accelerating(&self) -> bool {
        get_bit_at_as_bool(self.low, BIT_4)
    }

    /// if motor is deacelerating
    pub fn is_deacelerating(&self) -> bool {
        get_bit_at_as_bool(self.low, BIT_5)
    }

    // BIT_6 is reserved
    fn reserved(&self) -> bool {
        get_bit_at_as_bool(self.low, BIT_6)
    }

    /// If true indicates an error event, and ErrorMask must be consulted
    pub fn has_an_error_event(&self) -> bool {
        get_bit_at_as_bool(self.low, BIT_7)
    }
}

///////////////////////////////////////////////////////////

const DIR_GET: u8 = (0 << BIT_7) + (0 << BIT_6);
const DIR_RESET_BIT_MASK: u8 = (0 << BIT_7) + (1 << BIT_6);
const DIR_SET_BIT_MASK: u8 = (1 << BIT_7) + (0 << BIT_6);
const DIR_SET: u8 = (1 << BIT_7) + (1 << BIT_6);

/// Direction to be used in the MasterFrame which is sent to cmpp. For more details see the cmpp v1 protocol specification.
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Direction {
    Get = DIR_GET,
    ResetBitmask = DIR_RESET_BIT_MASK,
    SetBitmask = DIR_SET_BIT_MASK,
    Set = DIR_SET,
}

impl Direction {
    pub fn from_u8(value: u8) -> Result<Direction, ()> {
        match value {
            DIR_GET => Ok(Self::Get),
            DIR_RESET_BIT_MASK => Ok(Self::ResetBitmask),
            DIR_SET_BIT_MASK => Ok(Self::SetBitmask),
            DIR_SET => Ok(Self::Set),
            _ => Err(()),
        }
    }

    /// Only considers bit 7 and 6 of the u8 value
    pub fn from_u8_unchecked(value: u8) -> Direction {
        let bit_7 = get_bit_at(value, BIT_7);
        let bit_6 = get_bit_at(value, BIT_6);
        let direction_number = (bit_7 << 7) + (bit_6 << 6);
        Direction::from_u8(direction_number).unwrap_or_else(|_| {
            // Should never happen
            panic!("E448")
        })
    }
}

impl TryFrom<u8> for Direction {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_u8(value)
    }
}

pub struct DirectionAndChannel {
    /// Acording protocol:
    /// Bits 6 and 7 are for direction
    /// Bits 0 to 5 are for represent the channel number
    direction_and_number: u8,
}

impl DirectionAndChannel {
    pub fn from_raw(direction_and_channel: u8) -> Self {
        Self {
            direction_and_number: direction_and_channel,
        }
    }

    pub fn get_channel(&self) -> Channel {
        let temp = reset_bit_at(self.direction_and_number, BIT_7);
        let number = reset_bit_at(temp, BIT_6);
        // SAFETY: It's safe to call this function because is garanteed that a 6 bits
        // unsigned integer is inside current 0..Channel::MAX_CHANNELS (inclusive, exclusive)
        // range.
        Channel::from_u8(number).unwrap()
    }

    pub fn get_direction(&self) -> Direction {
        Direction::from_u8_unchecked(self.direction_and_number)
    }
}

///////////////////////////////////////////////////////////

/// Represents a 16 bits word
#[derive(Copy, Clone)]
pub struct Word16 {
    data: u16,
}

impl Word16 {
    pub fn from_u16(data: u16) -> Self {
        Self { data }
    }

    pub fn to_u16(&self) -> u16 {
        self.data
    }

    /// Constructs a 16 bits word from two bytes
    pub fn from_bytes(byte_low: u8, byte_high: u8) -> Self {
        let value: u16 = ((byte_high as u16) * 256) + (byte_low as u16);
        Self { data: value }
    }

    /// Splits the word16 into (byte_low, byte_high)
    pub fn split_bytes(&self) -> (u8, u8) {
        word_to_byte(self.data)
    }

    pub fn get_byte_low(&self) -> u8 {
        self.split_bytes().0
    }

    pub fn get_byte_high(&self) -> u8 {
        self.split_bytes().1
    }

    /// if `bit position` is outside `0..15` returns None, because Word16 is just 16 bits long.
    pub fn get_bit_at(&self, position: u8) -> Option<bool> {
        if position < 16 {
            if position < 8 {
                //byte_low
                Some(get_bit_at_as_bool(self.get_byte_low(), position))
            } else {
                //byte_high
                let position__ = position - 8; // rotate bits
                Some(get_bit_at_as_bool(self.get_byte_high(), position__))
            }
        } else {
            None
        }
    }
}

impl Into<u16> for Word16 {
    fn into(self) -> u16 {
        self.to_u16()
    }
}

///////////////////////////////////////////////////////////

/// ByteDeErro is defined according to cmpp's protocol v1 specification. For more details see specification.
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum ByteDeErro {
    StartByteInvalidSTX = 1,
    EstruturaDePacoteDeComunicacaoInvalido02 = 2,
    EstruturaDePacoteDeComunicacaoInvalido03 = 3,
    EstruturaDePacoteDeComunicacaoInvalido04 = 4,
    EstruturaDePacoteDeComunicacaoInvalido05 = 5,
    EstruturaDePacoteDeComunicacaoInvalido06 = 6,
    EstruturaDePacoteDeComunicacaoInvalido07 = 7,
    EstruturaDePacoteDeComunicacaoInvalido08 = 8,
    EstruturaDePacoteDeComunicacaoInvalido09 = 9,
    NaoUsado10 = 10,
    EndByteInvalidETX = 11,
    TimerIn = 12,
    NaoUsado13 = 13,
    Framming = 14,
    OverRun = 15,
    BufferDeRecepcaoCheio = 16,
    CheckSum = 17,
    BufferAuxiliarOcupado = 18,
    SequenciaDeByteEnviadaMuitoGrande = 19,
}

/// Is it possible to slave send an `specification undefined` value, in this case it will be casted to type UnknownByteDeErro
#[derive(Copy, Clone, Debug)]
pub struct UnknownByteDeErro {
    data: u8,
}

impl TryFrom<u8> for ByteDeErro {
    type Error = UnknownByteDeErro;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        // TODO: If possible change this match to a simplified if statement for u8 convertion
        match value {
            1 => Ok(ByteDeErro::StartByteInvalidSTX),
            2 => Ok(ByteDeErro::EstruturaDePacoteDeComunicacaoInvalido02),
            3 => Ok(ByteDeErro::EstruturaDePacoteDeComunicacaoInvalido03),
            4 => Ok(ByteDeErro::EstruturaDePacoteDeComunicacaoInvalido04),
            5 => Ok(ByteDeErro::EstruturaDePacoteDeComunicacaoInvalido05),
            6 => Ok(ByteDeErro::EstruturaDePacoteDeComunicacaoInvalido06),
            7 => Ok(ByteDeErro::EstruturaDePacoteDeComunicacaoInvalido07),
            8 => Ok(ByteDeErro::EstruturaDePacoteDeComunicacaoInvalido08),
            9 => Ok(ByteDeErro::EstruturaDePacoteDeComunicacaoInvalido09),
            10 => Ok(ByteDeErro::NaoUsado10),
            11 => Ok(ByteDeErro::EndByteInvalidETX),
            12 => Ok(ByteDeErro::TimerIn),
            13 => Ok(ByteDeErro::NaoUsado13),
            14 => Ok(ByteDeErro::Framming),
            15 => Ok(ByteDeErro::OverRun),
            16 => Ok(ByteDeErro::BufferDeRecepcaoCheio),
            17 => Ok(ByteDeErro::CheckSum),
            18 => Ok(ByteDeErro::BufferAuxiliarOcupado),
            19 => Ok(ByteDeErro::SequenciaDeByteEnviadaMuitoGrande),
            // otherwise
            _ => Err(UnknownByteDeErro { data: value }),
        }
    }
}

///////////////////////////////////////////////////////////

pub struct PacoteDeRetorno_<T> {
    content: T,
}

impl<T> PacoteDeRetorno_<T> {}

/// TODO: Consider to change this to just `Word16` in all dependencies
pub struct RespostaDeSolicitacao {
    data: Word16,
}

/// TODO: Consider to change this to just `Status` in all dependencies
#[derive(Copy, Clone)]
pub struct RespostaDeEnvio {
    status: Status,
}

#[derive(Copy, Clone)]
pub struct RespostaComErro {
    byte_de_erro: Result<ByteDeErro, UnknownByteDeErro>,
    status: Status,
}

impl PacoteDeRetorno_<RespostaDeSolicitacao> {
    fn get_word(&self) -> Word16 {
        self.content.data
    }
}

impl From<SlaveFrame> for PacoteDeRetorno_<RespostaDeSolicitacao> {
    fn from(slave_frame: SlaveFrame) -> Self {
        let word = slave_frame.payload.get_word();
        Self {
            content: RespostaDeSolicitacao { data: word },
        }
    }
}

impl PacoteDeRetorno_<RespostaDeEnvio> {
    fn get_status(&self) -> Status {
        self.content.status
    }
}

impl From<SlaveFrame> for PacoteDeRetorno_<RespostaDeEnvio> {
    fn from(slave_frame: SlaveFrame) -> Self {
        let byte_low = slave_frame.payload.byte_low;
        let status = Status::new_from_raw(byte_low);
        Self {
            content: RespostaDeEnvio { status },
        }
    }
}

impl PacoteDeRetorno_<RespostaComErro> {
    fn get_error(&self) -> RespostaComErro {
        self.content
    }
}

/// When master sends an Direction::{ Set, SetBitMask, ResetBitMask} MasterFrame this is what is expected the slave to respond if
/// no error happens and slave responds with ACK.
pub struct PacoteDeRetornoDeEnvio {
    pub status: Status,
}

impl PacoteDeRetornoDeEnvio {
    pub fn from_slave_frame(slave_frame: SlaveFrame) -> Self {
        let [_, _, byte_low, _] = slave_frame.payload.as_array();
        Self {
            status: Status::new_from_raw(byte_low),
        }
    }
}

/// When master sends an Direction::get MasterFrame this is what is expected the slave to respond if
/// no error happens and slave responds with ACK.
pub struct PacodeDeRetornoDeSolicitacao {
    pub data: Word16,
}

impl PacodeDeRetornoDeSolicitacao {
    pub fn from_slave_frame(slave_frame: SlaveFrame) -> Self {
        let [_, _, byte_low, byte_high] = slave_frame.payload.as_array();
        let word = Word16::from_bytes(byte_low, byte_high);
        Self { data: word }
    }
}

/// When master sends any Direction MasterFrame this is what is expected the slave to respond if it did
/// not accepeted or understood the master request
#[derive(Debug)]
pub struct PacoteDeRetornoComErro {
    byte_de_erro: Result<ByteDeErro, UnknownByteDeErro>,
    status: Status,
}

impl PacoteDeRetornoComErro {
    pub fn from_slave_frame(slave_frame: SlaveFrame) -> Self {
        let [_, _, byte_low, byte_high] = slave_frame.payload.as_array();
        let byte_de_erro = byte_low.try_into();
        let status = Status::new_from_raw(byte_high);
        Self {
            byte_de_erro,
            status,
        }
    }
}

///////////////////////////////////////////////////////////

/// A cmpp Datalink is capable to send and receive data frames. It basically implements the cmpp protocol v1 specification.
/// See official specification for more details.
pub struct Datalink {
    pub channel: Channel,
    ///timeout in miliseconds
    /// TODO: Study to change this to u8 to reduce memory footprint
    pub timeout_ms: u16,
    /// None if some error happened
    pub try_tx: fn(u8) -> Option<()>,
    /// Ok_None if nothing to receive, Err if some error happened, Ok_Some if a byte has been received
    pub try_rx: fn() -> Result<Option<u8>, ()>,
    /// Returns miliseconds elapsed since `Epoch` (when machine was turned on)
    /// TODO: Make this function more memory eficient, because I think it's not necessary
    /// two u64 to calculate elapsed time like: `let time_elapsed = now() - start_time;`
    /// eventually something with the size statically defined by user should be better
    pub now: fn() -> u64,
}

impl Datalink {
    /// Creates a frame to be sent by the master
    fn encode_data(
        channel: Channel,
        direction: Direction,
        word_address: u8,
        word_value: u16,
    ) -> Encoder {
        let start_byte = StartByte::STX;
        let (byte_low, byte_high) = Word16::from_u16(word_value).split_bytes();
        let direction_and_channel: u8 = channel.to_u8() + direction as u8;
        let payload: Payload = [direction_and_channel, word_address, byte_low, byte_high].into();
        let frame = Frame::new(start_byte, payload);
        let encoded = Encoder::new(frame);
        encoded
    }

    fn transmit(&self, encoded_data: Encoder) -> Result<(), DLError> {
        let send_byte = (self.try_tx);
        for byte in encoded_data {
            if let None = send_byte(byte) {
                return Err(DLError::SerialTransmissionError);
            };
        }
        Ok(())
    }

    /// Synchronously reads RX waiting for stream of bytes to decode, considering the
    /// specified timeout time.
    /// I reads a response Frame but does not checks if the Frame is an SlaveFrame
    /// (start_byte equals 'ACK' or 'NACK'). Alternatively use [`Self::receive`].
    fn receive_frame(&self) -> Result<Frame, DLError> {
        let now = (self.now);
        let start_time = now();
        let timeout = self.timeout_ms;
        let try_receive_some_byte_from_serial = (self.try_rx);
        let mut decoder = Decoder::new();
        // Loops until something meaningful happens
        loop {
            match try_receive_some_byte_from_serial() {
                // Ok we received some byte !
                Ok(Some(byte)) => {
                    match decoder.parse_next(byte) {
                        // Still decoding; after considering incomming byte continue to wait next byte.
                        Ok(None) => {}

                        // Complete frame Successfully received.
                        Ok(Some(frame)) => {
                            return Ok(frame);
                        }

                        // An Error has happned while decoding incomming stream, return the error.
                        Err(decoding_error) => return Err(DLError::DecodingError(decoding_error)),
                    }
                }

                // No byte available in this turn, check for timeout time to decide if is possible to continue waiting for.
                Ok(None) => {
                    let time_elapsed = now() - start_time;
                    if time_elapsed > (self.timeout_ms as u64) {
                        return Err(DLError::Timeout(self.timeout_ms));
                    }
                }

                // Recepetion error occuried
                Err(_) => return Err(DLError::SerialReceptionError),
            }
        }
    }

    /// Perform all reception check plus check if the response is not a MasterFrame (start_byte equals STX)
    fn receive(&self) -> Result<SlaveFrame, DLError> {
        let received_frame = self.receive_frame()?;
        // check if it is an slave frame and return it
        received_frame
            .try_into()
            .map_err(|_| DLError::SlaveHasReturnedStartByteAsNeitherAckNorNack)
    }

    fn transact(
        &self,
        direction: Direction,
        word_address: u8,
        word_value: u16,
    ) -> Result<SlaveFrame, DLError> {
        // Send
        let encoded = Self::encode_data(self.channel, direction, word_address, word_value);
        self.transmit(encoded)?;
        // Receive
        self.receive()
    }

    /// TODO: reduce code surface when possible (reduce redundance)
    fn cast_to_pacote_de_retorno_solicitacao(
        slave_frame: SlaveFrame,
    ) -> Result<PacodeDeRetornoDeSolicitacao, PacoteDeRetornoComErro> {
        match slave_frame.start_byte {
            SlaveStartByte::ACK => Ok(PacodeDeRetornoDeSolicitacao::from_slave_frame(slave_frame)),
            SlaveStartByte::NACK => Err(PacoteDeRetornoComErro::from_slave_frame(slave_frame)),
        }
    }

    /// TODO: reduce code surface when possible (reduce redundance)
    fn cast_to_pacote_de_retorno_envio(
        slave_frame: SlaveFrame,
    ) -> Result<PacoteDeRetornoDeEnvio, PacoteDeRetornoComErro> {
        match slave_frame.start_byte {
            SlaveStartByte::ACK => Ok(PacoteDeRetornoDeEnvio::from_slave_frame(slave_frame)),
            SlaveStartByte::NACK => Err(PacoteDeRetornoComErro::from_slave_frame(slave_frame)),
        }
    }

    // Public API

    pub fn get_word16(
        &self,
        word_address: u8,
    ) -> Result<Result<PacodeDeRetornoDeSolicitacao, PacoteDeRetornoComErro>, DLError> {
        let direction = Direction::Get;
        let word_value = 0x00; // according spec this value does not matter
        self.transact(direction, word_address, word_value)
            .map(|slave_frame| Self::cast_to_pacote_de_retorno_solicitacao(slave_frame))
    }

    pub fn reset_bit_mask(
        &self,
        word_address: u8,
        bit_mask: u16,
    ) -> Result<Result<PacoteDeRetornoDeEnvio, PacoteDeRetornoComErro>, DLError> {
        let direction = Direction::Get;
        self.transact(direction, word_address, bit_mask)
            .map(|slave_frame| Self::cast_to_pacote_de_retorno_envio(slave_frame))
    }

    pub fn set_bit_mask(
        &self,
        word_address: u8,
        bit_mask: u16,
    ) -> Result<Result<PacoteDeRetornoDeEnvio, PacoteDeRetornoComErro>, DLError> {
        let direction = Direction::Get;
        self.transact(direction, word_address, bit_mask)
            .map(|slave_frame| Self::cast_to_pacote_de_retorno_envio(slave_frame))
    }

    pub fn set_word16(
        &self,
        word_address: u8,
        word_value: u16,
    ) -> Result<Result<PacoteDeRetornoDeEnvio, PacoteDeRetornoComErro>, DLError> {
        let direction = Direction::Set;
        self.transact(direction, word_address, word_value)
            .map(|slave_frame| Self::cast_to_pacote_de_retorno_envio(slave_frame))
    }

    // Testing new api

    fn request<U: From<SlaveFrame>>(
        &self,
        direction: Direction,
        word_address: u8,
        word_value: u16,
    ) -> Result<U, DLError> {
        let response = self.transact(direction, word_address, word_value)?;
        let response_ack = response.kind()?;
        let pacote: U = response.into();
        Ok(pacote)
    }

    pub fn get_word16_(
        &self,
        word_address: u8,
    ) -> Result<PacoteDeRetorno_<RespostaDeSolicitacao>, DLError> {
        let word_value = 0x00; // according spec this value does not matter
        self.request(Direction::Get, word_address, word_value)
    }

    fn set_word16_(
        &self,
        word_address: u8,
        word_value: u16,
    ) -> Result<PacoteDeRetorno_<RespostaDeEnvio>, DLError> {
        self.request(Direction::Set, word_address, word_value)
    }

    pub fn reset_bit_mask_(
        &self,
        word_address: u8,
        bit_mask: u16,
    ) -> Result<PacoteDeRetorno_<RespostaDeEnvio>, DLError> {
        self.request(Direction::ResetBitmask, word_address, bit_mask)
    }

    pub fn set_bit_mask_(
        &self,
        word_address: u8,
        bit_mask: u16,
    ) -> Result<PacoteDeRetorno_<RespostaDeEnvio>, DLError> {
        self.request(Direction::SetBitmask, word_address, bit_mask)
    }
}

//////////////////////////////////////////////////////
// Emulation Mock (useful for tests)
/////////////////////////////////////////////////////////

pub mod emulated {
    use heapless::Deque;

    use crate::protocol::datalink::{
        decoder::{Decoder, DecodingError},
        encoder::Encoder,
        prelude::StartByte,
    };

    /// Does never timeout ;)! Because time does not pass :D !
    pub fn lazy_now() -> u64 {
        0
    }

    /// emulated server's buffer
    static mut server: Deque<u8, 20> = Deque::new();
    pub fn try_tx(byte: u8) -> Option<()> {
        unsafe { server.push_back(byte).ok() }
    }

    //
    static mut decoder: Decoder = Decoder::new();

    /// This Rx function will receive any encoded master frame, but change
    /// the start_byte from STX to ACK.
    pub fn smart_try_tx(byte: u8) -> Option<()> {
        let parsed = unsafe { decoder.parse_next(byte) };
        //decode master data
        match parsed {
            Ok(Some(mut frame)) => {
                frame.start_byte = StartByte::ACK;
                let encoder = Encoder::new(frame);
                for byte in encoder {
                    // reinject data into buffer
                    match try_tx(byte) {
                        Some(_) => {
                            // sending to server's buffer
                        }
                        None => unreachable!(),
                    }
                }
            }
            Ok(None) => {
                // still parsing
            }
            Err(error) => {
                match error {
                    DecodingError::InvalidStartByte(_) => assert!(false, "InvalidStartByte"),
                    DecodingError::BufferOverFlow => assert!(false, "BufferOverFlow"),
                    DecodingError::ExpectedEtxOrEscDupButFoundOtherThing(_) => {
                        assert!(false, "ExpectedEtxOrEscDupButFoundOtherThing")
                    }
                    DecodingError::ChecksumIsEscButNotDuplicated(_) => {
                        assert!(false, "ChecksumIsEscButNotDuplicated")
                    }
                    DecodingError::InvalidChecksum { expected, received } => {
                        assert!(false, "InvalidChecksum")
                    }
                }
                //unreachable!("Master is expected to always be an well-formed frame")
            }
        };

        Some(())
    }

    /// if master sends a master slave, receives exactly a master slave.
    pub fn loopback_try_rx() -> Result<Option<u8>, ()> {
        unsafe { Ok(server.pop_front()) }
    }

    /// it decode what it received from master then give it back, but it
    /// changes the start_byte from 'STX' to 'ACK'
    pub fn convert_ok_master_into_ok_slave() {}
}

//////////////////////////////////////////////////////
// TESTS
/////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_can_mock_the_serial_for_one_byte_transaction() {
        // setup
        let probe = 0x10;

        //send
        if let Some(_) = emulated::try_tx(probe) {
            assert!(true)
        } else {
            assert!(false)
        }

        //receive
        if let Ok(Some(byte)) = emulated::loopback_try_rx() {
            assert_eq!(byte, probe)
        }
    }

    #[test]
    fn it_can_mock_the_serial_for_an_entire_frame() {
        // setup
        let payload: Payload = [0, 1, 2, 3].into();
        let frame = Frame::make_master_block(payload);
        let encoder = Encoder::new(frame);
        let mut decoder = Decoder::new();
        use emulated::{loopback_try_rx, try_tx};
        let mut check: u8 = 0;

        //send
        for byte in encoder {
            if let None = try_tx(byte) {
                assert!(false, "TX mocked should never fail")
            }
        }

        // receive
        loop {
            if let Ok(Some(byte)) = loopback_try_rx() {
                if let Ok(Some(frame)) = decoder.parse_next(byte) {
                    let expected = frame.payload;
                    assert_eq!(payload, expected, "Correctly decoded the sent payload");
                    check += 1;
                    break;
                }
            }
        }

        assert_eq!(check, 1, "All important steps performed (checked)!");
    }

    #[test]
    fn it_can_send_data_through_datalink() {
        // setup
        let payload: Payload = [0, 1, 2, 3].into();
        let frame = Frame::make_master_block(payload);
        let encoder = Encoder::new(frame);
        let mut decoder = Decoder::new();
        use emulated::{lazy_now, loopback_try_rx, smart_try_tx};
        let mut check: u8 = 0;

        let datalink = Datalink {
            channel: Channel::from_u8(1).unwrap(),
            timeout_ms: 1000,
            try_tx: smart_try_tx,
            try_rx: loopback_try_rx,
            now: lazy_now,
        };

        // run
        let word_address = 0x12;
        let word_value = 0x34;
        match datalink.set_word16(word_address, word_value) {
            Ok(Ok(pacote_de_retorno)) => {
                let PacoteDeRetornoDeEnvio { status } = pacote_de_retorno;
                assert!(true);
                check += 1;
            }

            Ok(Err(pacote_de_erro)) => {
                let PacoteDeRetornoComErro {
                    byte_de_erro,
                    status,
                } = pacote_de_erro;
                match byte_de_erro {
                    Ok(byte_de_erro_conhecido) => match byte_de_erro_conhecido {
                        ByteDeErro::StartByteInvalidSTX => {
                            assert!(false, "StartByteInvalidSTX");
                        }
                        ByteDeErro::EstruturaDePacoteDeComunicacaoInvalido02 => {
                            assert!(false, "EstruturaDePacoteDeComunicacaoInvalido02");
                        }
                        ByteDeErro::EstruturaDePacoteDeComunicacaoInvalido03 => {
                            assert!(false, "EstruturaDePacoteDeComunicacaoInvalido03");
                        }
                        ByteDeErro::EstruturaDePacoteDeComunicacaoInvalido04 => {
                            assert!(false, "EstruturaDePacoteDeComunicacaoInvalido04");
                        }
                        ByteDeErro::EstruturaDePacoteDeComunicacaoInvalido05 => {
                            assert!(false, "EstruturaDePacoteDeComunicacaoInvalido05");
                        }
                        ByteDeErro::EstruturaDePacoteDeComunicacaoInvalido06 => {
                            assert!(false, "EstruturaDePacoteDeComunicacaoInvalido06");
                        }
                        ByteDeErro::EstruturaDePacoteDeComunicacaoInvalido07 => {
                            assert!(false, "EstruturaDePacoteDeComunicacaoInvalido07");
                        }
                        ByteDeErro::EstruturaDePacoteDeComunicacaoInvalido08 => {
                            assert!(false, "EstruturaDePacoteDeComunicacaoInvalido08");
                        }
                        ByteDeErro::EstruturaDePacoteDeComunicacaoInvalido09 => {
                            assert!(false, "EstruturaDePacoteDeComunicacaoInvalido09");
                        }
                        ByteDeErro::NaoUsado10 => {
                            assert!(false, "NaoUsado10");
                        }
                        ByteDeErro::EndByteInvalidETX => {
                            assert!(false, "EndByteInvalidETX");
                        }
                        ByteDeErro::TimerIn => {
                            assert!(false, "TimerIn");
                        }
                        ByteDeErro::NaoUsado13 => {
                            assert!(false, "NaoUsado13");
                        }
                        ByteDeErro::Framming => {
                            assert!(false, "Framming");
                        }
                        ByteDeErro::OverRun => {
                            assert!(false, "OverRun");
                        }
                        ByteDeErro::BufferDeRecepcaoCheio => {
                            assert!(false, "BufferDeRecepcaoCheio");
                        }
                        ByteDeErro::CheckSum => {
                            assert!(false, "CheckSum");
                        }
                        ByteDeErro::BufferAuxiliarOcupado => {
                            assert!(false, "BufferAuxiliarOcupado");
                        }
                        ByteDeErro::SequenciaDeByteEnviadaMuitoGrande => {
                            assert!(false, "SequenciaDeByteEnviadaMuitoGrande");
                        }
                    },
                    Err(byte_de_erro_desconhecido) => {
                        assert!(false, "Byte de erro desconhecido")
                    }
                };

                assert!(false, "status");
            }

            Err(datalink_error) => match datalink_error {
                DLError::InvalidChannel(channel) => {
                    assert!(false, "InvalidChannel");
                }
                DLError::SerialTransmissionError => {
                    assert!(false, "SerialTransmissionError");
                }
                DLError::DecodingError(segment_error) => {
                    assert!(false, "DecodingError");
                }
                DLError::Timeout(timeout) => {
                    assert!(false, "timeout");
                }
                DLError::SerialReceptionError => {
                    assert!(false, "SerialReceptionError");
                }
                DLError::SlaveHasReturnedStartByteAsNeitherAckNorNack => {
                    assert!(false, "SlaveHasReturnedStartByteEqualsToSTX");
                }
                DLError::SlaveHasReturnedNack(nack_frame) => {
                    assert!(false, "SlaveHasReturnedNack");
                }
            },
        };
        assert_eq!(check, 1, "Everything is checked")
    }

    // /////////////////////////////////////
    // TODO: Test for check each error condition (ie: timeout, checksum wrong, etc)
    // ////////////////////
}
