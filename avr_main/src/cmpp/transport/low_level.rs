use crate::board::lcd;

// ==========================================
//  cmpp api - communication services
// ==========================================

// set_bit_mask
// reset_bit_mask
// set_word
// get_word





pub fn development_entry_point() -> ! {
    lcd::lcd_initialize();
    lcd::print("ney");

    loop { }
}