// menu parameters database

use super::accessor::Accessor;

pub struct DataBase {
    parameter01: u16,
    parameter02: u16,
    parameter03: bool,
    parameter04: u8,
}

impl DataBase {

    pub fn new() -> Self {
        Self {
            parameter01: 1,
            parameter02: 2,
            parameter03: false,
            parameter04: 4,
        }
    }

    pub fn parameter_01(&mut self) -> Accessor<u16> {
        Accessor::new(&mut self.parameter01)
    }

    pub fn parameter_02(&mut self) -> Accessor<u16> {
        Accessor::new(&mut self.parameter02)
    }

    pub fn parameter_03(&mut self) -> Accessor<bool> {
        Accessor::new(&mut self.parameter03)
    }

    pub fn parameter_04(&mut self) -> Accessor<u8> {
        Accessor::new(&mut self.parameter04)
    }
}
