use std::net::TcpStream;
use std::io;
use std::io::prelude::*;
use std::thread;
use std::str;

const COMMAND_BUF_SIZE: usize = 4096;

struct IrcMessage {
	message: [u8; COMMAND_BUF_SIZE],
	length: usize,
    has_prefix: bool,
    command_index: usize,
    param_index: usize,
    trail_index: usize,
}

impl IrcMessage {
    pub fn new(message: [u8; COMMAND_BUF_SIZE], length: usize) -> IrcMessage {
        let has_prefix = message[0] == b':';

        let mut command_index = 0;
        if has_prefix {
            for x in 1 .. length {
                if message[x] == b' ' {
                    command_index = x + 1;
                    break;
                }
            }
        }

        let mut param_index = 0;
        for x in command_index .. length {
            if message[x] == b' ' {
                param_index = x + 1;
                break;
            }
        }

        let mut trail_index = 0;
        for x in param_index .. length {
            if message[x] == b':' {
                trail_index = x + 1;
                break;
            }
        }

        let new_message = IrcMessage { 
            message: message,
            length: length,
            has_prefix: has_prefix,
            command_index: command_index,
            param_index: param_index,
            trail_index: trail_index,
        };

        new_message
    }

    pub fn prefix(&self) -> Option<&[u8]> {
        if self.has_prefix {
            return Some(&self.message[1..self.command_index - 1]);
        }
        None
    }

    pub fn command(&self) -> &[u8] {
        &self.message[self.command_index..self.param_index - 1]
    }

    pub fn param_at(&self, index: u32) -> Option<&[u8]> {
        if self.param_index == 0 {
            return None;
        }
        
        let mut last_param_index = self.param_index;
        let mut param_count = 0;
        let mut x = last_param_index;
        while x < self.length && (self.trail_index == 0 || x < self.trail_index) {
            if self.message[x] == b' '  {
                if param_count == index {
                    return Some(&self.message[last_param_index..x]);
                }
                
                param_count = param_count + 1;
                last_param_index = x + 1;
            }
            x = x + 1;
        }
        None
    }

    pub fn trailing(&self) -> Option<&[u8]> {
        if self.trail_index == 0 {
            return None;
        }
        Some(&self.message[self.trail_index..self.length])
    }
}

#[cfg(test)]
mod tests {
    const COMMAND_BUF_SIZE: usize = 4096;
    use std::str; 
    #[test]
    fn parse_message() {
        let msg = super::IrcMessage::new(load_message(b":syrk!kalt@millennium.stealth.net QUIT :Gone to have lunch"), 58);
        assert_eq!("syrk!kalt@millennium.stealth.net", str::from_utf8(msg.prefix().unwrap()).unwrap());
        assert_eq!("QUIT", str::from_utf8(msg.command()).unwrap());
        assert_eq!(None, msg.param_at(0));
        assert_eq!(None, msg.param_at(1));
        assert_eq!("Gone to have lunch", str::from_utf8(msg.trailing().unwrap()).unwrap());

        let msg_two = super::IrcMessage::new(load_message(b"SERVICE dict * *.fr 0 0 :French Dictionary"),
            "SERVICE dict * *.fr 0 0 :French Dictionary".len());
        assert_eq!(None, msg_two.prefix());
        assert_eq!("SERVICE", str::from_utf8(msg_two.command()).unwrap());
        assert_eq!("dict", str::from_utf8(msg_two.param_at(0).unwrap()).unwrap());
        assert_eq!("*", str::from_utf8(msg_two.param_at(1).unwrap()).unwrap());
        assert_eq!("*.fr", str::from_utf8(msg_two.param_at(2).unwrap()).unwrap());
        assert_eq!("0", str::from_utf8(msg_two.param_at(3).unwrap()).unwrap());
        assert_eq!("0", str::from_utf8(msg_two.param_at(4).unwrap()).unwrap());
        assert_eq!(None, msg_two.param_at(5));
        assert_eq!("French Dictionary", str::from_utf8(msg_two.trailing().unwrap()).unwrap());
    }

    fn load_message(msg_str: &[u8]) -> [u8; COMMAND_BUF_SIZE] {
        let mut buf: [u8; COMMAND_BUF_SIZE] = [0; COMMAND_BUF_SIZE];

        for x in 0 .. msg_str.len() {
            buf[x] = msg_str[x];
        }

        buf
    }
}

struct IrcReader {
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

struct IrcWriter {
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

fn main() {
	let connection_result = TcpStream::connect("127.0.0.1:6667");
	if connection_result.is_err() {
		return;
	}
	
	let connection = connection_result.unwrap();	
	let reader = IrcReader::new(&connection).unwrap();
	let mut writer = IrcWriter::new(&connection).unwrap();
	
	thread::spawn(move || {
		let stdin = io::stdin();
		let stdin_lock = stdin.lock();
		for in_line in stdin_lock.lines() {
			let _ = in_line.and_then(|x| {
				writer.write(x.as_bytes());
				writer.write(b"\n");
				Ok(x)
			});
		}
	});
	
	for irc_msg in reader {
		parse_command(&irc_msg.message[0..irc_msg.length]);
	}
}

fn parse_command(command: &[u8]) {
	let command_str = str::from_utf8(command);
	if command_str.is_err() {
		return;
	}
	println!("{}", command_str.unwrap());
}
