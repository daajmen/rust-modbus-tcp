use super::types::Frame;
use crate::modbus::types::{
    ApplicationProtocolHeader, ExceptionCode, FunctionCode, ModbusPDU, ResponseData,
};
use std::io::{Read, Result, Write};
use std::net::TcpStream;

pub fn connect(adress: String, port: String) -> Result<TcpStream> {
    let connection_info = format!("{}:{}", adress, port);

    match TcpStream::connect(connection_info) {
        Ok(stream) => Ok(stream),
        Err(e) => Err(e),
    }
}

pub fn fetch_data(
    unit_id: u8,
    start_reg: u16,
    quanitiy: u16,
    fn_code: FunctionCode,
    mut stream: TcpStream,
) -> Result<Vec<u8>> {
    let mut result: Vec<u8> = Vec::new();
    let mut response = [255u8; 254];

    // Modbus request frame
    let mut modbus_data: ModbusPDU = ModbusPDU {
        function_code: fn_code,
        data: [start_reg, quanitiy], // start reg, quantity
    };

    // Creating frame
    let mut header: ApplicationProtocolHeader = ApplicationProtocolHeader {
        transaction_identifier: [0x12, 0x34],
        protocol_identifiter: [0x00, 0x00],
        length_field: [0x00, 0x00],
        unit_identifier: unit_id,
    };

    let mut modbus_frame: Frame = Frame {
        application_header: header,
        modbus_pdu: modbus_data,
    };

    match stream.write_all(&modbus_frame.as_vec()) {
        Ok(()) => match stream.read(&mut response) {
            Ok(r) => {
                println!("Data recivied, size -> {r}");
                result.extend_from_slice(&response[0..r]);
                Ok(result)
            }
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

pub fn parse_data(mut data: Vec<u8>) -> ResponseData {
    data.reverse();

    let mut response_data: ResponseData = ResponseData {
        function_code: None,
        exception: None,
        modbus_data: None,
    };

    let tran = [data.pop(), data.pop()];
    let ident = [data.pop(), data.pop()];
    let length = [data.pop(), data.pop()];
    let unit = data.pop();
    let function_byte = data.pop();
    let mut exception: ExceptionCode = ExceptionCode {
        exception_code: None,
        exception_message: None,
    };
    let mut byte_count_data: Option<u8> = Some(0);

    if function_byte.is_some() {
        if exception.is_exception(function_byte.unwrap()) {
            exception.get_exception_message(data.pop().unwrap());
            response_data.exception = Some(exception);
        } else {
            byte_count_data = data.pop();
        }
    }
    match function_byte {
        Some(d) => response_data.function_code = FunctionCode::get_function_code(d),
        None => (),
    }

    data.reverse();
    response_data.modbus_data = Some(data);
    response_data
}

pub fn convert_to_u16(data: Vec<u8>) -> Vec<u16> {
    let mut converted_data: Vec<u16> = Vec::new();

    for i in (0..data.len() - 1).step_by(2) {
        println!("{:?},{:?}", data[i], data[i + 1]);
        let var = u16::from_be_bytes([data[i], data[i + 1]]);
        converted_data.push(var);
    }

    converted_data
}

pub fn convert_to_i16(data: Vec<u8>) -> Vec<i16> {
    let mut converted_data: Vec<i16> = Vec::new();

    for i in (0..data.len() - 1).step_by(2) {
        println!("{:?},{:?}", data[i], data[i + 1]);
        let var = i16::from_be_bytes([data[i], data[i + 1]]);
        converted_data.push(var);
    }

    converted_data
}
