use std::net::TcpStream;
use std::io;
use std::io::prelude::*;
use irc::IrcMessage;
use irc::COMMAND_BUF_SIZE;

pub struct IrcReader {
    stream: TcpStream,

}

impl IrcReader {
    pub fn new(base_stream: &TcpStream) -> io::Result<IrcReader> {
        let stream = try!(base_stream.try_clone());
        Ok(IrcReader {
            stream: stream,
        })
    }
}

impl Iterator for IrcReader {
    type Item = IrcMessage;
    fn next(&mut self) -> Option<IrcMessage> {
        let mut command_buf: [u8; COMMAND_BUF_SIZE] = [0; COMMAND_BUF_SIZE];
        let mut buf_ind = 0;
        let mut command_end = false;
        let stream = &self.stream;
        let bytes = stream.bytes();
        for x in bytes {
            if x.is_err() {
                return None;
            }
            let next_char = x.unwrap();
            command_buf[buf_ind] = next_char;
            buf_ind = buf_ind + 1;
            if command_end && next_char == 10 {
                return Some(IrcMessage::new(command_buf, buf_ind - 2));
            } else if next_char == 13 {
                command_end = true;
            } else {
                command_end = false;
            }

            if buf_ind >= COMMAND_BUF_SIZE {
                buf_ind = 0;
            }
        }
        None
    }
}
