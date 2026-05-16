use color_eyre::{Result}; 
use crossterm::{event::{self, Event}, terminal}; 
use ratatui::{DefaultTerminal, Frame, style::{Color, Stylize}, text::ToSpan, widgets::{Block, Borders, Paragraph, Widget}}; 
use ratatui::layout::{Layout, Direction, Constraint};
use modbus::modbus_client::ModbusMaster;
use crate::modbus::modbus_client::ModbusFunction;
use ui::ui::run; 

mod ui; 
mod modbus; 

fn main() -> Result<()> {

    let ip_adress = "127.0.0.1";
    let port = "502"; 
    let slave_id = 1;  
    let start_adress = 0;
    let quantity = 6; 
    let poll_time = 1500; 
    let mut poll_count = 0; 
    let mut master = ModbusMaster::new(ip_adress, port);

    master.connect();


    let data = [master.read_modbus_register(
        ModbusFunction::ReadCoilRegister,
        slave_id,
        start_adress,
        quantity)?,

        master.read_modbus_register(
        ModbusFunction::ReadInputStatusRegister,
        slave_id,
        start_adress,
        quantity)?,
        
        master.read_modbus_register(
        ModbusFunction::ReadInputRegister,
        slave_id,
        start_adress,
        quantity)?,
        
        master.read_modbus_register(
        ModbusFunction::ReadHoldingRegister,
        slave_id,
        start_adress,
        quantity)?,        
        ];  




    color_eyre::install()?;
    let terminal = ratatui::init(); 
    let result = run(terminal, data.to_vec()); 
    ratatui::restore();
    result
}

