use crate::errors::MessageError;

#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    StringSet,
    StringGet,
    Noop,
    Error,
}

impl Operation {
    pub fn from_str(op: &str) -> Self {
        match op {
            "SET" => Self::StringSet,
            "GET" => Self::StringGet,
            _ => Self::Noop,
        }
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StringSet => write!(f, "SET"),
            Self::StringGet => write!(f, "GET"),
            Self::Error => write!(f, "ERR"),
            Self::Noop => write!(f, "NOOP"),
        }
    }
}

pub struct Message {
    pub op: Operation,
    pub args: Vec<String>,
}

impl Message {
    pub fn validate(&self) -> bool {
        let mut valid = false;

        match self.op {
            // Should have TWO entries - ONE key and ONE value
            Operation::StringSet => {
                debug_assert!(self.args.len() == 2);
                if self.args.len() == 2 {
                    valid = true;
                }
            }
            // Should have ONE entry - a key
            Operation::StringGet => {
                debug_assert!(self.args.len() == 1);
                if self.args.len() == 1 {
                    valid = true;
                }
            }
            _ => {}
        }

        valid
    }
}

pub fn create_request(op_code: Operation, args: Vec<String>) -> String {
    format!("{}::{}", op_code, args.join(" "))
}

pub fn parse_request(req: &str) -> Result<Message, MessageError> {
    let r_split = req
        .split("::")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let op = Operation::from_str(&r_split[0]);
    let args = r_split[1]
        .split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let msg = Message { op, args };

    if !msg.validate() {
        return Err(MessageError);
    }

    Ok(msg)
}

pub fn parse_response(msg: &str) -> String {
    let resp = msg.split("::").collect::<Vec<&str>>();
    if resp.len() < 2 {
        return String::from("");
    }

    return resp[1].to_string();
}
