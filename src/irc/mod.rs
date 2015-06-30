pub use self::message::IrcMessage;
pub use self::reader::IrcReader;
pub use self::writer::IrcWriter;
pub use self::connection::IrcConnection;
pub use self::connection::IrcConnectionOptions;

pub const COMMAND_BUF_SIZE: usize = 4096;

mod message;
mod reader;
mod writer;
mod connection;
