use irc::IrcConnection;
use irc::IrcConnectionOptions;
use irc::IrcMessage;
use std::str;
use std::io;
use std::io::prelude::*;
use std::thread;

extern crate openssl;

mod irc;

fn main() {
    let irc_connection = IrcConnection::new(IrcConnectionOptions::default()).unwrap();
    let (tx, rx) = irc_connection.start();
    
    thread::spawn(move || {
        for msg in rx {
            parse_command(&msg.message());
        }
    });
    
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();
    for in_line in stdin_lock.lines() {
        let _ = in_line.and_then(|x| {
            match x.as_bytes() {
                b"user" => {
                    tx.send(IrcMessage::test_user());
                    tx.send(IrcMessage::test_nick());
                },
                _ => {
                }
            }
            Ok(x)
        });
    }
}

fn parse_command(command: &[u8]) {
    let command_str = str::from_utf8(command);
    if command_str.is_err() {
        return;
    }
    println!("{}", command_str.unwrap());
}
