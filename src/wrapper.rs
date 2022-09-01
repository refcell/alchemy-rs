//! TODO: An ethers-rs wrapper for the Alchemy API.
//! Similar to the CeloMiddleware as a

// #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
// #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
// pub trait AlchemyMiddleware: Middleware {
//     async fn alchemy_pending_transactions<T: Into<Address> + Send + Sync + serde::Serialize>(
//         &self,
//         to: Option<T>,
//         from: Option<T>,
//     ) -> Result<FilterWatcher<'_, Self::Provider, H256>, Self::Error> {
//         self.provider()
//             .alchemy_pending_transactions(to, from)
//             .await
//             .map_err(FromErr::from)
//     }
// }

// #[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
// #[cfg_attr(not(target_arch = "wasm32"), async_trait)]
// impl<P: JsonRpcClient> AlchemyMiddleware for Provider<P> {
//     async fn alchemy_pending_transactions<T: Into<Address> + Send + Sync + serde::Serialize>(
//         &self,
//         to: Option<T>,
//         from: Option<T>,
//     ) -> Result<FilterWatcher<'_, Self::Provider, H256>, Self::Error> {
//         let params = vec![
//             to.map(|t| serde_json::to_value(t).expect("Types never fail to serialize.")),
//             from.map(|t| serde_json::to_value(t).expect("Types never fail to serialize.")),
//         ]
//         .into_iter()
//         .flatten()
//         .collect();
//         self.request("alchemy_pendingTransactions", params).await
//     }
// }
