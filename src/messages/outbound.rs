use serde::{Deserialize, Serialize};

/// An Outbound Alchemy Websocket Message
///
/// ## Example
///
/// The websocket message should serialize into a json string like:
/// ```json
/// {
///     "id": 1,
///     "method": "eth_subscribe",
///     "params": [
///         "alchemy_pendingTransactions",
///         {
///             "toAddress": "00000000219ab540356cBB839Cbe05303d7705Fa"
///         }
///     ]
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct AlchemySocketMessage {
    /// The message id
    pub id: u64,
    /// The message method
    pub method: OutSocketMethod,
    /// The message params
    pub params: Vec<serde_json::Value>,
}

/// An Alchemy Websocket Message Method
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum OutSocketMethod {
    /// Subscribe to a websocket channel
    #[serde(rename = "eth_subscribe")]
    Subscribe,
    /// Unsubscribe from a websocket channel
    #[serde(rename = "eth_unsubscribe")]
    Unsubscribe,
}
