use color_eyre::{Result};  
use crate::ui::app::{AppState, UiStates};
use crate::modbus::modbus_client::{ModbusFunction};
use ui::ui::run; 

mod ui; 
mod modbus; 

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app_state = AppState { 
        ip_adress: "127.0.0.1".to_string(),
        port: "502".to_string(), 
        connect_requested: false, 
        modbus_data: "".to_string(),
        modbus_requests: vec![],
        modbus_request_data: ui::app::ModbusRequestData { slave_id: None, function: ModbusFunction::ReadCoilRegister, start_addr: None, quantity: None}, 
        modbus_write_request: false, 
        poll_time: Some(1500),
        counter: 0,
        connection_error: false,
        ui_state: UiStates::Home, 
    };



    let terminal = ratatui::init(); 
    let result = run(terminal, &mut app_state); 
    ratatui::restore(); 

    result
        
}

