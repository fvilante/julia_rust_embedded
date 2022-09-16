



fn transact_packet() {

}



pub fn development_entry_point() -> ! {

    use crate::board::lcd;

    lcd::lcd_initialize();
    lcd::print("Running... -> juca ->");
    lcd::print_u8_in_hex(crate::lib_1::add(1,1));

    loop { }

}