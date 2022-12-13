// low level functions to read and write values direct on microcontroler registers
// for details on what work each register perform see microcontroler datasheet
// for example: ATMEL AVR328P DATASHEET -> https://ww1.microchip.com/downloads/en/DeviceDoc/Atmel-7810-Automotive-Microcontrollers-ATmega328P_Datasheet.pdf

pub fn write_register(reg_address: *mut u8, value: u8) -> () {
    unsafe {
        core::ptr::write_volatile(reg_address, value);
    }
}

pub fn read_register(reg_address: *const u8) -> u8 {
    unsafe { core::ptr::read_volatile(reg_address) }
}
