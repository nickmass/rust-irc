use std::io::prelude::*;
use irc::IrcMessage;

pub struct IrcWriter<T: Write> {
    stream: T,
}

impl<T: Write> IrcWriter<T> {
    pub fn new(stream: T) -> IrcWriter<T> {
        IrcWriter {
            stream: stream,
        }
    }

    pub fn send(&mut self, message: &IrcMessage) {
        let _ = &self.stream.write(&message.message());
        let _ = &self.stream.write(b"\r\n");
    }

    pub fn write(&mut self, bytes: &[u8]) {
        let _ = &self.stream.write(&bytes);
    }
}
