use crate::modbus::types::{ModbusFunction, ModbusRequestData};
use crate::runtime::run::run;
use crate::ui::app::{AppState, ConnectionSettingsData, ConnectionStatus, UiStates};
use color_eyre::Result;

mod modbus;
mod runtime;
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app_state = AppState {
        connection_settings: ConnectionSettingsData {
            ip_adress: "127.0.0.1".to_string(),
            port: 502.to_string(),
            poll_time: Some(1500),
        },
        connect_requested: false,
        connection_status: ConnectionStatus::Disconnected,
        modbus_data: "".to_string(),
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
    };

    let terminal = ratatui::init();
    let result = run(terminal, &mut app_state);
    ratatui::restore();

    result
}
