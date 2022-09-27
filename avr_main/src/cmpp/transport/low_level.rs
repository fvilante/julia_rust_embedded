// use lib_1::protocol::common::Frame;

// use crate::{board::lcd, cmpp::datalink::{transact::{DatalinkError, transact}}, common::get_bit_at};

// // utils
// 
// pub struct Word  {
//     pub dataHigh: u8,
//     pub dataLow: u8,
// }
// 
// impl Word {
//     pub fn new(dataLow: u8, dataHigh: u8) -> Self {
//         Self { dataHigh, dataLow }
//     }
// 
//     pub fn from_u16(data: u16) -> Self {
//         let n = data;
//         let dataHigh = (n as f32/256_f32).floor() as u8;
//         let dataLow = n % 256 as u8;
//         Self { dataLow, dataHigh }
//     }
// 
// 
//     pub fn to_u16(&self) -> u16 {
//         let dataHigh = self.dataHigh;
//         let dataLow = self.dataLow;
//         let result:u16 = ((dataHigh as u16) * 256_u16) + dataLow as u16;
//         result
//     }
// 
// }
// 
// 
// // types
// 
// pub enum TransportError {
//     DatalinkError(DatalinkError),
//     InvalidChannel(u8),
// }
// 
// pub enum Direcao {
//     Solicitacao = 0x00,     // 0b00xxxxxx
//     MascaraResetar = 0x40,  // 0b01xxxxxx
//     MascaraSetar = 0x80,    // 0b10xxxxxx
//     Envio = 0xC0,           // 0b11xxxxxx
// }
// 
// struct Canal {
//     numero: u8,
// }
// 
// impl Canal {
//     fn new(numero: u8) -> Self {
//         Self {
//             numero,
//         }
//     }
// 
//     fn get(&self) -> Result<u8,TransportError> {
//         let canal = self.numero;
//         if canal > 63 {
//             Err(TransportError::InvalidChannel(canal))
//         } else {
//             Ok(canal)
//         }
//     }
// }
// 
// struct Packet {
//     direcao: Direcao,
//     canal: Canal,
//     word_address: u8, 
//     word: Word,
// }
// 
// impl Packet {
// 
//     fn from_frame(frame: Frame) -> Self {
//         let Frame(direcao_canal, word_address, dataLow, dataHigh) = frame;
//         let direcao = ((get_bit_at(direcao_canal, 7) << 7_u8) + (get_bit_at(direcao_canal, 6) << 6_u8)) as Direcao;
//         let canal = direcao_canal - direcao;
//         let word = Word{ dataLow, dataHigh };
//         Self {
//             direcao,
//             canal,
//             word_address,
//             word,
//         }
//     }
// 
// }
// 
// 
// 
// 
// 
// 
// 
// 
// // ==========================================
// //  cmpp api - communication services
// // ==========================================
// 
// // get_word
// // reset_bit_mask
// // set_bit_mask
// // set_word
// 
// 
// 
// pub fn get_word(canal: Canal, word_address: u8, connection: impl SerialConnection, timeout: u64) -> Result<Word, TransportError> {
//     let direcao = Direcao::Solicitacao;
//     let frame = Packet { canal, direcao, word_address, word };
//     let result = transact(frame, connection, timeout);
//     
// }   
// 
// 
// 
// 

use crate::board::lcd;

pub fn development_entry_point() -> ! {
    lcd::lcd_initialize();
    lcd::print("ney");

    loop { }
}