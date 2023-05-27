#[derive(Debug, Clone)]
pub struct MessageError;

impl std::fmt::Display for MessageError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "invalid message")
    }
}
