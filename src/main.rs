use std::io::prelude::*;
use modbus::modbus_client;

use crate::modbus::modbus_requests; 

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

    // Skapa plats för svar  
    let mut response = [0u8; 254]; 

    // Kolla storlek på svaret
    let byte_count: usize = stream.read(&mut response)?; 

    // Skriv ut antal bytes 
    println!("Number of bytes in answer -> {}", byte_count);
    // Printa ut rådata från modbus.  
    println!("Response -> {:02x?}", &response[..byte_count]);


    for i in (9..byte_count).step_by(2){
        let regs = u16::from_be_bytes([response[i], response[i+1]]);

        println!("-> {}",  regs);  
    }


    Ok(())
    // returnerar OK om allt gick bra
}