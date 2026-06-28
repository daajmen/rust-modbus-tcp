use std::io::Result;
use std::net::TcpStream;

pub fn connect(adress: String, port: String) -> Result<TcpStream> {
    let connection_info = format!("{}:{}", adress, port);

    match TcpStream::connect(connection_info) {
        Ok(stream) => Ok(stream),
        Err(e) => Err(e),
    }
}
