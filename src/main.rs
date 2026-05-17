use color_eyre::{Result};  
use crate::ui::app::{AppState, PopupField};
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
        show_config_popup: false, 
        active_popup_field: PopupField::Ip,
        modbus_data: "".to_string(),
        poll_time: 1500,
        counter: 0,
        connection_error: false, 
    };



    let terminal = ratatui::init(); 
    let result = run(terminal, &mut app_state); 
    ratatui::restore(); 

    result
        
}

