use std::io::prelude::*;
use std::io::Result;
use std::collections::HashMap;

use std::{net::TcpStream};
use crate::modbus::modbus_response::{decode_response_bits, decode_response};

#[derive(Clone, Copy)]
pub enum ModbusFunction {
    ReadCoilRegister = 1, 
    ReadInputStatusRegister = 2, 
    ReadHoldingRegister = 3, 
    ReadInputRegister = 4, 
}

pub struct ModbusMaster {
    addr: String, 
    stream : Option<TcpStream>,
}

impl ModbusMaster {
    
    // Build instance
    pub fn new(addr: &str) -> Self {
        Self {
            addr: addr.to_string(),
            stream: None,
        }
    }

    // Connect to Modbus server
    pub fn connect(&mut self) {

        let s = TcpStream::connect(&self.addr); 

        self.stream = s.ok(); 
    }


    // Read modbus register
    pub fn read_modbus_register(&mut self, modbus_function: ModbusFunction,unit_id: u8, start_addr: u16, quantity : u16 ) -> Result<HashMap<u16, u16>> {

        fn build_read_register(modbus_function : ModbusFunction, unit_id: u8, start_addr: u16, quantity: u16 ) -> Vec<u8> {

            let transaction_id : u16 = 0x1501; 
            let protocol_id : u16 = 0;  
            
            // Startadress 2byte, quantity 2byte    
            
            let addr_bytes = start_addr.to_be_bytes();  
            let qty_bytes = quantity.to_be_bytes();  
            
            
            let data = [ addr_bytes[0], addr_bytes[1], qty_bytes[0], qty_bytes[1]]; 
            let length: u16 = 1 + 1 + data.len() as u16; 

            // Request 
            let mut request : Vec<u8> = Vec::new(); 

            request.extend_from_slice(&transaction_id.to_be_bytes()); // High / Low
            request.extend_from_slice(&protocol_id.to_be_bytes()); // 00 = modbus TCP
            request.extend_from_slice(&length.to_be_bytes()); // unit_id + function_code + data

            request.push(unit_id); // ID <
            request.push(modbus_function as u8); // Modbus kod 

            request.extend_from_slice(&data); // Payload

            return request; 
        }

        let mb_function = modbus_function.clone(); 

        

        // Unpack
        let stream = self.stream.as_mut().unwrap(); 

        let modbus_request = build_read_register(modbus_function, unit_id, start_addr, quantity);
        stream.write(&modbus_request)?;

        let mut response = [0u8; 254]; 
        let byte_count: usize = stream.read(&mut response)?; 

        
        
        // Try to pair. 
        let map_start = start_addr + (modbus_function as u16 * 10000);



        match mb_function {
            ModbusFunction::ReadCoilRegister => Ok(decode_response_bits(byte_count, quantity, &response, map_start )), 
            ModbusFunction::ReadInputStatusRegister =>  Ok(decode_response_bits(byte_count, quantity, &response, map_start)),
            _ => Ok(decode_response(byte_count, &response, map_start))
        }
        

    }



}




