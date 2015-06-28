use std::io::prelude::*;

pub struct IrcWriter<T: Write> {
    stream: T,
}

impl<T: Write> IrcWriter<T> {
    pub fn new(stream: T) -> IrcWriter<T> {
        IrcWriter {
            stream: stream,
        }
    }

    pub fn write(&mut self, bytes: &[u8]) {
        let _ = &self.stream.write(&bytes);
    }
}
