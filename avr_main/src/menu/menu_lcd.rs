use crate::board::lcd;


use lib_1::adt::reader::Reader;

pub fn development_entry_point() -> ! {

    lcd::lcd_initialize();

    let mut reader = 
        Reader::new(|text: &str| lcd::print(text))
        .contra_map(|_| "juca free!")
        .unwrap("My humble opinion is...");


    loop { 

    }
}