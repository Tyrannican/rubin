#[derive(Debug, Clone, PartialEq)]
pub enum Operation {
    StringSet,
    StringGet,
    Noop,
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
            Self::Noop => write!(f, "NOOP"),
        }
    }
}

pub struct Message {
    pub op: Operation,
    pub args: Vec<String>,
}

pub fn create_request(op_code: Operation, args: Vec<String>) -> String {
    format!("{}::{}", op_code, args.join(" "))
}

pub fn parse_request(req: &str) -> Message {
    let r_split = req
        .split("::")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let op = Operation::from_str(&r_split[0]);
    let args = r_split[1..].to_vec();

    Message { op, args }
}
