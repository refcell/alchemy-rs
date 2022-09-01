use std::str::FromStr;

use ethers::providers::StreamExt;
use ethers::types::Address;

use alchemy_rs::prelude::*;

#[actix_rt::test]
async fn test_alchemy_subscription() {
    // We have to "await" the creation of a new AlchemyManager as it establishes the connection to the websocket
    // We humbly ask that you do not use this alchemy api key
    let manager =
        AlchemyManager::new("wss://eth-mainnet.g.alchemy.com/v2/MVNYMOb_58bAMzhXX2pS25NDiZ3Q9HeC")
            .await;

    println!("Instantiated AlchemyManager!");

    let eth2_deposit_addr = Address::from_str("00000000219ab540356cBB839Cbe05303d7705Fa").unwrap();
    println!("Streaming transactions to {:?} ...", eth2_deposit_addr);

    // Listen to _pending_ transactions from the Eth2 Deposit Contract
    let stream = manager
        .alchemy_pending_transactions(Some(eth2_deposit_addr), None)
        .await
        .expect("Failed to create TransactionStream for Alchemy");

    println!("Created stream!");

    // Get the next transasction from the stream
    let eth2_deposit_tx = stream.take(1).collect::<Vec<_>>().await;

    println!("Took from stream: {:?}", eth2_deposit_tx);

    // Print the block number of the first transaction
    assert_eq!(eth2_deposit_tx[0].to.unwrap(), eth2_deposit_addr);
}
