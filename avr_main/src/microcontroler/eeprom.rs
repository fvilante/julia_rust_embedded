use lib_1::protocol::datalink::datalink::word16::Word16;
use ruduino::{cores::current as avr_core, interrupt::without_interrupts, Register};

use avr_core::{EEAR, EECR, EEDR, SPMCSR};

/// This code was originally written by `MalteT` and was grab from github commit below:
/// https://github.com/MalteT/counter-avr/blob/ebb7ca36d7a04b11265cb41024798a38ac31ad05/src/main.rs#L251
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

    /// Writes u16 into the current address (in `little-endian` format) and returns the address of the next chunk.
    /// TODO: KNOWN-ISSUES: only address first 255 bytes of eeprom, and cannot address the last word address.
    pub fn write_u16(&mut self, val: u16) -> EepromAddress {
        let address_low = self.0;
        let Some(address_next) = address_low.checked_add(2) else {
            panic!("81") // EEprom address out of 255 range.
            //TODO: Currently this eeprom driver only address 255 bytes, change it to address the 1KB available
            //      in the avr328p
        };
        // SAFETY: Safe because the check is done above
        let address_high = unsafe { address_low.unchecked_add(1) };
        let (byte_low, byte_high) = Word16::from_u16(val).split_bytes();
        EepromAddress(address_low).write(byte_low);
        EepromAddress(address_high).write(byte_high);
        EepromAddress(address_next)
    }

    /// Return the u16 read (in `little-endian` format) and the address pointing to the next chunk
    /// TODO: KNOWN-ISSUES: only address first 255 bytes of eeprom, and cannot address the last word address.
    pub fn read_u16(&self) -> (u16, Self) {
        let address_low = self.0;
        let Some(address_next) = address_low.checked_add(2) else {
            panic!("82") // EEprom address out of 255 range.
            //TODO: Currently this eeprom driver only address 255 bytes, change it to address the 1KB available
            //      in the avr328p
        };
        // SAFETY: Safe because the check is done above
        let address_high = unsafe { address_low.unchecked_add(1) };
        let byte_low = EepromAddress(address_low).read();
        let byte_high = EepromAddress(address_high).read();
        let value = Word16::from_bytes(byte_low, byte_high).to_u16();
        (value, EepromAddress(address_next))
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
