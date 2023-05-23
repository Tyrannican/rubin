use crate::errors::MessageLengthError;

#[derive(Debug, Clone)]
pub enum OpCode {
    SSet,
    SGet,
    Noop,
}

impl OpCode {
    pub fn new(code: &str) -> Self {
        match code.to_uppercase().as_str() {
            "SSET" => Self::SSet,
            "SGET" => Self::SGet,
            _ => Self::Noop,
        }
    }
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::SSet => write!(f, "SSET"),
            OpCode::SGet => write!(f, "SGET"),
            OpCode::Noop => write!(f, "NOOP"),
        }
    }
}

pub struct Message {
    pub op: OpCode,
    pub contents: Vec<String>,
}

impl Message {
    pub fn new(msg: &str) -> Result<Self, MessageLengthError> {
        let msg_split = msg
            .split(' ')
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        if msg_split.len() < 2 {
            return Err(MessageLengthError);
        }

        let rest = msg_split[1..].to_vec();
        let op_code = OpCode::new(&msg_split[0]);

        Ok(Self {
            op: op_code,
            contents: rest,
        })
    }
}
