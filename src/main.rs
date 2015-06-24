use std::net::TcpStream;
use std::io;
use std::io::prelude::*;
use std::thread;
use std::str;
use irc::IrcReader;
use irc::IrcWriter;

mod irc;

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
		parse_command(&irc_msg.message());
	}
}

fn parse_command(command: &[u8]) {
	let command_str = str::from_utf8(command);
	if command_str.is_err() {
		return;
	}
	println!("{}", command_str.unwrap());
}
