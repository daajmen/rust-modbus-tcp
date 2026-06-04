use std::collections::BTreeMap;

pub fn decode_response(
    byte_count: usize,
    stream_response: &[u8],
    start_addr: u16,
) -> BTreeMap<u16, u16> {
    let mut response: BTreeMap<u16, u16> = BTreeMap::new();
    let mut counter: u16 = start_addr;

    for i in (9..byte_count).step_by(2) {
        let var = u16::from_be_bytes([stream_response[i], stream_response[i + 1]]);
        response.insert(counter, var);
        counter += 1;
    }

    response
}

pub fn decode_response_bits(
    quantity: u8,
    stream_response: &[u8],
    start_addr: u16,
) -> BTreeMap<u16, u16> {
    let mut response = BTreeMap::new();

    for bit_index in 0..quantity {
        let byte_index = 9 + (bit_index / 8) as usize;
        let bit_pos = bit_index % 8;

        let bit = (stream_response[byte_index] >> bit_pos) & 1;

        response.insert(start_addr + bit_index as u16, bit as u16);
    }

    response
}
