use std::collections::BTreeMap;
use std::io::Result;
use std::io::prelude::*;

use std::net::TcpStream;

use crate::modbus::modbus_response::{decode_response, decode_response_bits};
use crate::modbus::types::{ModbusFunction, ModbusRequestData};

pub struct ModbusMaster {
    addr: String,
    port: String,
    pub stream: Option<TcpStream>,
}

impl ModbusMaster {
    // Build instance
    pub fn new(addr: &str, port: &str) -> Self {
        Self {
            addr: addr.to_string(),
            port: port.to_string(),
            stream: None,
        }
    }

    // Connect to Modbus server
    pub fn connect(&mut self) -> Result<()> {
        let connection_info = format!("{}:{}", &self.addr, &self.port);

        match TcpStream::connect(connection_info) {
            Ok(stream) => {
                self.stream = Some(stream);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    fn build_read_register(
        modbus_function: ModbusFunction,
        unit_id: u8,
        start_addr: u16,
        quantity: u16,
    ) -> Vec<u8> {
        let transaction_id: u16 = 0x1501;
        let protocol_id: u16 = 0;

        // Startadress 2byte, quantity 2byte

        let addr_bytes = start_addr.to_be_bytes();
        let qty_bytes = quantity.to_be_bytes();

        let data = [addr_bytes[0], addr_bytes[1], qty_bytes[0], qty_bytes[1]];
        let length: u16 = 1 + 1 + data.len() as u16;

        // Request
        let mut request: Vec<u8> = Vec::new();

        request.extend_from_slice(&transaction_id.to_be_bytes()); // High / Low
        request.extend_from_slice(&protocol_id.to_be_bytes()); // 00 = modbus TCP
        request.extend_from_slice(&length.to_be_bytes()); // unit_id + function_code + data

        request.push(unit_id); // ID <
        request.push(modbus_function as u8); // Modbus kod

        request.extend_from_slice(&data); // Payload

        request
    }

    // Read modbus register
    pub fn read_modbus_register(
        &mut self,
        request_data: ModbusRequestData,
    ) -> Result<BTreeMap<u16, u16>> {
        let mb_function = request_data.function;

        let Some(stream) = self.stream.as_mut() else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotConnected,
                "Gateway not responding",
            ));
        };

        let Some(slave_id) = request_data.slave_id else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Missing slave_id",
            ));
        };

        let Some(start_addr) = request_data.start_addr else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Missing start adress",
            ));
        };

        let Some(quantity) = request_data.quantity else {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Missing quanity",
            ));
        };

        let modbus_request =
            Self::build_read_register(request_data.function, slave_id, start_addr, quantity as u16);
        stream.write_all(&modbus_request)?;

        let mut response = [0u8; 254];
        let byte_count: usize = stream.read(&mut response)?;

        match mb_function {
            ModbusFunction::Coil => Ok(decode_response_bits(
                quantity,
                &response,
                start_addr + 10000,
            )),
            ModbusFunction::DiscreteInput => Ok(decode_response_bits(
                quantity,
                &response,
                start_addr + 20000,
            )),
            ModbusFunction::InputRegister => {
                Ok(decode_response(byte_count, &response, start_addr + 30000))
            }
            ModbusFunction::HoldingRegister => {
                Ok(decode_response(byte_count, &response, start_addr + 40000))
            }
        }
    }
}
