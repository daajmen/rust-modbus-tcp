mod modbus;

use std::io::{Read, Write};

use modbus::client::{connect, fetch_data};

use crate::modbus::types::{
    ApplicationProtocolHeader, ExceptionCode, ExceptionCodeTypes, ModbusPDU,
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
                    r.reverse();
                    println!("Raw data -> {:x?} \n", r);
                    println!("Decimal -> {:?} \n", r);

                    let tran = [r.pop(), r.pop()];
                    let ident = [r.pop(), r.pop()];
                    let length = [r.pop(), r.pop()];
                    let unit = r.pop();
                    let function_byte = r.pop();
                    let mut exception: ExceptionCode = ExceptionCode {
                        exception_code: None,
                        exception_message: None,
                    };

                    println!("Transaction ID -> {:x?} \n", tran);
                    println!("Protocol identifier -> {:x?}\n", ident);
                    println!("Length -> {:x?} \n", length);
                    println!("Unit ID -> {:x?}\n", unit);
                    // If it contains 8x then we have an exception,
                    match function_byte {
                        Some(value) => {
                            if value >= 0x80 {
                                println!("EXCEPTION!! -> {:x?}\n", value);
                                let exception_value = r.pop().unwrap();

                                exception.get_exception_message(exception_value);

                                println!(
                                    "Exception code -> {:?}",
                                    exception.exception_code.unwrap()
                                );
                                println!("Message -> {:?}", exception.exception_message.unwrap());
                            } else {
                                println!("FunctionByte byte -> {:?}", value);
                                let byte_count_data = r.pop();
                                println!("Data size in bytes -> {:?}", byte_count_data);
                            }
                            // Reverse data to handle to the correct order according to register
                            r.reverse();
                            println!("Remaining data -> {:?} \n", r);
                        }
                        None => (),
                    }
                }
                Err(e) => (),
            }
        }
        Err(e) => println!("{e}"),
    }
}
