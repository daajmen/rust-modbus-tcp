#[derive(Clone, Copy, Debug, Default)]
pub enum ModbusFunction {
    #[default]
    CoilRegister = 1,
    InputStatusRegister = 2,
    InputRegister = 4,
    HoldingRegister = 3,
}

impl ModbusFunction {
    pub fn as_str(&self) -> &'static str {
        match self {
            ModbusFunction::CoilRegister => "ReadCoilRegister",
            ModbusFunction::InputStatusRegister => "ReadInputStatusRegister",
            ModbusFunction::InputRegister => "ReadInputRegister",
            ModbusFunction::HoldingRegister => "ReadHoldingRegister",
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ModbusRequestData {
    pub slave_id: Option<u8>,
    pub function: ModbusFunction,
    pub start_addr: Option<u16>,
    pub quantity: Option<u8>,
}

impl ModbusRequestData {
    pub fn as_string(&self) -> String {
        format!(
            "id: {}\n{:?}\nStartReg: {}\nQuanity: {} \n---------",
            match self.slave_id {
                Some(v) => v.to_string(),
                None => "-".to_string(),
            },
            self.function,
            match self.start_addr {
                Some(v) => v.to_string(),
                None => "-".to_string(),
            },
            match self.quantity {
                Some(v) => v.to_string(),
                None => "-".to_string(),
            }
        )
    }

    pub fn clear_data(&mut self) {
        self.slave_id = None;
        self.start_addr = None;
        self.quantity = None;
    }
}
#[derive(Debug, Default)]
pub struct RegisterData {
    pub register: u16,
    pub data: u16,
}
