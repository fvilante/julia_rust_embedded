



fn transact_packet() {

}



pub fn development_entry_point() -> ! {

    use crate::board::lcd;

    lcd::lcd_initialize();
    lcd::print("Running -> juca");

    loop { }

}