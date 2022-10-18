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
use super::widget::main_menu::MainMenu;
use super::widget::menu_item::MenuItemParsed;
use super::widget::menu_item::parse_menu_item_constructor_string;
use super::widget::splash::Splash;
use super::widget::submenu::Items;
use super::widget::submenu::SubMenu;
use crate::menu::widget::widget::Widget;


use avr_progmem::progmem;

progmem! {

    //                          123456789012345678901234567890123456789 -> 39 characters   
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




pub fn development_entry_point() -> ! {

    // initialization
    lcd::lcd_initialize();
    let mut output_expander = OutputExpander::new();
    let _front_panel = FrontPanel::new(&mut output_expander).reset();
    let beep = |on:bool| { OutputExpander::new().BUZZER(on).commit(); };
    let mut keyboard = Keyboard::new(beep);
    let mut canvas = Canvas::new();
    canvas.render();  
    
    //splash
    let mut splash = Splash::new(4500);
    loop {
        if let Some(key) = keyboard.get_key() {
            splash.send_key(key);
        }
        splash.update();
        splash.draw(&mut canvas);
        if splash.isRunningYet == false {
            break;
        }
        canvas.render();
    }

    //main menu
    let mut main_menu = MainMenu::new();
    loop {
        if let Some(key) = keyboard.get_key() {
            main_menu.send_key(key);
        }
        main_menu.update();
        main_menu.draw(&mut canvas);
        canvas.render();
    }

    //submenu
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