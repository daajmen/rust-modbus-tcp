use std::collections::BTreeMap;


#[derive(Debug, Default)]
pub struct AppState {
    pub ip_adress: String, 
    pub port: String, 
    pub slave_id: u8,
    pub connect_requested: bool,
    pub show_config_popup: bool, 
    pub active_popup_field: PopupField,
    pub modbus_data: String,
    pub poll_time: u16,
    pub counter: u16,
}

#[derive(Debug, Default)]
pub enum PopupField {
    #[default]
    Ip,
    Port,
    Poll,
}

pub fn handle_modbus_data(app: &mut AppState, data: Vec<BTreeMap<u16,u16>>) {

    app.modbus_data.clear(); 
    app.counter = app.counter + 1; 

    for x in data {
        app.modbus_data.push_str(&format!("{:?}", x));
        app.modbus_data.push_str("\n");
    }  
        
    }