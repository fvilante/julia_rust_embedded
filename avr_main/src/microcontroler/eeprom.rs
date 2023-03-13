use ruduino::{
    cores::current as avr_core, interrupt::without_interrupts, Pin, Register, RegisterBits,
};

use avr_core::{port, DDRB, DDRD, EEAR, EECR, EEDR, PCICR, PCMSK0, PORTB, PORTD, SPMCSR, SREG};

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
