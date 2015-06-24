use std::net::TcpStream;
use std::io;
use std::io::prelude::*;

pub struct IrcWriter {
    stream: TcpStream,
}

impl IrcWriter {
    pub fn new(base_stream: &TcpStream) -> io::Result<IrcWriter> {
        let stream = try!(base_stream.try_clone());
        Ok(IrcWriter {
            stream: stream,
        })
    }

    pub fn write(&mut self, bytes: &[u8]) {
        let _ = &self.stream.write(&bytes);
    }
}
