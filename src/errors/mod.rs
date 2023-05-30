/// Represents the errors that can occur when using the client / server protocols.
///
/// `MessageError` is used to descibe the different errors that can occur when parsing
/// requests / responses received from clients when using the `net` feature.
///
/// # Variants
///
/// * `InvalidFormat`: The request / response format received is not recognised
///   * Can occur if the data received is malformed in some way.
/// * `InvalidMessage`: The parsed message failed the validation checks for the type
/// of operation used
///
#[cfg(feature = "net")]
#[derive(Debug, Clone, PartialEq)]
pub enum MessageError {
    InvalidFormat,
    InvalidMessage,
}
