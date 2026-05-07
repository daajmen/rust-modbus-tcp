

pub fn decode_response(byte_count : usize, stream_response: &[u8] ) -> Vec<u16> {


let mut response : Vec<u16> = Vec::new(); 

for i in (9..byte_count).step_by(2){
    let var = u16::from_be_bytes([stream_response[i], stream_response[i+1]]);

    response.push(var);

}


return response; 


} 