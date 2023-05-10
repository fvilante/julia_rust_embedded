use lib_1::{protocol::datalink::datalink::word16::Word16, utils::cursor::Cursor};
use ruduino::{cores::current as avr_core, interrupt::without_interrupts, Register};

use avr_core::{EEAR, EECR, EEDR, SPMCSR};

/// This code was originally written by `MalteT` and was grab from github commit below:
/// https://github.com/MalteT/counter-avr/blob/ebb7ca36d7a04b11265cb41024798a38ac31ad05/src/main.rs#L251
#[derive(Copy, Clone)]
pub struct EepromAddress(pub u8);

impl EepromAddress {
    pub fn read(&self) -> u8 {
        without_interrupts(|| {
            // Do not acces eeprom, if it is written to or the flash is currently programmed!
            while EECR::is_set(EECR::EEPE) || SPMCSR::is_set(SPMCSR::SPMEN) {}
            // Write the address
            EEAR::write(self.0);
            // Start reading from eeprom
            // XXX: This could be `set` but `set` isn't using volatile_* atm.
            EECR::write(EECR::EERE);
            // Return the read value
            let ret = EEDR::read();
            ret
        })
    }
    pub fn write(&mut self, val: u8) {
        without_interrupts(|| {
            // Do not acces eeprom, if it is written to or the flash is currently programmed!
            while EECR::is_set(EECR::EEPE) || SPMCSR::is_set(SPMCSR::SPMEN) {}
            // Write the address
            EEAR::write(self.0);
            // Write the value
            EEDR::write(val);
            // Start writing to the eeprom
            // XXX: This could be `set` but `set` isn't using volatile_* atm.
            EECR::write(EECR::EEMPE);
            // XXX: This could be `set` but `set` isn't using volatile_* atm.
            EECR::write(EECR::EEPE);
        })
    }

    // Flavio's API

    /// EEprom address out of 255 range.
    /// TODO: Currently this eeprom driver only address 255 bytes, change it to address the 1KB available
    ///      in the avr328p
    pub fn out_of_range_error() -> ! {
        //TODO: Currently panic is not showing message in lcd because this is a costly operation in terms of flash consumption
        //because that it's necessary to elaborate a way to show this error message on lcd display in a cheap way.
        panic!("E81")
    }

    /// TODO: KNOWN-ISSUES: only address first 255 bytes of eeprom, and cannot address the last word address.
    pub fn write_u8(&mut self, val: u8) -> EepromAddress {
        let address = self.0;
        let Some(next_address) = address.checked_add(1) else {
            Self::out_of_range_error()
        };
        self.write(val);
        EepromAddress(next_address)
    }

    /// Writes u16 into the current address (in `little-endian` format) and returns the address of the next chunk.
    /// TODO: KNOWN-ISSUES: only address first 255 bytes of eeprom, and cannot address the last word address.
    pub fn write_u16(&mut self, val: u16) -> EepromAddress {
        let (byte_low, byte_high) = Word16::from_u16(val).split_bytes();
        let mut next = self.write_u8(byte_low);
        let next = next.write_u8(byte_high);
        next
    }

    /// TODO: Cursor is being write in eeprom using 3 bytes, but if Cursor::start is always zero we can use just 2 bytes
    pub fn write_cursor(&mut self, cursor: Cursor) -> EepromAddress {
        let byte_0 = cursor.get_current();
        let byte_1 = cursor.get_range().start; // TODO: Check if this byte is always 0, and if it is remove it from eeprom
        let byte_2 = cursor.get_range().end;
        let mut next = self.write_u8(byte_0);
        let mut next = next.write_u8(byte_1);
        let next = next.write_u8(byte_2);
        next
    }

    /// TODO: KNOWN-ISSUES: only address first 255 bytes of eeprom, and cannot address the last word address.
    pub fn read_u8(&self) -> (u8, Self) {
        let address = self.0;
        let Some(next_address) = address.checked_add(1) else {
            Self::out_of_range_error()
        };
        let value = self.read();
        (value, EepromAddress(next_address))
    }

    /// Return the u16 read (in `little-endian` format) and the address pointing to the next chunk
    /// TODO: KNOWN-ISSUES: only address first 255 bytes of eeprom, and cannot address the last word address.
    pub fn read_u16(&self) -> (u16, Self) {
        let (byte_low, next) = self.read_u8();
        let (byte_high, next) = next.read_u8();
        let value = Word16::from_bytes(byte_low, byte_high).to_u16();
        (value, next)
    }

    pub fn read_cursor(&self) -> (Cursor, Self) {
        let (byte_0, next) = self.read_u8();
        let (byte_1, next) = next.read_u8();
        let (byte_2, next) = next.read_u8();
        let (initial_value, start, end) = (byte_0, byte_1, byte_2);
        let cursor = Cursor::new(start, end, initial_value);
        (cursor, next)
    }
}

pub struct EepromTestError {
    address: u16,
    expected_value: u8,
    actual_value: u8,
}

/// CAUTION: This test will destroy the current content of the EEPROM
pub fn auto_test_eeprom() -> Result<(), EepromTestError> {
    use core::ops::Range;

    fn write_data_into_eeprom(range: Range<u8>) {
        for address in range {
            EepromAddress(address).write(address);
        }
    }

    fn check_data_into_eeprom(range: Range<u8>) -> Result<(), EepromTestError> {
        for address in range {
            let data_read = EepromAddress(address).read();
            if data_read == address {
                continue;
            } else {
                return Err(EepromTestError {
                    address: address as u16,
                    expected_value: address,
                    actual_value: data_read,
                });
            }
        }

        Ok(())
    }

    // Perform test
    let address_range_to_check = 0..0xff;
    write_data_into_eeprom(address_range_to_check.clone());
    check_data_into_eeprom(address_range_to_check.clone())
}
