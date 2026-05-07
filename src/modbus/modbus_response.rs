

pub fn decode_response(byte_count : usize, stream_response: &[u8] ) -> Vec<u16> {

    let mut response : Vec<u16> = Vec::new(); 

    for i in (9..byte_count).step_by(2){
        let var = u16::from_be_bytes([stream_response[i], stream_response[i+1]]);

    response.push(var);

    }

return response; 
} 


pub fn decode_response_bits(byte_count : usize, quantity : u16, stream_response: &[u8] ) -> Vec<u16> {

    let mut response : Vec<u16> = Vec::new(); 

    for _i in 9..byte_count {
        
        for i in 0..quantity {
            let bit = (stream_response[9] >> i ) & 1; 
            response.push(bit as u16);

        }

    }

return  response;

}
