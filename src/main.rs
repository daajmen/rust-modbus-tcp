use std::io::prelude::*;
use crate::modbus::{modbus_client, modbus_response}; 
use modbus::modbus_requests::{ModbusFunction, build_read_register}; 

mod modbus; 


fn main() -> std::io::Result<()> { 

    let ip_adress = "127.0.0.1:502";
    let slave_id = 1;  
    let start_adress = 0;
    let quantity = 6; 

    let mut stream = modbus_client::connect_modbus(ip_adress)?; 

// ReadCoilRegister

    let request_coil = build_read_register( ModbusFunction::ReadCoilRegister, slave_id, start_adress, quantity); 

    // Write 
    stream.write(&request_coil)?;

    // Placeholder
    let mut response_coil = [0u8; 254]; 
    let byte_count: usize = stream.read(&mut response_coil)?; 

    println!("ReadCoilRegister -> {:?}", modbus_response::decode_response_bits(byte_count, quantity, &response_coil));

 // ReadInputStatusRegister

    let request_inputstatus = build_read_register( ModbusFunction::ReadInputStatusRegister, slave_id, start_adress, quantity); 
    // Write 
    stream.write(&request_inputstatus)?;

    // Placeholder
    let mut response_inputstatus = [0u8; 254]; 
    let byte_count: usize = stream.read(&mut response_inputstatus)?; 

    println!("ReadInputStatusRegister -> {:?}", modbus_response::decode_response_bits(byte_count, quantity, &response_inputstatus));

// ReadInputRegister

    let request_inputregister = build_read_register( ModbusFunction::ReadInputRegister, slave_id, start_adress, quantity); 

    // Write 
    stream.write(&request_inputregister)?;

    // Placeholder
    let mut response_inputregister = [0u8; 254]; 
    let byte_count: usize = stream.read(&mut response_inputregister)?; 

    println!("ReadInputRegister -> {:?}", modbus_response::decode_response(byte_count, &response_inputregister));

// ReadHoldingRegister

    let request_holding = build_read_register( ModbusFunction::ReadHoldingRegister, slave_id, start_adress, quantity); 

    // Write 
    stream.write(&request_holding)?;

    // Placeholder
    let mut response_holdingregister = [0u8; 254]; 
    let byte_count: usize = stream.read(&mut response_holdingregister)?; 

    println!("ReadHoldingRegister -> {:?}", modbus_response::decode_response(byte_count, &response_holdingregister));

    Ok(())
}