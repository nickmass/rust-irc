use std::net::TcpStream;
use std::io;
use std::io::prelude::*;
use std::thread;
use std::str;

const COMMAND_BUF_SIZE: usize = 4096;

struct IrcMessage {
	msg: [u8; COMMAND_BUF_SIZE],
	length: usize,
}

struct IrcReader {
	stream: TcpStream,

}

impl IrcReader {
	pub fn new(base_stream: &TcpStream) -> std::io::Result<IrcReader> {
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
		let mut bytes = stream.bytes();
		loop {
			match bytes.next() {
				Some(x) => {
					if x.is_err() {
						return None;
					}
					let next_char = x.unwrap();
					command_buf[buf_ind] = next_char;
					buf_ind = buf_ind + 1;
					if command_end && next_char == 10 {
						return Some(IrcMessage { msg: command_buf, length: buf_ind - 2 });
					} else if next_char == 102 {
						command_end = true;
					} else {
						command_end = false;
					}

					if buf_ind >= COMMAND_BUF_SIZE {
						buf_ind = 0;
					}

				},
				None => {
					return None;
				}
			}
		}
	}
}

struct IrcWriter {
	stream: TcpStream,
}

impl IrcWriter {
	pub fn new(base_stream: &TcpStream) -> std::io::Result<IrcWriter> {
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
		let mut stdin = io::stdin();
		let mut buf = String::new();
		loop {
			let _= stdin.read_line(&mut buf).unwrap();
			writer.write(&buf.as_bytes());
		}
	});
	
	for irc_msg in reader {
		parse_command(&irc_msg.msg[0..irc_msg.length]);
	}
}

fn parse_command(command: &[u8]) {
	let command_str = str::from_utf8(command);
	if command_str.is_err() {
		return;
	}
	println!("{}", command_str.unwrap());
}
