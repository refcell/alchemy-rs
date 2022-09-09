<img align="right" width="150" height="150" top="100" src="./assets/alchemy.png">

# alchemy-rs • [![ci](https://github.com/abigger87/alchemy-rs/actions/workflows/ci.yaml/badge.svg)](https://github.com/abigger87/alchemy-rs/actions/workflows/ci.yaml) ![license](https://img.shields.io/github/license/abigger87/alchemy-rs?label=license) [![crates.io](https://img.shields.io/crates/v/alchemy-rs.svg)](https://crates.io/crates/alchemy-rs)


**Minimal** ethers-rs wrappers for the Alchemy API built in pure Rust.


## Getting Started

Add the `alchemy-rs` crate to your project:

```toml
alchemy_rs = "0.1.0"
```


## Usage

[alchemy-rs](https://github.com/abigger87/alchemy-rs) is a minimal ethers-rs wrapper for the Alchemy API built in pure rust.

The [AlchemyManager](src/manager.rs) is the main entry point for interacting with the Alchemy API. It is initializable with an Alchemy API key and a [Chain](https://docs.rs/ethers/latest/ethers/types/enum.Chain.html). Alchemy supports the following chains: ...



## Examples

Listening to pending transactions using alchemy's `alchemy_pendingTransactions` method is demonstrated below.

```rust
use std::str::FromStr;
use std::env;

use alchemy_rs::prelude::*;

async {
    // Read an alchemy websocket api key from the `ALCHEMY_API_KEY` environment variable
    let api_key = env::var("ALCHEMY_API_KEY").expect("ALCHEMY_API_KEY must be set");

    // Create the AlchemyManager
    let mut manager = AlchemyManager::new(&format!("wss://eth-mainnet.g.alchemy.com/v2/{}", api_key), None);

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
    let item: AlchemySocketMessageResponse;
    loop {
        match manager.receive(sub_id).await {
            Ok(i) => {
                item = i;
                break;
            },
            Err(e) => {
                println!("Error receiving item: {:?}", e);
                return;
            }
        }
    }

   // Print the next item
   println!("Received pending transaction from the stream: {:?}", item);
};
```


## Safety

> **Warning**
>
> This is **experimental software** and is provided on an "as is" and "as available" basis.
> Expect rapid iteration and **use at your own risk**.


## License

[MIT](https://github.com/abigger87/alchemy-rs/blob/master/LICENSE), but go crazy :P


## Acknowledgements

- [ethers-rs](https://github.com/gakonst/ethers-rs)
