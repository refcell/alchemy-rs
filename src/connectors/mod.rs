//! Alchemy Connectors

/// An ethers provider connector
pub mod provider;

/// A raw websocket connector
pub mod raw;

/// Common Errors
pub mod errors;

/// Re-export a prelude
pub mod prelude {
    pub use super::{errors::*, provider::*, raw::*, *};
}

/// An alchemy api connection manager
#[derive(Debug)]
pub enum AlchemyConnector {
    /// An ethers-rs websocket [Provider](ethers::providers::Provider) for alchemy
    Provider(Option<provider::EthersWsProvider>),
    /// A Raw, Persistent Websocket Connection to the Alchemy API using [soketto](https://docs.rs/soketto/latest/soketto/)
    Raw(Option<raw::RawAlchemyConnection>),
}

/// The type of alchemy api websocket connection
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlchemyConnectorType {
    /// An ethers-rs websocket [Provider](ethers::providers::Provider) for alchemy
    Provider,
    /// A Raw, Persistent Websocket Connection to the Alchemy API using [soketto](https://docs.rs/soketto/latest/soketto/)
    Raw,
}

impl Default for AlchemyConnectorType {
    fn default() -> Self {
        AlchemyConnectorType::Raw
    }
}

impl From<AlchemyConnectorType> for AlchemyConnector {
    fn from(t: AlchemyConnectorType) -> Self {
        match t {
            AlchemyConnectorType::Provider => AlchemyConnector::Provider(None),
            AlchemyConnectorType::Raw => AlchemyConnector::Raw(None),
        }
    }
}
