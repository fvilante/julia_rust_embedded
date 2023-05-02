use crate::menu::{
    flash::FlashString,
    widget::field::optional::{make_options_buffer_from_array, OptionsBuffer},
};

use super::flash_texts::{
    ABERTO, B2400, B9600, CONTINUO, DESLIGADO, FECHADO, LIGADO, PASSO_A_PASSO,
};

/// A storage for variable Options existent on the menu system
/// TODO: Eventually move this to a more appropriate place (ie: ".\widget/field/optional.rs")
pub struct Options;

impl Options {
    pub fn ligado_desligado() -> OptionsBuffer {
        make_options_buffer_from_array([FlashString::new(&DESLIGADO), FlashString::new(&LIGADO)])
    }

    pub fn continuo_passo_a_passo() -> OptionsBuffer {
        make_options_buffer_from_array([
            FlashString::new(&CONTINUO),
            FlashString::new(&PASSO_A_PASSO),
        ])
    }

    pub fn aberto_fechado() -> OptionsBuffer {
        make_options_buffer_from_array([FlashString::new(&ABERTO), FlashString::new(&FECHADO)])
    }

    pub fn baudrate_2400_9600() -> OptionsBuffer {
        make_options_buffer_from_array([FlashString::new(&B2400), FlashString::new(&B9600)])
    }
}
