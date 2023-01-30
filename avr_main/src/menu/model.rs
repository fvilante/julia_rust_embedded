use core::cell::Cell;

use lib_1::utils::cursor::Cursor;

pub struct ArquivoDeEixo {
    pub parametro1: Cell<u16>,
    pub parametro2: Cell<u16>,
    pub parametro3: Cell<u8>,
    pub parametro4: Cell<u8>,
    pub parametro5: Cell<Cursor>,
    pub parametro6: Cell<Cursor>,
}

pub struct ParametrosDeCpu {
    parametro_cpu_1: Cell<u8>,
    parametro_cpu_2: Cell<u8>,
    parametro_cpu_3: Cell<u8>,
    parametro_cpu_4: Cell<u8>,
}

pub struct MachineModel {
    pub arquivo_de_eixo: ArquivoDeEixo,
    pub parametros_de_cpu: ParametrosDeCpu,
}

impl MachineModel {
    pub fn new() -> Self {
        Self {
            arquivo_de_eixo: ArquivoDeEixo {
                parametro1: Cell::new(0),
                parametro2: Cell::new(0),
                parametro3: Cell::new(0),
                parametro4: Cell::new(0),
                parametro5: Cell::new(Cursor::new(0, 2, 1)),
                parametro6: Cell::new(Cursor::new(0, 2, 1)),
            },
            parametros_de_cpu: ParametrosDeCpu {
                parametro_cpu_1: Cell::new(0),
                parametro_cpu_2: Cell::new(0),
                parametro_cpu_3: Cell::new(0),
                parametro_cpu_4: Cell::new(0),
            },
        }
    }
}
