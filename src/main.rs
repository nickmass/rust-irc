use std::net::TcpStream;
use std::io::prelude::*;
use std::str;

fn main() {
	let mut connection = TcpStream::connect("127.0.0.1:6667").unwrap();
	let _ = connection.write(b"Hello World\n");
	let mut bytes = connection.bytes();
	const COMMAND_BUF_SIZE: usize = 4096;
	let mut command_buf: [u8; COMMAND_BUF_SIZE] = [0; COMMAND_BUF_SIZE];
	let mut buf_ind = 0;
	let mut command_end = false;
	loop {
		match bytes.next() {
			Some(x) => {
				let next_char = x.unwrap();
				command_buf[buf_ind] = next_char;
				buf_ind = buf_ind + 1;
				if command_end && next_char == 10 {
					parse_command(&command_buf[0..(buf_ind-2)]);
					buf_ind = 0;
					command_end = false;
				} else if next_char == 102 {
					command_end = true;
				} else {
					command_end = false;
				}
				if buf_ind >= COMMAND_BUF_SIZE {
					buf_ind = 0;
				}
			},
			None => break,
		}
	}
}

fn parse_command(command: &[u8]) {
	println!("{}", str::from_utf8(command).unwrap());
}
