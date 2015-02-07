extern crate std;

use std::io::{TcpStream, IoResult, IoError};

struct Connection {
    stream: TcpStream,
}

enum HandleError {
    IoError
}

fn pong(stream: TcpStream, msg: String) -> Result<(),HandleError> {
    
}

