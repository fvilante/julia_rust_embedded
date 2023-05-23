use crate::board::keypad::KeyCode;
use crate::board::peripherals::Peripherals;
use crate::menu::model::DataModel;
use crate::menu::widget::submenu_programa::menu_programa_controler::MenuProgramaControler;

use crate::geometry::point::Point;
use crate::menu::widget::execucao::MenuExecucaoControler;
use crate::menu::widget::main_menu::MainMenu;
use crate::menu::widget::manual_mode::ManualModeMenuControler;
use crate::menu::widget::splash::Splash;
use crate::menu::widget::submenu_programa::spec::{MenuProgramaHandle, MenuProgramaView};
use crate::microcontroler::timer::now;
use crate::microcontroler::{serial, timer};
use cross_platform::protocol::datalink::datalink::Datalink;
use cross_platform::protocol::transport::channel::Channel;
use cross_platform::protocol::transport::transport_layer::cmpp_value::MechanicalProperties;
use cross_platform::protocol::transport::transport_layer::TransportLayer;

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

/// TODO: Make this a type
type Baudrate = u32;

/// High-level cmpp driver  
///
/// Represents an entire Cmpp Axis System, including unit of measurement convertion
struct CmppAxis {
    mechanical_properties: MechanicalProperties,
    baudrate: Baudrate,
    channel: Channel,
    datalink: Datalink,
}

impl CmppAxis {
    pub fn new(
        baudrate: u32,
        channel: Channel,
        timeout_ms: u16,
        mechanical_properties: MechanicalProperties,
    ) -> Self {
        // set callbacks
        fn now__() -> u16 {
            timer::now() as u16
        }
        fn try_rx() -> Result<Option<u8>, ()> {
            Ok(serial::try_receive())
        }
        fn try_tx(byte: u8) -> Option<()> {
            serial::try_transmit(byte).ok()
        }
        // instantiation
        let datalink = Datalink {
            channel,
            now: now__,
            timeout_ms,
            try_rx,
            try_tx,
            debug_reception: None,
        };
        Self {
            mechanical_properties,
            baudrate,
            channel,
            datalink,
        }
    }

    fn get_transport_layer(&self) -> TransportLayer {
        let transport = TransportLayer::new(&self.datalink, self.mechanical_properties);
        transport
    }
}

/// Entry point of the main application
pub fn run() -> ! {
    // /////////////////////////////////////////////////////////////////////
    // Initialize system
    // ////////////////////////////////////////////////////////////////////

    // ////////////////////////////////////////
    //  Start main data storage
    // ////////////////////////////////////////
    //
    let mut data_model = DataModel::new();
    data_model.load_from_eeprom();

    // ////////////////////////////////////////
    // initialize peripherals
    // ////////////////////////////////////////
    //

    // Serial port
    // TODO: Abstract and Improve readability of this initialization
    const B2400_CODE: u8 = 0;
    let baudrate_code = data_model
        .configuracao_do_equipamento
        .velocidade_de_comunicacao
        .get()
        .get_current();
    let baudrate = if baudrate_code == B2400_CODE {
        2400
    } else {
        9600
    };
    serial::init(baudrate);

    // other peripherals
    let peripherals = Peripherals::new();
    let mut front_panel = peripherals.get_front_panel();
    let mut keyboard = peripherals.get_keyboard();
    let mut screnn_buffer = peripherals.get_screen_buffer();

    // ////////////////////////////////////////
    // Initialize cmpp communication infrastructure
    // ////////////////////////////////////////
    //

    const TIMEOUT_MS: u16 = 1000; // TODO: Maybe this value in future be calculated as a function of the connection baud rate

    let mechanical_properties = MechanicalProperties {
        pulses_per_motor_revolution: 400,
        linear_displacement_per_tooth_belt_mult_by_100: 508,
        number_of_tooths_of_motor_pulley: 16,
    };

    let ch = data_model.configuracao_do_eixo_x.numero_do_canal.get();
    let channel = Channel::from_u16(ch).unwrap_or_default();
    let cmpp_axis = CmppAxis::new(baudrate, channel, TIMEOUT_MS, mechanical_properties);
    let transport = cmpp_axis.get_transport_layer();

    // ///////////////////////////////////////
    //  Main menu mounting
    // ///////////////////////////////////////
    //
    let menu_programa_view = MenuProgramaView::new(&data_model);
    let menu_programa_handle = MenuProgramaHandle::MenuPrograma;
    let menu_programa_controler =
        MenuProgramaControler::new(menu_programa_handle, &menu_programa_view);

    let menu_manual_controler = ManualModeMenuControler::new(&transport);
    let menu_execucao_controler = MenuExecucaoControler::new(&transport);

    let mut main_menu_controler = MainMenu::new(
        menu_manual_controler,
        menu_execucao_controler,
        menu_programa_controler,
        &transport,
        &data_model,
        &mut front_panel,
    );

    // ///////////////////////////////////////
    //  Show initial splash window
    // ///////////////////////////////////////
    //
    let mut splash_window = Splash::new(&data_model, &transport);

    while splash_window.is_running() {
        if let Some(key) = keyboard.get_key() {
            splash_window.send_key(key);
        }

        splash_window.update();
        let _start_point = Point::new(0, 0);
        splash_window.draw(&mut screnn_buffer);
        screnn_buffer.render();
    }

    // /////////////////////////////////////////////////////////////////////
    //  Main loop
    // ////////////////////////////////////////////////////////////////////

    //
    let fps = 30; // 200 milisecs
    let mut next_frame = now() + (1000 / fps);

    loop {
        // Proccess keystrokes
        if let Some(key) = keyboard.get_key() {
            match key {
                KeyCode::KEY_F2 => emit_print_go_signal(&transport),
                _ => main_menu_controler.send_key(key),
            }
        }
        // Update calculations
        main_menu_controler.update();

        // Render next frame
        if now() > next_frame {
            next_frame = now() + (1000 / fps);
            main_menu_controler.draw(&mut screnn_buffer, Point::new(0, 0));
            screnn_buffer.render();
        }
    }
}