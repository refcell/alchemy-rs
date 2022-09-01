use ethers::providers::{Http, Middleware, Provider};
use std::convert::TryFrom;

use alchemy_rs::types::*;

#[derive(Debug, Clone)]
pub struct ProviderWrapper {
    /// The provider
    pub provider: Provider<Http>,
}

impl ExposedProvider<Http> for ProviderWrapper {
    fn provider(&self) -> &Provider<Http> {
        &self.provider
    }
}

#[actix_rt::test]
async fn test_expose_provider() {
    // Instantiate the provider
    let provider =
        Provider::<Http>::try_from("https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27")
            .expect("could not instantiate HTTP Provider");

    // Create the wrapper containing the provider
    let wrapper = ProviderWrapper { provider };

    // Retrieve the provider from the wrapper
    let retrieved = wrapper.provider();

    // Verify that we can get a block from the provider
    let block = retrieved.get_block(100u64).await.unwrap();
    println!("Got block: {}", serde_json::to_string(&block).unwrap());
}
