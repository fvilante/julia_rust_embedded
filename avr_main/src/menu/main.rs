use heapless::String;
use heapless::Vec;
use crate::board::output_expander::OutputExpander;
use crate::board::{lcd, output_expander};
use crate::board::keyboard::KeyCode;
use crate::enviroment::front_panel::FrontPanel;
use super::flash::FlashString;
use super::keyboard::Keyboard;
use super::canvas::Canvas;
use super::widget::submenu::Items;
use super::widget::submenu::SubMenu;
use crate::menu::widget::widget::Widget;


// Load Some Flash strings
use core::include;
include!("ttc3100_strings.rs");


pub fn development_entry_point() -> ! {

    lcd::lcd_initialize();

    //temp
    let mut output_expander = OutputExpander::new();
    let _front_panel = FrontPanel::new(&mut output_expander).reset();

    // initialization
    let beep = |on:bool| { OutputExpander::new().BUZZER(on).commit(); };
    let mut keyboard = Keyboard::new(beep);
    let mut canvas = Canvas::new();

    canvas.render();
    
    //widgets
    let mut items: Items = Vec::new();

    items.push(FlashString::new(&S0));
    items.push(FlashString::new(&S1));
    items.push(FlashString::new(&S2));
    items.push(FlashString::new(&S3));
    
    let mut menu = SubMenu::new(items);

    canvas.clear();

    let mut c:u16=0;

    loop { 
        //c += 1;
        //lcd::clear();
        //lcd::print_u16_in_hex(c);
        //loop { }
        // scan: read one key on keyboard
        // update: send key to the Field
        if let Some(key) = keyboard.get_key() {
            menu.send_key(key);
        }

  
        // draw: draw the Field
        canvas.render();

        
        // update & draw
        menu.update();
      
        menu.draw(&mut canvas);
    }
}