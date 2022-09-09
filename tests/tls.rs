use async_tls::TlsConnector;
use async_std::net::TcpStream;

#[actix_rt::test]
async fn test_connecting_to_alchemy() {
    // Create the socket connection
    let socket = match TcpStream::connect("eth-mainnet.g.alchemy.com:443").await {
        Ok(s) => s,
        Err(e) => panic!("Could not connect to Alchemy: {:?}", e),
    };
    println!("Create socket connection to Alchemy");

    // Create the async-tls connector
    let connector = TlsConnector::default();

    // Establish the connection
    let mut _tls_stream = match connector.connect("wss://eth-mainnet.g.alchemy.com/v2/MVNYMOb_58bAMzhXX2pS25NDiZ3Q9HeC", socket).await {
        Ok(s) => s,
        Err(e) => panic!("Could not connect to Alchemy: {:?}", e),
    };

}
