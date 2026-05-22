use std::collections::BTreeMap;
use crate::modbus::modbus_client::ModbusFunction; 


#[derive(Debug, Default)]
pub struct AppState {
    pub ip_adress: String, 
    pub port: String, 
    pub slave_id: u8,
    pub connect_requested: bool,
    pub active_popup_field: PopupField,
    pub modbus_data: String,
    pub modbus_request_data: ModbusRequestData,
    pub poll_time: u16,
    pub poll_time_input: String,
    pub counter: u16,
    pub connection_error: bool, 
    pub ui_state: UiStates,
}

#[derive(Debug, Default)]
pub struct ModbusRequestData {
    pub slave_id: u8,
    pub function: ModbusFunction, 
    pub start_addr: u16,
    pub quantity: u8,
    pub input_field: ModbusRequestPopupField,
    pub input_field_popup: bool,
}
#[derive(Debug, Default)]
pub enum PopupField {
    #[default]
    Ip,
    Port,
    Poll,
}
#[derive(Debug, Default)]
pub enum ModbusRequestPopupField {
    #[default]
    SlavId,
    StartRegister,
    Quanity,
}

#[derive(Debug, Default)]
pub enum UiStates {
    #[default]
    Home,
    ConfGateway,
    AddRegisters,
    AddRegistersInput,
}



pub fn handle_modbus_data(app: &mut AppState, data: Vec<BTreeMap<u16,u16>>) {

    app.modbus_data.clear(); 
    app.counter = app.counter + 1; 

    for x in data {
        app.modbus_data.push_str(&format!("{:?}", x));
        app.modbus_data.push_str("\n");
    }  
        
    }