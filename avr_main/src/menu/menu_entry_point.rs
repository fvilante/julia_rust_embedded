use super::model::DataStorage;
use super::widget::submenu::render::SubmenuProgramaRender;
use crate::board::input_expander::InputExpander;
use crate::board::keyboard::KeyCode;
use crate::board::output_expander::OutputExpander;
use crate::board::shiftout::init_shiftout_pins;
use crate::board::{lcd, shiftin};
use crate::enviroment::front_panel::FrontPanel;
use crate::menu::canvas::Canvas;
use crate::menu::keyboard::Keyboard;
use crate::menu::point::Point;
use crate::menu::widget::execucao::MenuExecucao;
use crate::menu::widget::main_menu::MainMenu;
use crate::menu::widget::manual_mode::ManualModeMenu;
use crate::menu::widget::splash::Splash;
use crate::menu::widget::submenu::spec::{SubmenuProgramaHandle, SubmenuProgramaStorage};
use crate::menu::widget::widget::Widget;
use crate::microcontroler::delay::delay_ms;
use crate::microcontroler::timer::{init_timer, now};
use crate::microcontroler::{serial, timer};
use lib_1::protocol::datalink::datalink::{DLError, Datalink};
use lib_1::protocol::transport::channel::Channel;
use lib_1::protocol::transport::transport_layer::cmpp_value::MechanicalProperties;
use lib_1::protocol::transport::transport_layer::{TLError, TransportLayer};

/// TODO: Implement user interaction with the signal emitted
fn emit_print_go_signal(transport: &TransportLayer) {
    match transport.print_go() {
        Ok(_status) => {
            // TODO: Inform user that a print signal was successful sent to cmpp board
        }
        Err(_error) => {
            // TODO: Inform user what kind of error happened
        }
    }
}

pub fn development_entry_point() -> ! {
    // /////////////////////////////////////////////////////////////////////
    // Initialize system
    // ////////////////////////////////////////////////////////////////////

    // ////////////////////////////////////////////////////
    //
    //   Initialize peripherals:
    //   * Timer interruption (at 1khz)
    //   * Serial port
    //   * Lcd display
    //   * Input & output ports expander
    //   * Keyboard
    //   * Canvas
    //   * Front panel leds
    //
    // TODO: Improve the system enviroment construction
    // ////////////////////////////////////////////////////

    // Initialize timer couting (1khz)
    init_timer();
    // Serial port
    let baud_rate = 9600;
    serial::init(baud_rate);
    // Lcd display
    lcd::lcd_initialize();
    // Initialize on-board IO Expander
    let output_expander = OutputExpander::new();
    let intput_expander = InputExpander::new();
    // Keyboard
    let mut keyboard = Keyboard::new(&output_expander, &intput_expander);
    // Canvas
    let mut canvas = Canvas::new();
    // Leds from the frontal panel
    let mut frontal_panel_leds = FrontPanel::new(&output_expander);

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
        &mut frontal_panel_leds,
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

    // /////////////////////////////////////////////////////////////////////
    //  Main Loop
    // ////////////////////////////////////////////////////////////////////

    //
    let fps = 30; // 200 milisecs
    let mut next_frame = now() + (1000 / fps);

    loop {
        // Proccess keystrokes
        if let Some(key) = keyboard.get_key() {
            match key {
                KeyCode::KEY_F2 => emit_print_go_signal(&transport),
                _ => main_menu.send_key(key),
            }
        }
        // Update calculations
        main_menu.update();

        // Render next frame
        if now() > next_frame {
            next_frame = now() + (1000 / fps);
            main_menu.draw(&mut canvas, Point::new(0, 0));
            canvas.render();
        }
    }
}
