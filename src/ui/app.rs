use crate::modbus::modbus_client::ModbusFunction;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
pub enum UiStates {
    #[default]
    Home,
    ConfGateway,
    AddRegisters,
    AddRegistersInput,
}

#[derive(Debug, Default)]
pub enum ConnectionStatus {
    #[default]
    Disconnected,
    Connected,
    ConnectionErrorTimeOut,
}

#[derive(Debug, Default)]
pub struct AppState {
    pub connection_settings: ConnectionSettingsData,
    pub connect_requested: bool,
    pub connection_error: bool,
    pub connection_status: ConnectionStatus,
    pub modbus_data: String,
    pub modbus_requests: Vec<ModbusRequestData>,
    pub modbus_request_data: ModbusRequestData,
    pub modbus_write_request: bool,
    pub counter: u16,
    pub ui_state: UiStates,
}

impl AppState {
    pub fn update_state(&mut self) {
        if self.connect_requested && !self.connection_error {
            self.connection_status = ConnectionStatus::Connected;
        } else if self.connection_error {
            self.connection_status = ConnectionStatus::ConnectionErrorTimeOut;
        } else {
            self.connection_status = ConnectionStatus::Disconnected;
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct ConnectionSettingsData {
    pub ip_adress: String,
    pub port: String,
    pub poll_time: Option<u16>,
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

pub fn handle_modbus_data(app: &mut AppState, data: Vec<BTreeMap<u16, u16>>) {
    app.modbus_data.clear();
    app.counter = app.counter + 1;

    for x in data {
        app.modbus_data.push_str(&format!("{:?}", x));
        app.modbus_data.push_str("\n");
    }
}
