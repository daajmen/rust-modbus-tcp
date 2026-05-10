use std::collections::HashMap;


pub fn decode_response(byte_count : usize, stream_response: &[u8], start_addr : u16 ) -> HashMap<u16, u16> {

    let mut response : HashMap<u16, u16> = HashMap::new();
    let mut counter: u16 = start_addr; 

    for i in (9..byte_count).step_by(2){
        let var = u16::from_be_bytes([stream_response[i], stream_response[i+1]]);
        response.insert(counter, var); 
        counter = counter +1;         
    }

return response; 
} 


pub fn decode_response_bits(byte_count : usize, quantity : u16, stream_response: &[u8], start_addr : u16 ) -> HashMap<u16, u16> {

    let mut response : HashMap<u16, u16> = HashMap::new();
    let mut counter: u16 = start_addr; 
    for _i in 9..byte_count {
        
        for i in 0..quantity {
            let bit = (stream_response[9] >> i ) & 1; 
            response.insert(counter, bit as u16);
            counter = counter +1; 

        }

    }

return response;

}
