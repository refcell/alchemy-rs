use tokio_util::compat::TokioAsyncReadCompatExt;

#[actix_rt::test]
async fn test_connecting_to_alchemy() {
    // Create the socket connection
    let socket = match tokio::net::TcpStream::connect("eth-mainnet.g.alchemy.com:443").await {
        Ok(s) => s,
        Err(e) => panic!("Could not connect to Alchemy: {:?}", e),
    };
    println!("Create socket connection to Alchemy");

    // Create the client connection
    let compatible_socket = futures::io::BufReader::new(futures::io::BufWriter::new(socket.compat()));
    let mut client = soketto::handshake::Client::new(compatible_socket, "ws://eth-mainnet.g.alchemy.com", "/v2/MVNYMOb_58bAMzhXX2pS25NDiZ3Q9HeC");
    println!("Created client connection");

    // let api_key = "MVNYMOb_58bAMzhXX2pS25NDiZ3Q9HeC".as_bytes().to_vec();
    let gzip = "gzip".as_bytes().to_vec();
    let version = "2.0.3".as_bytes().to_vec();
    let auth_header = vec![
        // soketto::handshake::client::Header {
        //     name: "ALCHEMY_API_KEY",
        //     value: &api_key,
        // },
        soketto::handshake::client::Header {
            name: "Alchemy-Ethers-Sdk-Version",
            value: &version,
        },
        soketto::handshake::client::Header {
            name: "Accept-Encoding",
            value: &gzip,
        },
    ];
    client.set_headers(auth_header.as_slice());

    // Handshake the connection
    match client.handshake().await {
        Ok(sr) => {
            match sr {
                soketto::handshake::ServerResponse::Accepted { protocol } => {
                    println!("Accepted protocol: {:?}", protocol);
                }
                soketto::handshake::ServerResponse::Redirect { status_code, location } => {
                    println!("Redirected with status code: {}, location: {}", status_code, location);
                }
                soketto::handshake::ServerResponse::Rejected { status_code } => {
                    println!("Rejected with status code: {}", status_code);
                    panic!("Rejected with status code: {}", status_code);
                }
            }
        }
        Err(e) => panic!("Handshake error! {:?}", e),
    }

    let (_, _) = client.into_builder().finish();
}
