use core::str::FromStr;

use crate::board::lcd;

use alloc::string::String;
use alloc::vec;

use lib_1::adt::reader::Reader;


pub fn development_entry_point() -> ! {

    lcd::lcd_initialize();

    lcd::print("juca livre!");
    
    loop { 
        
    }
}