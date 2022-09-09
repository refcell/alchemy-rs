use ethers::types::Transaction;
use serde::{Deserialize, Serialize};

/// An `eth_subscription` message
pub type EthSubscription = String;

/// The json rpc version
pub type JsonRpc = String;

/// Alchemy Subscription Message Result
///
/// ## Example
///
/// After sending an eth_subscribe to the alchemy websocket, we should expect a json-stringified response like below.
///
/// ```json
/// {
///     "id": 1,
///     "result": "0xa79a6df98fb2a42516b5aca3177fbb6c",
///     "jsonrpc": "2.0"
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct AlchemySubscriptionMessageResult {
    /// The message id
    pub id: u64,
    /// The message result
    #[serde(
        serialize_with = "serialize_uuid_simple",
        deserialize_with = "deserialize_uuid_simple"
    )]
    pub result: uuid::Uuid,
    /// The message jsonrpc
    pub jsonrpc: JsonRpc,
}

pub(crate) fn serialize_uuid_simple<S>(uuid: &uuid::Uuid, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    s.serialize_str(&format!("0x{}", &uuid.as_simple()))
}

pub(crate) fn deserialize_uuid_simple<'de, D>(deserializer: D) -> Result<uuid::Uuid, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match serde_json::Value::deserialize(deserializer) {
        Ok(serde_json::Value::String(s)) => {
            uuid::Uuid::parse_str(&s.replace("0x", "")).map_err(serde::de::Error::custom)
        }
        Err(e) => Err(e),
        _ => Err(serde::de::Error::custom(
            "Deserialized invalid serde_json::Value from uuid",
        )),
    }
}

/// An Alchemy Websocket Message Response
///
/// ## Example
///
/// After receiving an [AlchemySubscriptionMessageResult](crate::messages::inbound::AlchemySubscriptionMessageResult),
/// we should expect to receive messages with a method string of `eth_subscription`.
///
/// ```json
/// {
///     "jsonrpc": "2.0",
///     "method": "eth_subscription",
///     "params": {
///         ...
///     }
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct AlchemySocketMessageResponse {
    /// The message jsonrpc
    pub jsonrpc: JsonRpc,
    /// The message method
    pub method: EthSubscription,
    /// The inner message result
    pub result: AlchemyInnerResponse,
}

/// An inner Alchemy Websocket Message Response
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum AlchemyInnerResponse {
    /// An `alchemy_pendingTransactions` message
    PendingTransactionResult(PendingTransactionResult),
}

/// An `alchemy_pendingTransactions` message result
///
/// ## Example
///
/// ```json
/// {
///     "result": {
///         "hash": "0xf3207c10a9b9e09b4b51d5c783a1ba85b632055d06100c51f2c1a331dc293d65",
///         "nonce": "0x47",
///         "blockHash": null,
///         "blockNumber": null,
///         "transactionIndex": null,
///         "from": "0xe2ca13527f5accdcdb571a7004a0324e6a36ee6f",
///         "to": "0xdac17f958d2ee523a2206206994597c13d831ec7",
///         "value": "0x0",
///         "gasPrice": "0x59682f000",
///         "gas": "0x11170",
///         "input": "0xa9059cbb000000000000000000000000a02462b8e950cb7ba26f69a6862069231eeb5da10000000000000000000000000000000000000000000000000000000002faf080",
///         "v": "0x0",
///         "r": "0x58abb3787d50b4bd6e4969d08136780eade64971b3a1c24a38b84cd0da52c3fb",
///         "s": "0x4c9ff3765093a1373e0cf08afd7d789636d3a6e1d75e544c917b191d566ab84b",
///         "maxFeePerGas": "0x59682f000",
///         "maxPriorityFeePerGas": "0x77359400",
///         "type": "0x2",
///         "accessList": [],
///         "chainId": "0x1",
///     },
///     "subscription": "0x79a3295f5d5f4bd7efaac4e1738c7ada"
/// }
/// ```
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PendingTransactionResult {
    /// The associated subscription uuid
    #[serde(
        serialize_with = "serialize_uuid_simple",
        deserialize_with = "deserialize_uuid_simple"
    )]
    pub subscription: uuid::Uuid,
    /// The message result
    pub result: Transaction,
}
