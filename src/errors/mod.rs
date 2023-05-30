#[derive(Debug, Clone, PartialEq)]
pub enum MessageError {
    InvalidLength,
    InvalidFormat,
    InvalidMessage,
}
