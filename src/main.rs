use std::io::prelude::*; // traits: Read, Write (ger .read/.write)
use std::net::TcpStream; // TCP socket-typ

fn main() -> std::io::Result<()> { // main kan returnera fel (Result)


    let transaction_id : u16 = 0x1501; 
    let protocol_id : u16 = 0;  
    let function_code : u8 = 3; 

    let unit_id : u8 = 1; 
    
    
    
    let data= [0x13, 0x37, 0x00, 0x10]; // Startadress 2byte, quantity 2byte  
    let length: u16 = 1 + 1 + data.len() as u16; 

    // Request 
    let mut request : Vec<u8> = Vec::new(); 

    request.extend_from_slice(&transaction_id.to_be_bytes()); // High / Low
    request.extend_from_slice(&protocol_id.to_be_bytes()); // 00 = modbus TCP
    request.extend_from_slice(&length.to_be_bytes()); // unit_id + function_code + data

    request.push(unit_id); // ID 
    request.push(function_code); // Modbus kod 

    request.extend_from_slice(&data); // Payload


    let mut stream = TcpStream::connect("192.168.50.15:5020")?; 

    stream.write(&request)?;

    Ok(())
    // returnerar OK om allt gick bra
}