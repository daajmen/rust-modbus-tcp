use std::net::TcpStream;


pub fn connect_modbus(addr: &str) -> std::io::Result<TcpStream> {
    return TcpStream::connect(addr); 
}