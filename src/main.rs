mod modbus;

use std::io::{Read, Write};

use modbus::client::connect;
use modbus::types::Frame;

use crate::modbus::types::{ApplicationProtocolHeader, ModbusPDU};

fn main() {
    // Connection
    let client = connect("127.0.0.1".to_string(), "502".to_string());
    let mut response = [0u8; 254];

    match client {
        Ok(mut stream) => {
            println!("We have connection");

            // Creating frame
            let header: ApplicationProtocolHeader = ApplicationProtocolHeader {
                transaction_identifier: [0x12, 0x34],
                protocol_identifiter: [0x00, 0x00],
                length_field: [0x00, 0x06],
                unit_identifier: 0x01,
            };
            let mut modbus_data: ModbusPDU = ModbusPDU {
                function_code: modbus::types::FunctionCode::ReadMultipleHoldingRegisters,
                data: Some([0x0001, 0x0001]),
            };

            let mut modbus_frame: Frame = Frame {
                application_header: header,
                modbus_pdu: modbus_data,
            };

            println!("Sending buffer {:?}", modbus_frame.as_vec());
            match stream.write_all(&modbus_frame.as_vec()) {
                Ok(()) => println!("Success, data sent"),
                Err(e) => println!("ERROR {e}"),
            };
            match stream.read(&mut response) {
                Ok(r) => {
                    println!("Data recivied, size -> {r}");
                }
                Err(e) => println!("ERROR: {e}"),
            }
        }
        Err(e) => println!("{e}"),
    }
}
