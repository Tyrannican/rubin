#[derive(Debug, Clone)]
pub struct MessageLengthError;

impl std::fmt::Display for MessageLengthError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid message length")
    }
}
