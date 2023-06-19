//! Errors that can occur using the lib
//!
//! `MessageError` is used to descibe the different errors that can occur when parsing
//! requests / responses received from clients when using the `net` feature.
//!
//! # Variants
//!
//! * `InvalidFormat`: The request / response format received is not recognised
//!   * Can occur if the data received is malformed in some way.
//! * `InvalidMessage`: The parsed message failed the validation checks for the type
//! of operation used

/// Errors for messages sent using the client / server protocol
#[derive(Debug, Clone, PartialEq)]
pub enum MessageError {
    /// Format of the message is not what was expected by the server
    InvalidFormat(String),

    /// Constructed message  has failed the validation checks
    InvalidMessage(String),
}
