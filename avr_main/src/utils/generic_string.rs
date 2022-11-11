use core::str::Chars;

use avr_progmem::string::PmString;
use heapless::String;

use crate::menu::flash::{FlashString, FlashStringIterator};


pub enum GenericString<'a> {
    FlashString(FlashString),
    RamString(&'a str)
}


impl<'a> GenericString<'a> {
    pub fn from_flash<const N: usize>(string_flash_pointer: &'a PmString<N>) -> Self {
        let flash_string = FlashString::new(string_flash_pointer);
        Self::FlashString(flash_string)
    }

    pub fn from_Ram(ram_string_reference: &'a str) -> Self {
        Self::RamString(ram_string_reference)
    }

    pub fn iter(&self) -> WrapperIterator<'a> {
        match self {
            Self::FlashString(flash) => {
                WrapperIterator{
                    flash: Some(flash.chars()),
                    ram: None,
                }
                 
            }

            Self::RamString(str) => {
                WrapperIterator{
                    flash: None,
                    ram: Some(str.chars()),
                }
                
            }
        }
    }
}


//Abstract different kind of generic string iterators
pub struct WrapperIterator<'a> {
    pub flash: Option<FlashStringIterator>,
    pub ram: Option<Chars<'a>>,
}

fn convert_char_to_u8(char: char) -> u8 {
    let mut dst = [0; 4];
    char.encode_utf8(&mut dst);
    let res = dst[0];
    res
}

impl<'a> Iterator for WrapperIterator<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(flash_iterator) = &mut self.flash {
            flash_iterator.next()
        } else if let Some(ram_iterator) = &mut self.ram {
            if let Some(byte) = ram_iterator.next() {
                Some(convert_char_to_u8(byte))
            } else {
                None
            }
        } else {
            None
        }
    }
}



