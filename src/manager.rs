use ethers::prelude::*;
use std::sync::Arc;

use crate::types::ExposedProvider;

/// An alchemy api connection manager
#[derive(Debug, Clone)]
pub struct AlchemyManager {
    /// A Provider
    /// Wrapped in an atomic reference counter to allow for more flexible consumption
    pub provider: Arc<Provider<Ws>>,
}

impl AlchemyManager {
    /// Create a new AlchemyManager
    pub async fn new(url: &str) -> Self {
        let provider = Provider::connect(String::from(url))
            .await
            .expect("could not instantiate WebSocket Provider");
        AlchemyManager {
            provider: Arc::new(provider),
        }
    }

    /// Initiates a socket stream on the Alchemy Subscriptions API
    ///
    /// ## Arguments
    ///
    /// * `to` - The address to filter transactions sent to
    /// * `from` - The address to filter transactions sent from
    ///
    /// ## Returns
    ///
    /// A [SubscriptionStream](ethers::providers::pubsub::TransactionStream) that implements the [Stream](std::stream::Stream) trait.
    /// This allows you to directly await on the stream to get new transactions.
    ///
    /// ## Example
    ///
    /// ```rust
    /// use std::str::FromStr;
    ///
    /// use ethers::types::Address;
    /// use ethers::providers::StreamExt;
    ///
    /// use alchemy_rs::manager::AlchemyManager;
    ///
    /// async {
    ///     // We have to "await" the creation of a new AlchemyManager as it establishes the connection to the websocket
    ///     let manager = AlchemyManager::new("wss://eth-mainnet.g.alchemy.com/v2/MVNYMOb_58bAMzhXX2pS25NDiZ3Q9HeC").await;
    ///
    ///     // Listen to _pending_ transactions from the Eth2 Deposit Contract
    ///     let eth2_deposit_addr = Address::from_str("00000000219ab540356cBB839Cbe05303d7705Fa").unwrap();
    ///     let stream = manager.alchemy_pending_transactions(
    ///         Some(eth2_deposit_addr),
    ///         None
    ///     ).await.expect("Failed to create TransactionStream for Alchemy");
    ///
    ///     // Get the next transasction from the stream
    ///     let eth2_deposit_tx = stream.take(1).collect::<Vec<_>>().await;
    ///
    ///    // Print the block number of the first transaction
    ///    assert_eq!(eth2_deposit_tx[0].to.unwrap(), eth2_deposit_addr);
    /// };
    /// ```
    pub async fn alchemy_pending_transactions<
        T: Into<Address> + Send + Sync + serde::ser::Serialize,
    >(
        &self,
        to: Option<T>,
        from: Option<T>,
    ) -> Result<SubscriptionStream<'_, ethers::providers::Ws, Transaction>, ProviderError> {
        let mut param_map: serde_json::Map<String, serde_json::value::Value> =
            serde_json::Map::new();
        if let Some(to) = to {
            param_map.insert(
                "toAddress".to_string(),
                serde_json::value::Value::String(format!("{:?}", to.into())),
            );
        }
        if let Some(from) = from {
            param_map.insert(
                "fromAddress".to_string(),
                serde_json::value::Value::String(format!("{:?}", from.into())),
            );
        }
        let params = serde_json::value::Value::Object(param_map);
        // let params = serde_json::value::Value::Array(vec![
        //     serde_json::value::Value::String(format!("{:?}", to.into())),
        // ]);
        println!("Constructed Params: {:?}", params);
        let id: U256 = self
            .provider
            .request("alchemy_pendingTransactions", params)
            .await?;
        // let id: U256 = U256::zero();
        println!("Sent request with id: {:?}", id);
        SubscriptionStream::new(id, &self.provider).map_err(Into::into)
    }
}

impl ExposedProvider<Ws> for AlchemyManager {
    fn provider(&self) -> &Provider<Ws> {
        &self.provider
    }
}
