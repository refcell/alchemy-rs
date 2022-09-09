use std::str::FromStr;

use ethers::types::Address;

use alchemy_rs::{prelude::*, connectors::AlchemyConnectorType};

#[actix_rt::test]
async fn test_alchemy_subscription() {
    // Create the AlchemyManager
    let mut manager = AlchemyManager::new(
        "wss://eth-mainnet.g.alchemy.com/v2/MVNYMOb_58bAMzhXX2pS25NDiZ3Q9HeC",
        Some(AlchemyConnectorType::Raw),
    );

    // Connect to the websocket
    let _ = manager.connect().await.unwrap();

    // Listen to _pending_ transactions to the USDT address on mainnet
    // (there should be a lot of these!)
    let usdt_address = Address::from_str("dac17f958d2ee523a2206206994597c13d831ec7").unwrap();

    // Try to subscribe to pending transactions
    let sub_id = match manager.subscribe(Some(usdt_address), None).await {
        Ok(id) => id,
        Err(e) => {
            println!("Error subscribing to pending transactions: {:?}", e);
            return;
        }
    };

    // Now we can grab items from the stream
    let item = match manager.receive(sub_id).await {
        Ok(i) => i,
        Err(e) => {
            println!("Error receiving item: {:?}", e);
            return;
        }
    };

    // Print the next item
    println!("Received pending transaction from the stream: {:?}", item);
}
