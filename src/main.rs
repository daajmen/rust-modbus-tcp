mod modbus;

use std::io::{Read, Write};

use modbus::client::{connect, fetch_data};

use crate::modbus::{
    client::parse_data,
    types::{ApplicationProtocolHeader, ExceptionCode, ExceptionCodeTypes, ModbusPDU},
};

fn main() {
    // Connection
    let client = connect("127.0.0.1".to_string(), "502".to_string());

    match client {
        Ok(mut stream) => {
            println!("We have connection");

            let mut data = fetch_data(
                1,
                0x0000,
                0x0005,
                modbus::types::FunctionCode::ReadMultipleHoldingRegisters,
                stream,
            );
            match data {
                Ok(mut r) => {
                    let response = parse_data(r);

                    match response.function_code {
                        Some(fu) => println!("Function code: {:?}", fu),
                        None => println!("Function code: None"),
                    }

                    match response.exception {
                        Some(ex) => println!("Exception: {:?}", ex),
                        None => println!("Exception: None"),
                    }

                    println!("Modbus data: {:?}", response.modbus_data.unwrap());
                }
                Err(e) => (),
            }
        }

        // TODO
        // -> Convert parsed data to i16, u16, etc.
        //
        Err(e) => println!("{e}"),
    }
}
