use irc::COMMAND_BUF_SIZE;

pub struct IrcMessage {
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
    pub fn message(&self) -> &[u8] {
        &self.message[0..self.length]
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
