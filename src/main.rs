use std::io::prelude::*; // traits: Read, Write (ger .read/.write)
use std::net::TcpStream;

fn main() -> std::io::Result<()> { // main kan returnera fel (Result)

    let mut stream = TcpStream::connect("127.0.0.1:502")?; 

    let transaction_id : u16 = 0x1501; 
    let protocol_id : u16 = 0;  
    let function_code : u8 = 3; 

    let unit_id : u8 = 1; 
    
    
    
    let data= [0x00, 0x00, 0x00, 0x10]; // Startadress 2byte, quantity 2byte  
    let length: u16 = 1 + 1 + data.len() as u16; 

    // Request 
    let mut request : Vec<u8> = Vec::new(); 

    request.extend_from_slice(&transaction_id.to_be_bytes()); // High / Low
    request.extend_from_slice(&protocol_id.to_be_bytes()); // 00 = modbus TCP
    request.extend_from_slice(&length.to_be_bytes()); // unit_id + function_code + data

    request.push(unit_id); // ID 
    request.push(function_code); // Modbus kod 

    request.extend_from_slice(&data); // Payload


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

    let mut reg_index : u16 = u16::from_be_bytes([data[0], data[1]]) + (function_code as u16 * 10000); 

    for i in (9..byte_count).step_by(2){
        let regs = u16::from_be_bytes([response[i], response[i+1]]);

        println!("{} -> {}", reg_index,  regs);  
        reg_index = reg_index +1;        
    }


    Ok(())
    // returnerar OK om allt gick bra
}