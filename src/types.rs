//! Common types for alchemy-rs

/// ## ExposedProvider
///
/// A minimal trait that allows objects to expose a method to retrieve their provider.
///
/// Backwards-compatible with ethers-rs, and forwards-compatible with higher-level wrappers.
///
/// ### Example
///
/// Below we demonstrate implementing the [ExposedProvider](alchemy-rs::types::ExposedProvider) trait for a custom struct wrapping a [Provider](ethers::providers::Provider).
///
/// ```rust
/// use std::convert::TryFrom;
/// use ethers::providers::{Middleware, Provider, Http};
/// use alchemy-rs::{ExposedProvider};
///
/// /// A custom struct that wraps a [Provider](ethers::providers::Provider)
/// #[derive(Debug, Clone)]
/// pub struct ProviderWrapper {
///     /// The provider
///     pub provider: Provider<Http>,
/// }
///
/// impl ExposedProvider<Http> for ProviderWrapper {
///     fn provider(&self) -> &Provider<Http> {
///         &self.provider
///     }
/// }
///
/// #[cfg(test)]
/// mod tests {
///     use super::*;
///     use ethers::providers::{Http, Provider};
///     use std::convert::TryFrom;
///
///    #[test]
///    fn test_expose_provider() {
///        // Instantiate the provider
///        let provider = Provider::<Http>::try_from(
///            "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27"
///        ).expect("could not instantiate HTTP Provider");
///
///        // Create the wrapper containing the provider
///        let wrapper = ProviderWrapper { provider };
///
///        // Retrieve the provider from the wrapper
///        let retrieved = wrapper.provider();
///
///        // Verify that we can get a block from the provider
///        let block = provider.get_block(100u64).await?;
///        println!("Got block: {}", serde_json::to_string(&block)?);
///    }
/// }
/// ```
pub trait ExposedProvider<T> {
    /// Retrieve the provider
    fn provider(&self) -> &ethers::providers::Provider<T>;
}
