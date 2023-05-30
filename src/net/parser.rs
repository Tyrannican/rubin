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

#[derive(Debug, Clone, PartialEq)]
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
                if self.args.len() == 2 {
                    valid = true;
                }
            }
            // Should have ONE entry - a key
            Operation::StringGet => {
                if self.args.len() == 1 {
                    valid = true;
                }
            }
            Operation::Noop => valid = true,
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

    if r_split.len() < 2 {
        return Err(MessageError::InvalidFormat);
    }

    let op = Operation::from_str(&r_split[0]);
    let args = r_split[1]
        .split(' ')
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let msg = Message { op, args };

    if !msg.validate() {
        return Err(MessageError::InvalidMessage);
    }

    Ok(msg)
}

pub fn parse_response(msg: &str) -> String {
    let resp = msg.split("::").collect::<Vec<&str>>();
    if resp.len() < 2 {
        return String::from("");
    }

    return resp[1].trim().to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_appropriate_operation() {
        let op_codes = vec!["SET", "GET", "SOMETHING"];
        for op in op_codes {
            let code: Operation = Operation::from_str(op);

            match op {
                "SET" => assert!(code == Operation::StringSet),
                "GET" => assert!(code == Operation::StringGet),
                _ => assert!(code == Operation::Noop),
            }
        }
    }

    #[test]
    fn validation_string_set_message() {
        let mut m = Message {
            op: Operation::StringSet,
            args: vec!["arg1".to_string(), "arg2".to_string()],
        };

        assert!(m.validate());

        m.args.pop();
        assert!(!m.validate());
    }

    #[test]
    fn validation_string_get_message() {
        let mut m = Message {
            op: Operation::StringGet,
            args: vec!["arg1".to_string()],
        };

        assert!(m.validate());

        m.args.push("arg2".to_string());
        assert!(!m.validate());
    }

    #[test]
    fn validation_noop_message() {
        let m = Message {
            op: Operation::Noop,
            args: vec![],
        };

        assert!(m.validate());
    }

    #[test]
    fn create_appropriate_request() {
        let ops = vec![Operation::StringSet, Operation::StringGet];
        for op in ops {
            let args = match op {
                Operation::StringSet => vec!["arg1".to_string(), "arg2".to_string()],
                Operation::StringGet => vec!["arg1".to_string()],
                _ => vec![],
            };
            let expected = format!("{}::{}", op, args.join(" "));
            let request = create_request(op, args);
            assert_eq!(expected, request);
        }
    }

    #[test]
    fn parse_requests_correctly() {
        let request = "SET::arg1 arg2";
        let result = parse_request(request).unwrap();

        let expected = Message {
            op: Operation::StringSet,
            args: vec!["arg1".to_string(), "arg2".to_string()],
        };

        assert_eq!(result, expected);
    }

    #[test]
    fn detects_an_invalid_message() {
        let request = "SET::arg1";
        let result = parse_request(request).unwrap_err();
        assert_eq!(result, MessageError::InvalidMessage);
    }

    #[test]
    fn parse_invalid_requests() {
        let request = "SGET:argumetns blahg";
        let result = parse_request(request).unwrap_err();
        assert_eq!(result, MessageError::InvalidFormat);
    }

    #[test]
    fn parses_a_valid_response() {
        let response = "SET::OK";
        let result = parse_response(response);

        assert_eq!(&result, "OK");
    }

    #[test]
    fn parses_an_invalid_response_correctly() {
        let response = "GET:";
        let result = parse_response(response);

        assert_eq!(&result, "");
    }
}
