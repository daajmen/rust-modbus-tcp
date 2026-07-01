use std::io::prelude::*;

#[derive(Debug, Clone, Default)]
pub enum FunctionCode {
    #[default]
    ReadCoil = 0x01,
    ReadDiscreteInputs = 0x02,
    ReadMultipleHoldingRegisters = 0x03,
    ReadInputRegisters = 0x04,
    WriteSingleCoil = 0x05,
    WriteSingleHoldingRegister = 0x06,
    WriteMultipleCoils = 0x15,
    WriteMultipleHolding = 0x16,
    MaskWriteRegister = 0x22,
    ReadWriteHoldRegisters = 0x23,
    ReadFIFOQueue = 0x24,
}

impl FunctionCode {
    pub fn get_function_code(code: u8) -> Option<FunctionCode> {
        match code {
            0x01 => Some(FunctionCode::ReadCoil),
            0x02 => Some(FunctionCode::ReadDiscreteInputs),
            0x03 => Some(FunctionCode::ReadMultipleHoldingRegisters),
            0x04 => Some(FunctionCode::ReadInputRegisters),
            0x05 => Some(FunctionCode::WriteSingleCoil),
            0x06 => Some(FunctionCode::WriteSingleHoldingRegister),
            0x15 => Some(FunctionCode::WriteMultipleCoils),
            0x16 => Some(FunctionCode::WriteMultipleHolding),
            0x22 => Some(FunctionCode::MaskWriteRegister),
            0x23 => Some(FunctionCode::ReadWriteHoldRegisters),
            0x24 => Some(FunctionCode::ReadFIFOQueue),
            _ => None,
        }
    }
}

pub struct PrimaryTables {
    pub discrete_input: Option<bool>,
    pub coil: Option<bool>,
    pub input_register: Option<u16>,
    pub holding_register: Option<u16>,
}

#[derive(Debug)]
pub enum ExceptionCodeTypes {
    /// Function code received in the query is not recognized or allowed by server
    IllegalFunction = 0x01,
    /// Data address of some or all the required entities are not allowed or do not exist in server
    IllegalDataAdress = 0x02,
    /// Value is not accepted by server
    IllegalDataValue = 0x03,
    /// Unrecoverable error occurred while server was attempting to perform requested action
    ServerDeviceFailure = 0x04,
    /// Server has accepted request and is processing it, but a long duration of time is required. This response is returned to prevent a timeout error from occurring in the client. client can next issue a Poll Program Complete message to determine whether processing is completed
    Acknowledge = 0x05,
    /// Server is engaged in processing a long-duration command; client should retry later
    ServerDeviceBusy = 0x06,
    /// Server cannot perform the programming functions; client should request diagnostic or error information from server
    NegativeAcknowledge = 0x07,
    /// Server detected a parity error in memory; client can retry the request
    MemoryParityError = 0x08,
    /// Specialized for Modbus gateways: indicates a misconfigured gateway
    GatewayPathUnavailable = 0x10,
    /// Specialized for Modbus gateways: sent when server fails to respond
    GatewayTargetDeviceFailedToRespond = 0x11,
}
#[derive(Debug, Default)]
pub struct ExceptionCode {
    pub exception_code: Option<ExceptionCodeTypes>,
    pub exception_message: Option<String>,
}

impl ExceptionCode {
    pub fn is_exception(&self, code: u8) -> bool {
        if code >= 0x80 { true } else { false }
    }

    pub fn get_exception_message(&mut self, code: u8) {
        match code {
            0x01 => {
                self.exception_code = Some(ExceptionCodeTypes::IllegalFunction);
                self.exception_message = Some(
                    "Function code received in the query is not recognized or allowed by server"
                        .to_string(),
                );
            }
            0x02 => {
                self.exception_code = Some(ExceptionCodeTypes::IllegalDataAdress);
                self.exception_message = Some(
                    "Data address of some or all the required entities are not allowed or do not exist in server"
                        .to_string(),
                );
            }
            0x03 => {
                self.exception_code = Some(ExceptionCodeTypes::IllegalDataValue);
                self.exception_message = Some("Value is not accepted by server".to_string());
            }
            0x04 => {
                self.exception_code = Some(ExceptionCodeTypes::ServerDeviceFailure);
                self.exception_message = Some(
                                "Unrecoverable error occurred while server was attempting to perform requested action"
                                    .to_string(),
                            );
            }
            0x05 => {
                self.exception_code = Some(ExceptionCodeTypes::Acknowledge);
                self.exception_message = Some(
                    "Server has accepted request and is processing it, but a long duration of time is required. This response is returned to prevent a timeout error from occurring in the client. client can next issue a Poll Program Complete message to determine whether processing is completed"
                                    .to_string(),
                            );
            }
            0x06 => {
                self.exception_code = Some(ExceptionCodeTypes::ServerDeviceBusy);
                self.exception_message = Some(
                                "Server is engaged in processing a long-duration command; client should retry later"
                                    .to_string(),
                            );
            }
            0x07 => {
                self.exception_code = Some(ExceptionCodeTypes::NegativeAcknowledge);
                self.exception_message = Some("Server cannot perform the programming functions; client should request diagnostic or error information from server".to_string());
            }
            0x08 => {
                self.exception_code = Some(ExceptionCodeTypes::MemoryParityError);
                self.exception_message = Some(
                    "Server detected a parity error in memory; client can retry the request"
                        .to_string(),
                );
            }
            0x10 => {
                self.exception_code = Some(ExceptionCodeTypes::GatewayPathUnavailable);
                self.exception_message = Some(
                    "Specialized for Modbus gateways: indicates a misconfigured gateway"
                        .to_string(),
                );
            }
            0x11 => {
                self.exception_code = Some(ExceptionCodeTypes::GatewayTargetDeviceFailedToRespond);
                self.exception_message = Some(
                    "Specialized for Modbus gateways: sent when server fails to respond"
                        .to_string(),
                );
            }
            _ => (),
        }
    }
}

#[derive(Debug, Default)]
pub struct ApplicationProtocolHeader {
    /// For synchronization between messages of server and client
    pub transaction_identifier: [u8; 2],
    /// 0 for Modbus/TCP
    pub protocol_identifiter: [u8; 2],
    /// Number of remaining bytes in this frame
    pub length_field: [u8; 2],
    /// Server address (255 if not used), treated like slave address in Modbus over Serial line
    pub unit_identifier: u8,
}

#[derive(Debug, Default)]
pub struct ModbusPDU {
    pub function_code: FunctionCode,
    pub data: [u16; 2],
}

#[derive(Debug, Default)]
pub struct Frame {
    pub application_header: ApplicationProtocolHeader,
    pub modbus_pdu: ModbusPDU,
}

impl Frame {
    pub fn as_vec(&self) -> Vec<u8> {
        let mut data = Vec::new();
        let size = &self.convert_length_field().to_be_bytes();
        let register = &self.modbus_pdu.data[0].to_be_bytes();
        let quantity = &self.modbus_pdu.data[1].to_be_bytes();

        data.extend_from_slice(&self.application_header.transaction_identifier);
        data.extend_from_slice(&self.application_header.protocol_identifiter);
        data.extend_from_slice(size);
        data.push(self.application_header.unit_identifier);
        data.push(self.modbus_pdu.function_code.clone() as u8);

        data.extend_from_slice(register);
        data.extend_from_slice(quantity);

        data
    }
    fn convert_length_field(&self) -> u16 {
        let register = self.modbus_pdu.data[0].to_be_bytes();
        let quantity = self.modbus_pdu.data[1].to_be_bytes();
        let code = self.modbus_pdu.function_code.clone() as u8;
        let field = [
            self.application_header.unit_identifier,
            code,
            register[0],
            register[1],
            quantity[0],
            quantity[1],
        ];

        field.len() as u16
    }
}
#[derive(Debug)]
pub struct ResponseData {
    pub function_code: Option<FunctionCode>,
    pub exception: Option<ExceptionCode>,
    pub modbus_data: Option<Vec<u8>>,
}
