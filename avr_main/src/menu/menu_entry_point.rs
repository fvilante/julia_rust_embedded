use super::model::DataStorage;
use super::widget::submenu::render::SubmenuProgramaRender;
use crate::menu::point::Point;
use crate::menu::widget::execucao::MenuExecucao;
use crate::menu::widget::main_menu::MainMenu;
use crate::menu::widget::manual_mode::ManualModeMenu;
use crate::menu::widget::splash::Splash;
use crate::menu::widget::submenu::spec::{SubmenuProgramaHandle, SubmenuProgramaStorage};
use crate::menu::widget::widget::Widget;
use crate::menu::widget::widget_tests::SystemEnviroment;
use crate::microcontroler::timer::now;
use crate::microcontroler::{serial, timer};
use lib_1::protocol::datalink::datalink::Datalink;
use lib_1::protocol::transport::channel::Channel;
use lib_1::protocol::transport::transport_layer::cmpp_value::MechanicalProperties;
use lib_1::protocol::transport::transport_layer::TransportLayer;

pub fn development_entry_point() -> ! {
    // ////////////////////////////////////////
    // Initialize System
    // ////////////////////////////////////////
    //
    let SystemEnviroment {
        mut canvas,
        mut keyboard,
        ..
    } = SystemEnviroment::new(BAUD_RATE);

    // ////////////////////////////////////////
    // Start comunication infrastructure
    // ////////////////////////////////////////
    //
    let channel = Channel::from_u8(0).unwrap();
    fn now__() -> u16 {
        timer::now() as u16
    }
    const TIMEOUT_MS: u16 = 1000; // TODO: Maybe in future be calculated as a function of the connection baud rate

    const BAUD_RATE: u32 = 9600; // FIX: 2400 is not working, the problem seems to be in the register's port setup configuration

    fn try_rx() -> Result<Option<u8>, ()> {
        Ok(serial::try_receive())
    }

    fn try_tx(byte: u8) -> Option<()> {
        serial::try_transmit(byte).ok()
    }

    let datalink = &Datalink {
        channel,
        now: now__,
        timeout_ms: TIMEOUT_MS,
        try_rx,
        try_tx,
        debug_reception: None,
    };

    let mechanical_properties = MechanicalProperties {
        pulses_per_motor_revolution: 400,
        linear_displacement_per_tooth_belt_mult_by_100: 508,
        number_of_tooths_of_motor_pulley: 16,
    };

    let transport = TransportLayer::new(datalink, mechanical_properties);

    // ////////////////////////////////////////
    //  Data Storage
    // ////////////////////////////////////////
    //
    let mut data_storage = DataStorage::new();
    data_storage.load_from_eeprom();

    // ///////////////////////////////////////
    //  Main Menu Mounting
    // ///////////////////////////////////////
    //
    let submenu_programa_storage: SubmenuProgramaStorage =
        SubmenuProgramaStorage::new(&data_storage);
    let submenu_programa_handle = SubmenuProgramaHandle::MenuPrograma;
    let menu_programa =
        SubmenuProgramaRender::new(submenu_programa_handle, &submenu_programa_storage);

    let menu_manual = ManualModeMenu::new(&transport);
    let menu_execucao = MenuExecucao::new(&transport);
    let mut main_menu = MainMenu::new(
        menu_manual,
        menu_execucao,
        menu_programa,
        &transport,
        &data_storage,
    );

    // ///////////////////////////////////////
    //  Show Initial Splash Window
    // ///////////////////////////////////////
    //
    let mut splash_window = Splash::new(&data_storage, &transport);

    while splash_window.is_running() {
        if let Some(key) = keyboard.get_key() {
            splash_window.send_key(key);
        }

        splash_window.update();
        let start_point = Point::new(0, 0);
        splash_window.draw(&mut canvas, start_point);
        canvas.render();
    }

    // ///////////////////////////////////////
    //  Main Loop
    // ///////////////////////////////////////
    //
    let fps = 5;
    let mut next_frame = now() + (1000 / fps);

    loop {
        if let Some(key) = keyboard.get_key() {
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
