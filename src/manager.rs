use ethers::prelude::*;

use crate::connectors::prelude::*;
use crate::messages::prelude::*;

/// An alchemy api connection manager
#[derive(Debug)]
pub struct AlchemyManager {
    /// The raw alchemy connection url
    pub url: String,
    /// The connector to the alchemy api
    pub connector: AlchemyConnector,
}

impl AlchemyManager {
    /// Create a new AlchemyManager
    pub fn new(url: &str, ty: Option<AlchemyConnectorType>) -> Self {
        Self {
            url: url.to_string(),
            connector: ty.unwrap_or_default().into(),
        }
    }

    /// Connect to the underlying [AlchemyConnector](AlchemyConnector)
    ///
    /// ## Return
    ///
    /// Returns a self reference to allow for method chaining.
    ///
    pub async fn connect(&mut self) -> Result<&Self, AlchemyConnectionError> {
        match &mut self.connector {
            AlchemyConnector::Provider(None) => {
                let provider = Provider::connect(self.url.clone())
                    .await
                    .map_err(AlchemyConnectionError::ProviderError)?;
                self.connector = AlchemyConnector::Provider(Some(provider.into()));
            }
            AlchemyConnector::Raw(None) => {
                let mut conn = RawAlchemyConnection::new();
                conn.connect(&self.url).await?;
                self.connector = AlchemyConnector::Raw(Some(conn));
            }
            AlchemyConnector::Provider(Some(ref mut c)) => match c.connect(&self.url.clone()).await
            {
                Ok(_) => (),
                Err(e) => return Err(e),
            },
            AlchemyConnector::Raw(Some(ref mut c)) => match c.connect(&self.url.clone()).await {
                Ok(_) => (),
                Err(e) => return Err(e),
            },
        }

        Ok(self)
    }

    /// Subscribes to pending transactions
    ///
    /// ## Arguments
    ///
    /// * `to` - The address to filter transactions sent to
    /// * `from` - The address to filter transactions sent from
    ///
    /// ## Returns
    ///
    /// A subscription [Uuid](uuid::Uuid).
    ///
    /// ## Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use alchemy_rs::prelude::*;
    ///
    /// async {
    ///     // We humbly ask that you do not use this alchemy api key
    ///     let mut manager = AlchemyManager::new(
    ///         "wss://eth-mainnet.g.alchemy.com/v2/MVNYMOb_58bAMzhXX2pS25NDiZ3Q9HeC",
    ///         None,
    ///     );
    ///
    ///     // Establish the websocket connection
    ///     // Note: on success, a self reference is returned so we can chain methods
    ///     let _ = manager.connect().await.unwrap();
    ///
    ///     // Listen to _pending_ transactions to the USDT address on mainnet
    ///     // (there should be a lot of these!)
    ///     let usdt_address = Address::from_str("dac17f958d2ee523a2206206994597c13d831ec7").unwrap();
    ///     let sub_id = manager.subscribe(
    ///         Some(usdt_address),
    ///         None
    ///     ).await.expect("Failed to subscribe to pending transactions!");
    ///
    ///     // Now we can grab items from the stream
    ///     let item: AlchemySocketMessageResponse;
    ///     loop {
    ///         match manager.receive(sub_id).await {
    ///             Ok(i) => {
    ///                 item = i;
    ///                 break;
    ///             },
    ///             Err(_) => return,
    ///         }
    ///     }
    ///
    ///    // Print the next item
    ///    println!("Received pending transaction from the stream: {:?}", item);
    /// };
    /// ```
    pub async fn subscribe(
        &mut self,
        to: Option<Address>,
        from: Option<Address>,
    ) -> Result<uuid::Uuid, AlchemyConnectionError> {
        // Extract the internal connection
        let connection = match &mut self.connector {
            AlchemyConnector::Raw(Some(raw_conn)) => raw_conn,
            AlchemyConnector::Provider(Some(_)) => {
                return Err(AlchemyConnectionError::Unimplemented)
            }
            AlchemyConnector::Raw(None) | AlchemyConnector::Provider(None) => {
                return Err(AlchemyConnectionError::MissingConnection)
            }
        };

        // Example Message body
        // { "id": 1, "method": "eth_subscribe", "params": [ "alchemy_pendingTransactions", { "toAddress": "00000000219ab540356cBB839Cbe05303d7705Fa" } ] }

        let mut param_mapping: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        if let Some(t) = to {
            param_mapping.insert(
                "toAddress".to_string(),
                serde_json::Value::String(t.to_string()),
            );
        }
        if let Some(f) = from {
            param_mapping.insert(
                "fromAddress".to_string(),
                serde_json::Value::String(f.to_string()),
            );
        }

        // Construct the Alchemy Socket Message
        let message = AlchemySocketMessage {
            id: 1,
            method: OutSocketMethod::Subscribe,
            params: vec![
                serde_json::Value::String(String::from("alchemy_pendingTransactions")),
                serde_json::Value::Object(param_mapping),
            ],
        };

        // Turn the message into stringified json
        let message_string = match serde_json::to_string(&message) {
            Ok(s) => s,
            Err(e) => return Err(AlchemyConnectionError::Deserialization(e)),
        };

        // Extract the sender from the contained websocket channel
        let sender = match &mut connection.sender {
            Some(s) => s,
            None => return Err(AlchemyConnectionError::MissingSender),
        };

        // Send the string message
        if let Err(e) = sender.send_text(message_string).await {
            return Err(AlchemyConnectionError::SendFailed(e));
        }

        // Flush the send channel
        if let Err(e) = sender.flush().await {
            return Err(AlchemyConnectionError::FlushFailed(e));
        }

        // Extract the receiver from the contained websocket channel
        let receiver = match &mut connection.receiver {
            Some(r) => r,
            None => return Err(AlchemyConnectionError::MissingReceiver),
        };

        // After sending the message, the alchemy socket should respond with a subscription id
        // Ex: { "id": 1, "result": "0x79a3295f5d5f4bd7efaac4e1738c7ada", "jsonrpc": "2.0" }
        let mut data = vec![];
        match receiver.receive_data(&mut data).await {
            Ok(soketto::Data::Text(_)) => {
                // Convert the data into a text string
                let text = match String::from_utf8(data) {
                    Ok(s) => s,
                    Err(_) => return Err(AlchemyConnectionError::InvalidTextString),
                };
                match serde_json::from_str::<AlchemySubscriptionMessageResult>(&text) {
                    Ok(asmr) => Ok(asmr.result), // lol
                    Err(e) => Err(AlchemyConnectionError::Deserialization(e)),
                }
            }
            Ok(soketto::Data::Binary(_)) => Err(AlchemyConnectionError::UnexpectedResponseType),
            Err(soketto::connection::Error::Closed) => Err(AlchemyConnectionError::Closed),
            Err(e) => Err(AlchemyConnectionError::SomeError(e)),
        }
    }

    /// Receive a socket message from the established websocket connection
    pub async fn receive(
        &mut self,
        _sub_id: uuid::Uuid,
    ) -> Result<AlchemySocketMessageResponse, AlchemyConnectionError> {
        // Extract the internal connection
        let connection = match &mut self.connector {
            AlchemyConnector::Raw(Some(raw_conn)) => raw_conn,
            AlchemyConnector::Provider(Some(_)) => {
                return Err(AlchemyConnectionError::Unimplemented)
            }
            AlchemyConnector::Raw(None) | AlchemyConnector::Provider(None) => {
                return Err(AlchemyConnectionError::MissingConnection)
            }
        };

        // Extract the receiver from the contained websocket channel
        let receiver = match &mut connection.receiver {
            Some(r) => r,
            None => return Err(AlchemyConnectionError::MissingReceiver),
        };

        // We should receive an [AlchemySocketMessageResponse](crate::messages::inbound::AlchemySocketMessageResponse)
        // on any new `eth_subscription` message
        let mut data = vec![];

        // TODO: we have to wait for the next message with the given subscription id, not just any message

        match receiver.receive_data(&mut data).await {
            Ok(soketto::Data::Text(_)) => {
                // Convert the data into a text string
                let text = match String::from_utf8(data) {
                    Ok(s) => s,
                    Err(_) => return Err(AlchemyConnectionError::InvalidTextString),
                };
                match serde_json::from_str::<AlchemySocketMessageResponse>(&text) {
                    Ok(asmr) => Ok(asmr),
                    Err(e) => Err(AlchemyConnectionError::Deserialization(e)),
                }
            }
            Ok(soketto::Data::Binary(_)) => Err(AlchemyConnectionError::UnexpectedResponseType),
            Err(soketto::connection::Error::Closed) => Err(AlchemyConnectionError::Closed),
            Err(e) => Err(AlchemyConnectionError::SomeError(e)),
        }
    }
}
