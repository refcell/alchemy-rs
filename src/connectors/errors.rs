use ethers::providers::ProviderError;
use soketto::handshake;

/// An Alchemy Websocket Connection Error
#[derive(Debug)]
pub enum AlchemyConnectionError {
    /// A Provider Connection Error
    ProviderError(ProviderError),
    /// Raw Websocket TcpStream Error
    RawStreamError(std::io::Error),
    /// A Raw Websocket Connection Error
    RawSocketError(soketto::handshake::Error),
    /// A Raw Websocket Handshake Error
    RawHandshakeError(handshake::Error),
    /// Deserialization Error
    Deserialization(serde_json::Error),
    /// Received an unexpected response type
    UnexpectedResponseType,
    /// Missing the websocket channel sender
    MissingSender,
    /// Missing the websocket channel receiver
    MissingReceiver,
    /// Connection Closed
    Closed,
    /// No websocket connection established yet
    MissingConnection,
    /// Sending a message to the websocket channel failed
    SendFailed(soketto::connection::Error),
    /// Flushing the websocket channel failed
    FlushFailed(soketto::connection::Error),

    /// Some soketto websocket error
    SomeError(soketto::connection::Error),
    /// The method is unimplemented
    Unimplemented,
    /// The text response could not be parsed as a string
    InvalidTextString,
}
