use core::str::FromStr;

use crate::board::lcd;

use alloc::string::String;
use alloc::vec;

use lib_1::adt::reader::Reader;


pub fn development_entry_point() -> ! {

    lcd::lcd_initialize();

    lcd::print("juca");
    let v = vec![1_u8, 2,3,4,5,6];
    for each in v {
        lcd::print_u8_in_hex(each);
    }
    //let s = String::from("12345");
    lcd::print("juca meneguel");
    
    loop { 

    }
}