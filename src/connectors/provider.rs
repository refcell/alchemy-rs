use ethers::prelude::*;

use super::errors::AlchemyConnectionError;

/// An ethers-rs websocket [Provider](ethers::providers::Provider) for alchemy
#[derive(Debug, Default, Clone)]
pub struct EthersWsProvider {
    /// The ethers-rs websocket [Provider](ethers::providers::Provider)
    pub provider: Option<Provider<Ws>>,
}

impl EthersWsProvider {
    /// Create a new ethers-rs websocket [Provider](ethers::providers::Provider) for alchemy
    pub fn new() -> Self {
        Self { provider: None }
    }

    /// Connect to the websocket provider
    pub async fn connect(&mut self, url: &str) -> Result<(), AlchemyConnectionError> {
        match Provider::connect(String::from(url)).await {
            Ok(p) => {
                self.provider = Some(p);
                Ok(())
            }
            Err(e) => Err(AlchemyConnectionError::ProviderError(e)),
        }
    }
}

impl From<Provider<Ws>> for EthersWsProvider {
    fn from(provider: Provider<Ws>) -> Self {
        Self {
            provider: Some(provider),
        }
    }
}
