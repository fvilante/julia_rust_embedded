use avr_progmem::progmem;

use crate::{microcontroler::timer::now, menu::{flash::FlashString, point::Point}};

use super::widget::Widget;


progmem! {
    static progmem string LINE0 = "Posijet Industria e Comercio Ltda";
    static progmem string LINE1 = "Aguarde carregando parametros...";
}


pub struct Splash {
    pub isRunningYet: bool,
    when_to_hide: u64,
}

impl Splash {
    pub fn new(interval_to_show_ms: u64) -> Self {
        Self { 
            isRunningYet: true,
            when_to_hide: now() + interval_to_show_ms
        }
    }

    fn get_line_helper(line_number: u8) -> (Point, FlashString) {
        let line0 = FlashString::new(&LINE0);
        let line1 = FlashString::new(&LINE1);
        let col0 = ((40 - line0.len()) / 2).try_into().unwrap_or(0);
        let col1 = ((40 - line1.len()) / 2).try_into().unwrap_or(0);
        if line_number == 0 {
            (Point::new(col0,0), line0)
        } else {
            (Point::new(col1,1), line1)
        }
    }
}

impl Widget for Splash {
    fn send_key(&mut self, key: crate::board::keyboard::KeyCode) {
        self.isRunningYet = false; 
    }

    fn update(&mut self) {
        if now() > self.when_to_hide {
            self.isRunningYet = false;
        }
    }

    fn draw(&self, canvas: &mut crate::menu::canvas::Canvas) {
        canvas.clear();
        for line_number in 0..2 {
            let ( point, flash_string ) = Self::get_line_helper(line_number);
            canvas.set_cursor(point);
            canvas.print_flash_str(flash_string);
        }
    }
}