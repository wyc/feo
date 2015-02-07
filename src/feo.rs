#![feature(phase)]
#[phase(plugin, link)] extern crate log;
extern crate std;

use std::option::Option;
use std::io::{TcpStream, IoResult};
use std::str;

static BUF_SIZE: uint = 512;

fn main() {

    println!("what's up");
    let mut stream = TcpStream::connect("irc.freenode.net", 6667);
    //try!(stream.write(data_get.as_bytes()));
    //try!(stream.write(data_headers.as_bytes()));
    loop {
        let mut buf = [0, ..BUF_SIZE];
        stream.read(buf);
        match str::from_utf8(buf) {
            Some(s) => (),
            None => error!("invalid input!")
        }
    }
}

