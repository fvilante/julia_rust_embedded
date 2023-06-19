use crate::board::front_panel::FrontPanel;
use crate::board::keyboard::Keyboard;
use crate::board::keypad::KeyCode;
use crate::board::peripherals::Peripherals;
use crate::board::peripherals::PeripheralsAvrHardware;
use crate::geometry::point::Point;
use crate::menu::model::DataModel;
use crate::menu::screen_buffer::ScreenBuffer;
use crate::menu::widget::execucao::MenuExecucaoControler;
use crate::menu::widget::main_menu::MainMenu;
use crate::menu::widget::manual_mode::ManualModeMenuControler;
use crate::menu::widget::splash::Splash;
use crate::menu::widget::submenu_programa::menu_programa_controler::MenuProgramaControler;
use crate::menu::widget::submenu_programa::spec::{MenuProgramaAreanaSelector, MenuProgramaArena};
use crate::menu::widget::widget::Widget;
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
    let peripherals = PeripheralsAvrHardware::new();
    let mut front_panel = peripherals.get_front_panel();
    let mut keyboard = peripherals.get_keyboard();
    let mut screen_buffer = peripherals.get_screen_buffer();

    // ////////////////////////////////////////
    // Initialize cmpp communication infrastructure
    // ////////////////////////////////////////
    //

    const TIMEOUT_MS: u16 = 1000; // TODO: Maybe this value in future be calculated as a function of the connection baud rate

    let mechanical_properties_x = MechanicalProperties {
        pulses_per_motor_revolution: 400,
        linear_displacement_per_tooth_belt_mult_by_100: 508,
        number_of_tooths_of_motor_pulley: 16,
    };

    let mechanical_properties_y = MechanicalProperties {
        pulses_per_motor_revolution: 400,
        linear_displacement_per_tooth_belt_mult_by_100: 508,
        number_of_tooths_of_motor_pulley: 16,
    };

    let ch_x = data_model.configuracao_do_eixo_x.numero_do_canal.get();
    let ch_y = data_model.configuracao_do_eixo_y.numero_do_canal.get();
    let channel_x = Channel::from_u16(ch_x).unwrap_or_default();
    let channel_y = Channel::from_u16(ch_y).unwrap_or_default();
    let cmpp_axis_x = CmppAxis::new(baudrate, channel_x, TIMEOUT_MS, mechanical_properties_x);
    let cmpp_axis_y = CmppAxis::new(baudrate, channel_y, TIMEOUT_MS, mechanical_properties_y);
    let transport_x = cmpp_axis_x.get_transport_layer();
    let transport_y = cmpp_axis_y.get_transport_layer();

    // ///////////////////////////////////////
    //  Main menu mounting
    // ///////////////////////////////////////
    //

    fn make_menu_controler<'a>(
        menu_programa_arena: &'a MenuProgramaArena,
        data_model: &'a DataModel,
        transport_x: &'a TransportLayer,
        transport_y: &'a TransportLayer,
        front_panel: &'a mut impl FrontPanel,
    ) -> impl Widget + 'a {
        // menu root
        let initial_menu_selector = MenuProgramaAreanaSelector::MenuPrograma;
        // child menus
        let menu_programa_controler =
            MenuProgramaControler::new(initial_menu_selector, &menu_programa_arena);
        let menu_manual_controler = ManualModeMenuControler::new(&transport_x);
        let menu_execucao_controler = MenuExecucaoControler::new(&transport_x);
        // parent menu
        MainMenu::new(
            menu_manual_controler,
            menu_execucao_controler,
            menu_programa_controler,
            &transport_x,
            &transport_y,
            &data_model,
            front_panel,
        )
    }

    let menu_programa_arena = MenuProgramaArena::new(&data_model);
    let menu_controler = make_menu_controler(
        &menu_programa_arena,
        &data_model,
        &transport_x,
        &transport_y,
        &mut front_panel,
    );

    // ///////////////////////////////////////
    //  Show initial splash window
    // ///////////////////////////////////////
    //
    let mut splash_window = Splash::new(&data_model, &transport_x, &transport_y);

    while splash_window.is_running() {
        if let Some(key) = keyboard.get_key() {
            splash_window.send_key(key);
        }

        splash_window.update();
        let _start_point = Point::new(0, 0);
        splash_window.draw(&mut screen_buffer);
        screen_buffer.render();
    }

    // /////////////////////////////////////////////////////////////////////
    //  Main loop
    // ////////////////////////////////////////////////////////////////////

    fn start_main_loop(
        mut screen_buffer: ScreenBuffer,
        mut keyboard: impl Keyboard,
        mut menu_controler: impl Widget,
        transport: &TransportLayer,
    ) -> ! {
        let fps = 30; // frames_per_second for lcd display redraw -> 30_fps = 200_milisecs
        let mut next_frame = now() + (1000 / fps);
        loop {
            // Proccess keystrokes
            if let Some(key) = keyboard.get_key() {
                match key {
                    KeyCode::KEY_F2 => emit_print_go_signal(&transport),
                    _ => menu_controler.send_key(key),
                }
            }
            // Update calculations
            menu_controler.update();

            // Render next frame
            if now() > next_frame {
                next_frame = now() + (1000 / fps);
                menu_controler.draw(&mut screen_buffer, Point::new(0, 0));
                screen_buffer.render();
            }
        }
    }

    start_main_loop(screen_buffer, keyboard, menu_controler, &transport_x)
}
