///////////////////////////////////////////////////////////

use crate::utils::common::{get_bit_at_as_bool, word_to_byte};

use super::{
    decoder::{Decoder, SegmentError},
    encoder::Encoder,
    frame::{Frame, Payload, SlaveFrame},
    prelude::{SlaveStartByte, StartByte},
};

/// DatalinkError
/// TODO: Resolve conflict naming with other struct named 'DataLinkError', this below
/// is intended to be more generic
pub enum DLError {
    /// Channel number must be between range 0..64 (inclusive, exclusive)
    InvalidChannel(u8),
    /// Low level (byte-level), serial TX function failed
    SerialTransmissionError,
    /// The incomming byte stream from slave does not obey the right syntax of the protocol or the checksum is wrong
    DecodingError(SegmentError),
    /// Timeout error (cannot wait response forever!)
    Timeout(u16),
    /// Low level (byte-level), serial RX function failed
    SerialReceptionError,
    /// If slave returns STX (which according to protocol v1 spec is not allowed, just
    /// Masters may use STX as start byte)
    SlaveHasReturnedStartByteEqualsToSTX,
}

///////////////////////////////////////////////////////////

/// For more details about how Channel concept works see the cmpp v1 protocol specification
#[derive(Copy, Clone)]
pub struct Channel {
    number: u8,
}

impl Channel {
    pub fn to_u8(&self) -> u8 {
        self.number
    }
}

impl TryFrom<u8> for Channel {
    type Error = DLError;

    fn try_from(channel: u8) -> Result<Self, Self::Error> {
        const MAX_CHANNELS: u8 = 64;
        if channel < MAX_CHANNELS {
            Ok(Self { number: channel })
        } else {
            Err(DLError::InvalidChannel(channel))
        }
    }
}

impl Into<u8> for Channel {
    fn into(self) -> u8 {
        self.number
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

/// Direction to be used in the MasterFrame which is sent to cmpp. For more details see the cmpp v1 protocol specification.
#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Direction {
    Get = (0 << BIT_7) + (0 << BIT_6),
    ResetBitmask = (0 << BIT_7) + (1 << BIT_6),
    SetBitmask = (1 << BIT_7) + (0 << BIT_6),
    Set = (1 << BIT_7) + (1 << BIT_6),
}

impl Direction {
    pub fn to_u8(&self) -> u8 {
        /// TODO: Reduce code surface when possible, eliminate redundance.
        match self {
            Direction::Get => 0,
            Direction::ResetBitmask => 64,
            Direction::SetBitmask => 128,
            Direction::Set => 192,
        }
    }
}

///////////////////////////////////////////////////////////

/// Represents a 16 bits word
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

/// When master sends an Direction::{ Set, SetBitMask, ResetBitMask} MasterFrame this is what is expected the slave to respond if
/// no error happens and slave responds with ACK.
pub struct PacoteDeRetornoDeEnvio {
    status: Status,
}

impl PacoteDeRetornoDeEnvio {
    pub fn from_slave_frame(slave_frame: SlaveFrame) -> Self {
        let [_, _, byte_low, _] = slave_frame.payload;
        Self {
            status: Status::new_from_raw(byte_low),
        }
    }
}

/// When master sends an Direction::get MasterFrame this is what is expected the slave to respond if
/// no error happens and slave responds with ACK.
pub struct PacodeDeRetornoDeSolicitacao {
    data: Word16,
}

impl PacodeDeRetornoDeSolicitacao {
    pub fn from_slave_frame(slave_frame: SlaveFrame) -> Self {
        let [_, _, byte_low, byte_high] = slave_frame.payload;
        let word = Word16::from_bytes(byte_low, byte_high);
        Self { data: word }
    }
}

/// When master sends any Direction MasterFrame this is what is expected the slave to respond if it did
/// not accepeted or understood the master request
pub struct PacoteDeRetornoComErro {
    byte_de_erro: Result<ByteDeErro, UnknownByteDeErro>,
    status: Status,
}

impl PacoteDeRetornoComErro {
    pub fn from_slave_frame(slave_frame: SlaveFrame) -> Self {
        let [_, _, byte_low, byte_high] = slave_frame.payload;
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
    channel: Channel,
    ///timeout in miliseconds
    /// TODO: Study to change this to u8 to reduce memory footprint
    timeout_ms: u16,
    /// None if some error happened
    try_tx: fn(u8) -> Option<()>,
    /// Ok_None if nothing to receive, Err if some error happened, Ok_Some if a byte has been received
    try_rx: fn() -> Result<Option<u8>, ()>,
    /// Returns miliseconds elapsed since `Epoch` (when machine was turned on)
    /// TODO: Make this function more memory eficient, because I think it's not necessary
    /// two u64 to calculate elapsed time like: `let time_elapsed = now() - start_time;`
    /// eventually something with the size statically defined by user should be better
    now: fn() -> u64,
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
        let direction_and_channel: u8 = channel.to_u8() + direction.to_u8();
        let payload: Payload = [direction_and_channel, word_address, byte_low, byte_high];
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
    fn receive(&self) -> Result<SlaveFrame, DLError> {
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
                            // check if it is an slave frame and return it
                            if let Ok(slave_frame) = frame.try_into() {
                                return Ok(slave_frame);
                            } else {
                                // It will not be an slave frame if it has a STX start byte
                                return Err(DLError::SlaveHasReturnedStartByteEqualsToSTX);
                            }
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

    fn transact(
        &self,
        direction: Direction,
        word_address: u8,
        word_value: u16,
    ) -> Result<SlaveFrame, DLError> {
        // send
        let encoded = Self::encode_data(self.channel, direction, word_address, word_value);
        if let Err(error) = self.transmit(encoded) {
            return Err(error);
        }
        //receive
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
}
