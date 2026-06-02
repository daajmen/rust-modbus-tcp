use crate::modbus::types::{ModbusFunction, ModbusRequestData, RegisterData};

/// States for tui element
#[derive(Debug, Default)]
pub enum UiStates {
    #[default]
    Home,
    ConfGateway,
    AddRegisters,
    AddRegistersInput,
}

/// States for connection status
#[derive(Debug, Default)]
pub enum ConnectionStatus {
    #[default]
    Disconnected,
    InitilizeConnection,
    Connected,
    ConnectionErrorTimeOut,
}

/// States to handle connection logic
#[derive(Debug, Default)]
pub struct AppState {
    pub connection_settings: ConnectionSettingsData,
    pub connect_requested: bool,
    pub connection_error: bool,
    pub connection_status: ConnectionStatus,
    pub modbus_data: Vec<RegisterData>,
    pub modbus_requests: Vec<ModbusRequestData>,
    pub modbus_request_data: ModbusRequestData,
    pub modbus_write_request: bool,
    pub counter: u16,
    pub ui_state: UiStates,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            connection_settings: ConnectionSettingsData {
                ip_adress: "127.0.0.1".to_string(),
                port: 502.to_string(),
                poll_time: Some(1500),
                init: false,
            },
            connect_requested: false,
            connection_status: ConnectionStatus::Disconnected,
            modbus_data: vec![],
            modbus_requests: vec![],
            modbus_request_data: ModbusRequestData {
                slave_id: None,
                function: ModbusFunction::ReadCoilRegister,
                start_addr: None,
                quantity: None,
            },
            modbus_write_request: false,
            counter: 0,
            connection_error: false,
            ui_state: UiStates::Home,
        }
    }
    pub fn update_state(&mut self) {
        if self.connect_requested && !self.connection_settings.init {
            self.connection_status = ConnectionStatus::InitilizeConnection;
        } else if self.connect_requested && !self.connection_error {
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
    pub init: bool,
}
