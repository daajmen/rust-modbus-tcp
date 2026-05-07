use std::io::prelude::*;
use modbus::modbus_client;
use crate::modbus::{modbus_requests, modbus_response::decode_response}; 

mod modbus; 


fn main() -> std::io::Result<()> { // main kan returnera fel (Result)

    let ip_adress = "127.0.0.1:502";
    let slave_id = 1;  
    let start_adress = 0;
    let quantity = 10; 

    let mut stream = modbus_client::connect_modbus(ip_adress)?; 

    let request = modbus_requests::build_read_holdingreg(slave_id, start_adress, quantity); 

    // Write 
    stream.write(&request)?;

    // Placeholder
    let mut response = [0u8; 254]; 
    let byte_count: usize = stream.read(&mut response)?; 

    println!("{:?}", decode_response(byte_count, &response));


    Ok(())
    // returnerar OK om allt gick bra
}