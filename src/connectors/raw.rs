use futures::io::{BufReader, BufWriter};
use soketto::handshake;
use tokio::net::TcpStream;
use tokio_util::compat::{Compat, TokioAsyncReadCompatExt};

use super::errors::AlchemyConnectionError;

/// A Raw, Persistent Websocket Connection to the Alchemy API using [soketto](https://docs.rs/soketto/latest/soketto/)
///
/// ## Alchemy
///
/// The Alchemy Websocket API allows you to interactively demo the endpoints.
///
/// Simply install a websocket shell command and connect to the demo endpoint:
///
/// ```sh
/// $ wscat -c wss://eth-mainnet.ws.alchemyapi.io/ws/demo
///
///  // create subscription
/// > {"id": 1, "method": "eth_subscribe", "params": ["newHeads"]}
/// < {"jsonrpc":"2.0","id":1,"result":"0xcd0c3e8af590364c09d0fa6a1210faf5"}
///
/// // incoming notifications
/// < {"jsonrpc":"2.0","method":"eth_subscription","params":{"subscription":"0xcd0c3e8af590364c09d0fa6a1210faf5","result":{"difficulty":"0xd9263f42a87",<...>, "uncles":[]}}}
/// < {"jsonrpc":"2.0","method":"eth_subscription","params":{"subscription":"0xcd0c3e8af590364c09d0fa6a1210faf5","result":{"difficulty":"0xd90b1a7ad02", <...>, "uncles":["0x80aacd1ea4c9da32efd8c2cc9ab38f8f70578fcd46a1a4ed73f82f3e0957f936"]}}}
///
/// // cancel subscription
/// > {"id": 1, "method": "eth_unsubscribe", "params": ["0xcd0c3e8af590364c09d0fa6a1210faf5"]}
/// < {"jsonrpc":"2.0","id":1,"result":true}
/// ```
///
#[derive(Debug, Default)]
pub struct RawAlchemyConnection {
    // / The websocket connection
    // pub connection: Option<soketto::handshake::Client<'a, BufReader<BufWriter<Compat<TcpStream>>>>>,
    /// The websocket client sender after building
    pub sender: Option<soketto::Sender<BufReader<BufWriter<Compat<TcpStream>>>>>,
    /// The websocket client receiver after building
    pub receiver: Option<soketto::Receiver<BufReader<BufWriter<Compat<TcpStream>>>>>,
}

impl RawAlchemyConnection {
    /// Create a new RawAlchemyConnection
    pub fn new() -> Self {
        Self {
            sender: None,
            receiver: None,
        }
    }

    /// Connect to the sokettot websocket
    pub async fn connect(&mut self, url: &'_ str) -> Result<(), AlchemyConnectionError> {
        // Create the socket connection
        let socket = match tokio::net::TcpStream::connect(url).await {
            Ok(s) => s,
            Err(e) => return Err(AlchemyConnectionError::RawStreamError(e)),
        };

        // Create the client connection
        let compatible_socket = BufReader::new(BufWriter::new(socket.compat()));
        let mut client = handshake::Client::new(compatible_socket, url, "");

        // Handshake the connection
        match client.handshake().await {
            Ok(sr) => {
                tracing::info!("Got handshake response: {:?}", sr);
                tracing::debug!(
                    "Expecting server response: {:?}",
                    handshake::ServerResponse::Accepted { protocol: None }
                );
            }
            Err(e) => return Err(AlchemyConnectionError::RawHandshakeError(e)),
        }

        let (sender, receiver) = client.into_builder().finish();
        self.sender = Some(sender);
        self.receiver = Some(receiver);

        Ok(())
    }
}
