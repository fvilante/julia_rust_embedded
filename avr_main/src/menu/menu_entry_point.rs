use avr_progmem::progmem;
use lib_1::protocol::datalink::datalink::Datalink;
use lib_1::protocol::transport::channel::Channel;
use lib_1::protocol::transport::transport_layer::cmpp_value::MechanicalProperties;
use lib_1::protocol::transport::transport_layer::TransportLayer;
use lib_1::utils::common::usize_to_u8_clamper;

use super::model::MachineModel;
use super::widget::submenu::render::SubMenuRender;

use crate::board::keyboard::KeyCode;
use crate::board::lcd;
use crate::menu::flash::FlashString;
use crate::menu::widget::submenu::spec::{MenuStorage, SubMenuHandle};

use crate::menu::widget::widget_tests::SystemEnviroment;

use crate::microcontroler::delay::delay_ms;
use crate::microcontroler::eeprom::auto_test_eeprom;
use crate::microcontroler::serial;
use crate::microcontroler::timer::{self};

///

pub fn development_entry_point() -> ! {
    //

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

    if let Err(_) = auto_test_eeprom() {
        progmem! {
            static progmem string TEXT = "Erro durante auto-teste da eeprom";
        }
        canvas.clear();
        canvas.print_flash_str(FlashString::new(&TEXT));
        canvas.render();
        delay_ms(2000);
    }

    let machine_model = MachineModel::load_from_eeprom();

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
