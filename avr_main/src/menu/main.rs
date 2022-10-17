use core::str::FromStr;

use alloc::string::ToString;
use heapless::String;
use heapless::Vec;
use crate::board::output_expander::OutputExpander;
use crate::board::{lcd, output_expander};
use crate::board::keyboard::KeyCode;
use crate::enviroment::front_panel::FrontPanel;
use super::flash::FlashString;
use super::keyboard::Keyboard;
use super::canvas::Canvas;
use super::widget::caption::Caption;
use super::widget::field::Field;
use super::widget::submenu::Items;
use super::widget::submenu::SubMenu;
use crate::menu::widget::widget::Widget;


use avr_progmem::progmem;

progmem! {

                              //123456789012345678901234567890123456789 -> 39 characters   
    static progmem string T0 = "Posicao inicial             ${nnnnn} mm";
    static progmem string T1 = "Posicao final               ${nnnnn} mm";
    static progmem string T2 = "Velocidade de avanco      ${nnnnn} mm/s";
    static progmem string T3 = "Velocidade de retorno     ${nnnnn} mm/s";
    static progmem string T4 = "Aceleracao de avanco     ${nnnnn} mm/s2";
    static progmem string T5 = "Aceleracao de reto       ${nnnnn} mm/s2";
    static progmem string T6 = "Numero de mensagens no avanco     ${nn}";
    static progmem string T7 = "Numero de mensagens no retorno    ${nn}";
    static progmem string T8 = "Modo continuo ou passo-a-passo [${alt1}]";
    static progmem string T9 = "Logica do start externo        [${alt2}]";


    static progmem string E0 = "Erro de carga de parametro";
    static progmem string S0 = "Posicao Inicial";
    static progmem string S1 = "Posicao Final";
    static progmem string S2 = "Velocidade de Avanco";
    static progmem string S3 = "Velocidade de Retorno";
    static progmem string S4 = "Aceleracao de Avanco";
    static progmem string S5 = "Aceleracao de Retorno";
    static progmem string S6 = "Start Automatico no Avanco";
    static progmem string S7 = "Start Automatico no Retorno";

    //NOTE: it is possible to load any type in progmem not only strings
    static progmem A0: [u8; 6] = [0,1,2,3,4,5];
    static progmem string ERRO_01 = "Erro de construcao de string";
}

enum MenuItemParsed {
    PureCaption(String<40>), // [Caption]
    CaptionWithOneField(String<40>, String<10>, String<10>), // [1st Caption, Field Type, Last Caption]
}

fn parse_menu_item_constructor_string(declaration: FlashString) -> MenuItemParsed {
    // example of declaration content = "Posicao inicial     ${nnnnn} mm/s"
    let s: String<40>  = declaration.to_string().unwrap_or(String::from_str("Error: Small container").unwrap());
    let begin_token: &[_] = &['$', '{'];
    let end_token: &[_] = &['}'];
    match s.find(begin_token) {
        Some(begin_index) =>  {
            //1st caption ends in begin_index
            let x = s.split_at(begin_index+begin_token.len());
            let first_caption_ = x.0;
            let first_caption = &first_caption_[0..first_caption_.len()-begin_token.len()];
            let remain = x.1;
            match remain.find(end_token) {
                Some(end_index) => {
                    let y = remain.split_at(end_index);
                    let field_type = y.0;
                    let last_caption_ = y.1;
                    let last_caption = &last_caption_[end_token.len()..last_caption_.len()];
                    MenuItemParsed::CaptionWithOneField(
                        String::from_str(first_caption).unwrap(), 
                        String::from_str(field_type).unwrap(), 
                        String::from_str(last_caption).unwrap(),
                    )
                }
                None => {
                    //false open, everything is caption
                    let caption = s.as_str();
                    MenuItemParsed::PureCaption(String::from_str(caption).unwrap())
                }
            }
        }

        None => {
            //caption entire string
            let caption = s.as_str();
            MenuItemParsed::PureCaption(String::from_str(caption).unwrap())
        }
    }
}



pub fn development_entry_point() -> ! {

    lcd::lcd_initialize();

    //temp
    let mut output_expander = OutputExpander::new();
    let _front_panel = FrontPanel::new(&mut output_expander).reset();

    // initialization
    let beep = |on:bool| { OutputExpander::new().BUZZER(on).commit(); };
    let mut keyboard = Keyboard::new(beep);
  
    let parsed = parse_menu_item_constructor_string(FlashString::new(&T0));
    match parsed {
        MenuItemParsed::PureCaption(caption) => {
            lcd::print(caption.as_str());
        },
        MenuItemParsed::CaptionWithOneField(first_caption, field_type, last_caption) => {
            lcd::print("1-{");
            lcd::print(first_caption.as_str());
            lcd::print("} 2-{");
            lcd::print(&field_type);
            lcd::print("} 3-{");
            lcd::print(last_caption.as_str());
            lcd::print("}");
        },
    }
  
    loop { }

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