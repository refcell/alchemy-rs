use alchemy_rs::messages::{
    outbound::{AlchemySocketMessage, OutSocketMethod},
    prelude::AlchemySubscriptionMessageResult,
};

mod util;

#[test]
fn test_outbound_alchemy_socket_message_serialization() {
    // Craft an expected alchemy outbound eth_subscribe socket message json string
    let expected = r#"{
        "id": 1,
        "method": "eth_subscribe",
        "params": [
            "alchemy_pendingTransactions",
            {
                "toAddress": "dac17f958d2ee523a2206206994597c13d831ec7"
            }
        ]
    }"#;

    // Construct a raw outbound alchemy socket message
    let mut map = serde_json::Map::new();
    map.insert(
        "toAddress".to_string(),
        serde_json::Value::String("dac17f958d2ee523a2206206994597c13d831ec7".to_string()),
    );
    let constructed = AlchemySocketMessage {
        id: 1,
        method: OutSocketMethod::Subscribe,
        params: vec![
            serde_json::Value::String(String::from("alchemy_pendingTransactions")),
            serde_json::Value::Object(map),
        ],
    };

    // Make sure it can serialize to the expected string
    let serialized_string = match serde_json::to_string(&constructed) {
        Ok(s) => {
            util::assert_strings_roughly_equal(&s, expected);
            s
        }
        Err(e) => panic!("Failed to serialize outbound message: {}", e),
    };

    // Now take the resulting serialized string and deserialize it into an alchemy outbound socket message
    let deserialized = match serde_json::from_str::<AlchemySocketMessage>(&serialized_string) {
        Ok(d) => d,
        Err(e) => panic!("Failed to deserialize outbound message: {}", e),
    };

    // Make sure the deserialized message is the same as the constructed message
    assert_eq!(constructed, deserialized);
}

#[test]
fn test_inbound_alchemy_subscription_message_result_serialization() {
    // Craft the expected json string
    let expected = r#"{
        "id": 1,
        "result": "0xa79a6df98fb2a42516b5aca3177fbb6c",
        "jsonrpc": "2.0"
    }"#;

    // Construct a concrete struct
    let constructed = AlchemySubscriptionMessageResult {
        id: 1,
        result: uuid::Uuid::parse_str("a79a6df98fb2a42516b5aca3177fbb6c").unwrap(),
        jsonrpc: "2.0".to_string(),
    };

    // Validate serialization
    let serialized_string = match serde_json::to_string(&constructed) {
        Ok(s) => {
            util::assert_strings_roughly_equal(&s, expected);
            s
        }
        Err(e) => panic!("Failed to serialize: {}", e),
    };

    // Now take the resulting serialized string and deserialize it
    let deserialized =
        match serde_json::from_str::<AlchemySubscriptionMessageResult>(&serialized_string) {
            Ok(d) => d,
            Err(e) => panic!("Failed to deserialize: {}", e),
        };

    // Make sure the deserialized message is the same as the constructed message
    assert_eq!(constructed, deserialized);
}
