use color_eyre::{Result};  
use crate::ui::app::{AppState, ModbusRequestPopupField, PopupField, UiStates};
use crate::modbus::modbus_client::{ModbusFunction};
use ui::ui::run; 

mod ui; 
mod modbus; 

fn main() -> Result<()> {
    color_eyre::install()?;
    let mut app_state = AppState { 
        ip_adress: "127.0.0.1".to_string(),
        port: "502".to_string(), 
        slave_id: 1, 
        connect_requested: false, 
        active_popup_field: PopupField::Ip,
        modbus_data: "".to_string(),
        modbus_request_data: { ui::app::ModbusRequestData { slave_id: 1, function: ModbusFunction::ReadCoilRegister, start_addr: 1, quantity: 1 , input_field: ModbusRequestPopupField::StartRegister, input_field_popup: false}}, 
        poll_time: 1500,
        poll_time_input: "1500".to_string(), 
        counter: 0,
        connection_error: false,
        ui_state: UiStates::Home, 
    };



    let terminal = ratatui::init(); 
    let result = run(terminal, &mut app_state); 
    ratatui::restore(); 

    result
        
}

