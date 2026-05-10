use modbus::modbus_client::ModbusMaster;

use crate::modbus::modbus_client::ModbusFunction;
mod modbus; 


fn main() -> std::io::Result<()> { 

    let ip_adress = "127.0.0.1:502";
    let slave_id = 1;  
    let start_adress = 0;
    let quantity = 6; 

    
    let mut master = ModbusMaster::new(ip_adress);

    // Connect to server
    master.connect();
    let md_coil = master.read_modbus_register(
        ModbusFunction::ReadCoilRegister,
        slave_id,
        start_adress,
        quantity);

    println!("{:?}", md_coil);

    let mb_inputstatus = master.read_modbus_register(
        ModbusFunction::ReadInputStatusRegister,
        slave_id,
        start_adress,
        quantity);

    println!("{:?}", mb_inputstatus);

    let mb_inputreg = master.read_modbus_register(
        ModbusFunction::ReadInputRegister,
        slave_id,
        start_adress,
        quantity);

    println!("{:?}", mb_inputreg);

    let mb_holding = master.read_modbus_register(
        ModbusFunction::ReadHoldingRegister,
        slave_id,
        start_adress,
        quantity);

    println!("{:?}", mb_holding);


    Ok(())

}
