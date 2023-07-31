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

    screen_buffer.clear();
    let text = "Juca Chaves";
    for each in text.chars() {
        screen_buffer.print_char(each);
    }
    screen_buffer.render();
    loop {}
}
