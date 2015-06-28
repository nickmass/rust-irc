use std::io::prelude::*;
use irc::IrcMessage;
use irc::COMMAND_BUF_SIZE;

pub struct IrcReader<T: Read> {
    stream: T,
}

impl<T: Read> IrcReader<T> {
    pub fn new(stream: T) -> IrcReader<T> {
        IrcReader {
            stream: stream
        }
    }
}

impl<T: Read> Iterator for IrcReader<T> {
    type Item = IrcMessage;
    fn next(&mut self) -> Option<IrcMessage> {
        let mut command_buf: [u8; COMMAND_BUF_SIZE] = [0; COMMAND_BUF_SIZE];
        let mut buf_ind = 0;
        let mut command_end = false;
        let mut read_byte: [u8; 1] = [0; 1];
        loop {
            let read_res = self.stream.read(&mut read_byte);
            if read_res.is_err() {
                break;
            }
            if read_res.unwrap() == 0 {
                break;
            }
            let next_char = read_byte[0];
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

