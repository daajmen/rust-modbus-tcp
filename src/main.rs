use std::thread; 
use std::time::Duration; 
use std::io::{self, Write};
use std::io::stdout;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};


use modbus::modbus_client::ModbusMaster;
use crate::modbus::modbus_client::ModbusFunction;
use ui::dashboard::App; 
mod modbus; 
mod ui; 






fn main() -> std::io::Result<()> { 

    let ip_adress = "127.0.0.1:502";
    let slave_id = 1;  
    let start_adress = 0;
    let quantity = 6; 
    let poll_time = 1500; 
    let mut poll_count = 0; 

    let mut master = ModbusMaster::new(ip_adress);
    master.connect();

    ratatui::run(|terminal| App::default().run(terminal))


//    loop {
//        execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).unwrap();
//        // Connect to server
//
//        let md_coil = master.read_modbus_register(
//            ModbusFunction::ReadCoilRegister,
//            slave_id,
//            start_adress,
//            quantity)?;
//
//
//        let mb_inputstatus = master.read_modbus_register(
//            ModbusFunction::ReadInputStatusRegister,
//            slave_id,
//            start_adress,
//            quantity)?;
//
//
//        let mb_inputreg = master.read_modbus_register(
//            ModbusFunction::ReadInputRegister,
//            slave_id,
//            start_adress,
//            quantity)?;
//
//
//        let mb_holding = master.read_modbus_register(
//            ModbusFunction::ReadHoldingRegister,
//            slave_id,
//            start_adress,
//            quantity)?;
//        
//        poll_count = poll_count +1; 
//        io::stdout().flush().unwrap();
//
//        println!("### RUST-MODBUS-TCP v.0.0.1-beta ###"); 
//        println!("------------------------------------------------------------------------------------------|");
//        println!("{:<20} | {} ", "CONNECTED TO " , ip_adress  ); 
//        println!("{:<20} | {:?} ", "Poll counter ->  " , poll_count  ); 
//        println!("{:<20} | {:?} MS", "POLL TIME" , poll_time  ); 
//        println!("------------------------------------------------------------------------------------------|"); 
//        println!("{:<20} | {:?}", "COILS", md_coil);
//        println!("------------------------------------------------------------------------------------------|"); 
//        println!("{:<20} | {:?}", "INPUT STATUS", mb_inputstatus);
//        println!("------------------------------------------------------------------------------------------|"); 
//        println!("{:<20} | {:?}", "INPUT REGISTER", mb_inputreg);
//        println!("------------------------------------------------------------------------------------------|"); 
//        println!("{:<20} | {:?}", "HOLDING REGISTER", mb_holding);
//        println!("------------------------------------------------------------------------------------------|"); 
//
//
//
//
//        thread::sleep(Duration::from_millis(1500));
//    }
// 





}
