use avr_progmem::progmem;

use lib_1::protocol::datalink::datalink::Datalink;
use lib_1::protocol::transport::channel::Channel;
use lib_1::protocol::transport::transport_layer::cmpp_value::MechanicalProperties;

use lib_1::protocol::transport::transport_layer::new_proposal::{
    Acceleration, Displacement, FromCmpp, ToCmpp, Velocity,
};
use lib_1::protocol::transport::transport_layer::TransportLayer;
use lib_1::utils::common::usize_to_u8_clamper;

use super::canvas::Canvas;
use super::model::MachineModel;
use super::widget::submenu::render::SubMenuRender;

use crate::board::keyboard::KeyCode;
use crate::board::lcd;
use crate::menu::flash::FlashString;
use crate::menu::point::Point;
use crate::menu::widget::execucao::MenuExecucao;
use crate::menu::widget::main_menu::MainMenu;
use crate::menu::widget::manual_mode::ManualModeMenu;
use crate::menu::widget::splash::Splash;
use crate::menu::widget::submenu::spec::{MenuStorage, SubMenuHandle};

use crate::menu::widget::widget::Widget;
use crate::menu::widget::widget_tests::SystemEnviroment;

use crate::microcontroler::delay::delay_ms;

use crate::microcontroler::serial;
use crate::microcontroler::timer::{self};
use crate::utils::generic_string::GenericString;

fn example_00(transport: &TransportLayer) {
    lcd::print("ini");
    transport.force_reference(None, None);
    transport.wait_to_stop();
    transport.posicao_inicial().set(Displacement(500));
    transport.posicao_final().set(Displacement(2500));
    transport.velocidade_de_avanco().set(Velocity(5000));
    transport.aceleracao_de_avanco().set(Acceleration(5000));

    for _ in 0..5 {
        transport.start();
        transport.wait_to_stop();
    }

    transport.start();
    transport.start();

    lcd::print("Fim");
}

///

pub fn development_entry_point() -> ! {
    ///////////////////

    lcd::lcd_initialize();
    lcd::print("Oi6");
    loop {}

    ///////////////////

    let channel = Channel::from_u8(0).unwrap();
    fn now() -> u16 {
        timer::now() as u16
    }
    const TIMEOUT_MS: u16 = 1000; // TODO: Maybe in future be calculated as a function of the connection baud rate

    const BAUD_RATE: u32 = 9600; // FIX: 2400 is not working, the problem seems to be in the register's port setup configuration
    let _serial = serial::init(BAUD_RATE);

    fn try_rx() -> Result<Option<u8>, ()> {
        Ok(serial::try_receive())
    }

    fn try_tx(byte: u8) -> Option<()> {
        serial::try_transmit(byte).ok()
    }

    let datalink = &Datalink {
        channel,
        now,
        timeout_ms: TIMEOUT_MS,
        try_rx,
        try_tx,
        debug_reception: None,
    };

    loop {
        for waddr in 0..0xFF {
            let Ok(Ok(pacote_retorno)) = datalink.get_word16(waddr.into()) else {
                loop {}
            };
            let word = pacote_retorno.data;
            lcd::clear();
            lcd::print_u16_in_hex(word.to_u16());
        }
    }

    let mechanical_properties = MechanicalProperties {
        pulses_per_motor_revolution: 400,
        linear_displacement_per_tooth_belt_mult_by_100: 508,
        number_of_tooths_of_motor_pulley: 16,
    };

    let transport = TransportLayer::new(datalink, mechanical_properties);

    ///////////////////

    let SystemEnviroment {
        mut canvas,
        mut keyboard,
        ..
    } = SystemEnviroment::new();

    ///////////////////

    let mut machine_model = MachineModel::new();
    machine_model.load_from_eeprom();

    //////

    let menu_storage: MenuStorage = MenuStorage::new(&machine_model);
    let menu_root = SubMenuHandle::MenuPrograma;
    let mut menu_programa = SubMenuRender::new(menu_root, &menu_storage);

    let menu_manual = ManualModeMenu::new(&transport);
    let menu_execucao = MenuExecucao::new(&transport);
    let mut main_menu = MainMenu::new(
        menu_manual,
        menu_execucao,
        menu_programa,
        &transport,
        &machine_model,
    );

    ///////

    let mut splash_window = Splash::new(&machine_model, &transport);

    while splash_window.is_running() {
        if let Some(key) = keyboard.get_key() {
            splash_window.send_key(key);
        }

        splash_window.update();
        let start_point = Point::new(0, 0);
        splash_window.draw(&mut canvas, start_point);
        canvas.render();
    }

    ///////

    let fps = 30; // frames_per_second
    let mut next_frame: u16 = now() + (1000 / fps);

    loop {
        if let Some(key) = keyboard.get_key() {
            if key == KeyCode::KEY_F4 {
                lcd::clear();
                lcd::print("Enviando");
                for (index, _response) in machine_model.send_all(&transport).enumerate() {
                    let index = usize_to_u8_clamper(index);
                    lcd::clear();
                    lcd::print_u8_in_hex(index);
                }
            }
            if key == KeyCode::KEY_F3 {
                progmem! {
                    static progmem string TEXT = "Gravando na EEPROM...";
                }
                canvas.clear();
                canvas.print_flash_str(FlashString::new(&TEXT));
                canvas.render();
                machine_model.save_to_eeprom();
                delay_ms(2000);
            }
            if key == KeyCode::KEY_F2 {
                progmem! {
                    static progmem string TEXT = "Enviando dados para cmpp";
                }
                canvas.clear();
                canvas.print_flash_str(FlashString::new(&TEXT));
                canvas.render();
                for _ in machine_model.send_all(&transport) {}
                delay_ms(500);
            }
            if key == KeyCode::KEY_F1 {
                progmem! {
                    static progmem string TEXT = "Enviando start para cmpp";
                }
                canvas.clear();
                canvas.print_flash_str(FlashString::new(&TEXT));
                canvas.render();
                transport.start();
                //transport.start();
                delay_ms(500);
            }
            main_menu.send_key(key);
        }

        if now() > next_frame {
            next_frame = now() + (1000 / fps);
            main_menu.update();
            main_menu.draw(&mut canvas, Point::new(0, 0));
            canvas.render();
        }
    }
}
