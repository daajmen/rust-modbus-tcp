mod modbus;

use std::io::{Read, Write};

use modbus::client::{connect, fetch_data};

use crate::modbus::types::{ApplicationProtocolHeader, ModbusPDU};

fn main() {
    // Connection
    let client = connect("127.0.0.1".to_string(), "502".to_string());

    match client {
        Ok(mut stream) => {
            println!("We have connection");

            let mut data = fetch_data(
                1,
                0x0001,
                0x0008,
                modbus::types::FunctionCode::ReadMultipleHoldingRegisters,
                stream,
            );
            println!("{:?}", data);
        }
        Err(e) => println!("{e}"),
    }
}
