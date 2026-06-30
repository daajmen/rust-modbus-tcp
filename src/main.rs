mod modbus;

use std::io::{Read, Write};

use modbus::client::{connect, fetch_data};

use crate::modbus::types::{ApplicationProtocolHeader, ModbusExceptionCode, ModbusPDU};

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

                    println!("Transaction ID -> {:x?} \n", tran);
                    println!("Protocol identifier -> {:x?}\n", ident);
                    println!("Length -> {:x?} \n", length);
                    println!("Unit ID -> {:x?}\n", unit);
                    // If it contains 8x then we have an exception,
                    match function_byte {
                        Some(value) => {
                            if value >= 0x80 {
                                println!("EXECPTION!! -> {:x?}\n", value);
                                let exeception = r.pop();

                                match exeception {
                                    Some(ex) => {
                                        match ex {
                                            0x01 => {
                                                println!(
                                                    "{:?}, -> {:?}",
                                                    ex,
                                                    ModbusExceptionCode::IllegalFunction
                                                )
                                            }
                                            0x02 => println!(
                                                "{:?}, -> {:?}",
                                                ex,
                                                ModbusExceptionCode::IllegalDataAdress
                                            ),
                                            0x03 => {
                                                println!(
                                                    "{:?}, -> {:?}",
                                                    ex,
                                                    ModbusExceptionCode::IllegalDataValue
                                                )
                                            }
                                            0x04 => println!(
                                                "{:?}, -> {:?}",
                                                ex,
                                                ModbusExceptionCode::ServerDeviceFailure
                                            ),
                                            0x05 => {
                                                println!(
                                                    "{:?}, -> {:?}",
                                                    ex,
                                                    ModbusExceptionCode::Acknowledge
                                                )
                                            }
                                            0x06 => {
                                                println!(
                                                    "{:?}, -> {:?}",
                                                    ex,
                                                    ModbusExceptionCode::ServerDeviceBusy
                                                )
                                            }
                                            0x07 => println!(
                                                "{:?}, -> {:?}",
                                                ex,
                                                ModbusExceptionCode::NegativeAcknowledge
                                            ),
                                            0x08 => println!(
                                                "{:?}, -> {:?}",
                                                ex,
                                                ModbusExceptionCode::MemoryParityError
                                            ),
                                            0x10 => {
                                                println!(
                                                    "{:?}, -> {:?}",
                                                    ex,
                                                    ModbusExceptionCode::GatewayPathUnavailable
                                                )
                                            }
                                            0x11 => println!(
                                            "{:?}, -> {:?}",
                                            ex,
                                            ModbusExceptionCode::GatewayTargetDeviceFailedToRespond
                                        ),
                                            _ => (),
                                        };
                                    }
                                    None => (),
                                }
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
