

pub fn build_read_holdingreg(unit_id: u8, start_addr: u16, quantity: u16 ) -> Vec<u8> {

    let transaction_id : u16 = 0x1501; 
    let protocol_id : u16 = 0;  
    let function_code : u8 = 3; 
    
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
    request.push(function_code); // Modbus kod 

    request.extend_from_slice(&data); // Payload

    return request; 
}

