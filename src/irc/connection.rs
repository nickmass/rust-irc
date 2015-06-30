use irc::IrcReader;
use irc::IrcWriter;
use irc::IrcMessage;
use std::str;
use std::io;
use std::io::prelude::*;
use std::net::TcpStream;
use std::thread;
use openssl::ssl::*;

#[derive(Debug)]
pub enum IrcError {
    Io(io::Error),
    Ssl(error::SslError),
}

impl From<io::Error> for IrcError {
    fn from(err: io::Error) -> IrcError {
        IrcError::Io(err)
    }
}

impl From<error::SslError> for IrcError {
    fn from(err: error::SslError) -> IrcError {
        IrcError::Ssl(err)
    }
}

pub struct IrcConnectionOptions {
    host: String,
    port: i32,
    use_ssl: bool,
}

impl Default for IrcConnectionOptions {
    fn default() -> IrcConnectionOptions {
        IrcConnectionOptions {
            host: format!("{}", "localhost"),
            port: 6697,
            use_ssl:  true,
        }
    }
}

pub struct IrcConnection {
    options: IrcConnectionOptions,
    reader: IrcReader<MaybeSslStream<TcpStream>>,
    writer: IrcWriter<MaybeSslStream<TcpStream>>,
}

impl IrcConnection {
    pub fn new(options: IrcConnectionOptions) -> Result<IrcConnection, IrcError> {
        let stream = try!(TcpStream::connect(str::from_utf8(format!("{}:{}", options.host, options.port).as_bytes()).unwrap()));
        if options.use_ssl {
            let ssl_context = try!(SslContext::new(SslMethod::Sslv23));
            let ssl_stream = try!(SslStream::new(&ssl_context, stream));
            let reader_connection = try!(ssl_stream.try_clone());
            let reader = IrcReader::new(MaybeSslStream::Ssl(reader_connection));
            let writer_connection = try!(ssl_stream.try_clone());
            let writer = IrcWriter::new(MaybeSslStream::Ssl(writer_connection));
            return Ok(IrcConnection {
                options: options,
                reader: reader,
                writer: writer,
            });
        } else {
            let reader_connection = try!(stream.try_clone());
            let reader = IrcReader::new(MaybeSslStream::Normal(reader_connection));
            let writer_connection = try!(stream.try_clone());
            let writer = IrcWriter::new(MaybeSslStream::Normal(writer_connection));
            return Ok(IrcConnection {
                options: options,
                reader: reader,
                writer: writer,
            });
        }
    }

    pub fn start(self) {
        let mut writer = self.writer;
        thread::spawn(move || {
            let stdin = io::stdin();
            let stdin_lock = stdin.lock();
            for in_line in stdin_lock.lines() {
                let _ = in_line.and_then(|x| {
                    match str::from_utf8(x.as_bytes()).unwrap() {
                        "user" => {
                            writer.send(&IrcMessage::test_user());
                            writer.send(&IrcMessage::test_nick());
                        },
                        _ => {
                            writer.write(x.as_bytes());
                            writer.write(b"\r\n");
                        }
                    }
                    Ok(x)
                });
            }
        });

        for irc_msg in self.reader {
            parse_command(&irc_msg.message());
        }
    }
}

fn parse_command(command: &[u8]) {
    let command_str = str::from_utf8(command);
    if command_str.is_err() {
        return;
    }
    println!("{}", command_str.unwrap());
}
