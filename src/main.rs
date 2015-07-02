use irc::IrcConnection;
use irc::IrcConnectionOptions;

extern crate openssl;

mod irc;

fn main() {
    let irc_connection = IrcConnection::new(IrcConnectionOptions::default()).unwrap();
    irc_connection.start();
    loop {
    }
}
