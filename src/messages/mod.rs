//! Alchemy Websocket Messages

/// Outbound requests
pub mod outbound;

/// Inbound responses
pub mod inbound;

/// A prelude to re-export commonly used types
pub mod prelude {
    pub use super::{inbound::*, outbound::*};
}
