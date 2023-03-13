use lib_1::protocol::datalink::datalink::Datalink;
use lib_1::protocol::transport::channel::Channel;
use lib_1::protocol::transport::transport_layer::cmpp_value::MechanicalProperties;
use lib_1::protocol::transport::transport_layer::TransportLayer;
use lib_1::utils::common::usize_to_u8_clamper;

use super::model::MachineModel;
use super::widget::submenu::render::SubMenuRender;

use crate::board::keyboard::KeyCode;
use crate::board::lcd;
use crate::menu::widget::submenu::spec::{MenuStorage, SubMenuHandle};

use crate::menu::widget::widget_tests::SystemEnviroment;

use crate::microcontroler::delay::delay_ms;
use crate::microcontroler::serial;
use crate::microcontroler::timer::{self, now};

///

pub fn development_entry_point() -> ! {
    //

    let channel = Channel::from_u8(0).unwrap();
    fn now() -> u16 {
        timer::now() as u16
    }
    const timeout_ms: u16 = 1000; // TODO: Maybe in future be calculated as a function of the connection baud rate

    const baud_rate: u32 = 9600; // FIX: 2400 is not working, the problem seems to be in the register's port setup configuration
    let _serial = serial::init(baud_rate);

    fn try_rx() -> Result<Option<u8>, ()> {
        Ok(serial::try_receive())
    }

    fn try_tx(byte: u8) -> Option<()> {
        serial::try_transmit(byte).ok()
    }

    let datalink = &Datalink {
        channel,
        now,
        timeout_ms,
        try_rx,
        try_tx,
    };

    let mechanical_properties = MechanicalProperties {
        pulses_per_motor_revolution: 400,
        linear_displacement_per_tooth_belt: 828,
    };

    let transport = TransportLayer::new(datalink, mechanical_properties);

    //

    let SystemEnviroment {
        mut canvas,
        mut keyboard,
        ..
    } = SystemEnviroment::new();

    let machine_model = MachineModel::new();

    let menu_storage: MenuStorage = MenuStorage::new(&machine_model);

    let menu_root = SubMenuHandle::MenuPrograma;

    let mut submenu = SubMenuRender::new(menu_root, &menu_storage);

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
            submenu.send_key(key);
        }

        if now() > next_frame {
            next_frame = now() + (1000 / fps);
            submenu.update();
            submenu.draw(&mut canvas);
            canvas.render();
        }
    }
}
