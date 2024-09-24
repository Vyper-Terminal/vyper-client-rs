use std::env;
use tokio;
use dotenv::dotenv; 
use vyper_client_rs::{
    websocket::{VyperWebsocketClient, FeedType, TokenSubscriptionMessage, SubscriptionMessageType, SubscriptionType},
    client::VyperClient,
};

#[tokio::test]
async fn test_vyper_client_integration() {
    dotenv().ok(); 

    let api_key = env::var("VYPER_API_KEY").expect("VYPER_API_KEY must be set");
    let client = VyperClient::new(&api_key);

    let chain_ids = client.get_chain_ids().await.expect("Failed to get chain IDs");
    assert!(!chain_ids.is_empty(), "Chain IDs should not be empty");

    use vyper_client_rs::types::TokenPairsParams;
    let params = TokenPairsParams {
        chain_ids: Some(vec![900]),
        page: Some(1),
        ..Default::default()
    };
    let token_pairs = client.get_token_pairs(params).await.expect("Failed to get token pairs");
    assert!(!token_pairs.pairs.is_empty(), "Token pairs should not be empty");
}

#[tokio::test]
async fn test_vyper_websocket_client_integration() {
    dotenv().ok(); 

    let api_key = env::var("VYPER_API_KEY").expect("VYPER_API_KEY must be set");
    let mut client = VyperWebsocketClient::new(api_key);

    client.connect(FeedType::TokenEvents).await.expect("Failed to connect");

    let received_message = std::sync::Arc::new(tokio::sync::Mutex::new(false));
    let received_message_clone = received_message.clone();
    client.set_message_handler(move |msg| {
        println!("Received message: {:?}", msg);
        let received_message_clone = received_message_clone.clone();
        tokio::spawn(async move {
            let mut received = received_message_clone.lock().await;
            *received = true;
        });
    });

    let subscription_message = TokenSubscriptionMessage {
        action: SubscriptionMessageType::Subscribe,
        types: vec![SubscriptionType::PumpfunTokens],
    };
    client.subscribe(FeedType::TokenEvents, subscription_message).await.expect("Failed to subscribe");

    let listen_future = client.listen();
    tokio::time::timeout(std::time::Duration::from_secs(60), listen_future).await.expect("Timeout waiting for message").expect("Error while listening");

    let received = received_message.lock().await;
    assert!(*received, "No message was received");

    let unsubscription_message = TokenSubscriptionMessage {
        action: SubscriptionMessageType::Unsubscribe,
        types: vec![SubscriptionType::PumpfunTokens],
    };
    client.unsubscribe(FeedType::TokenEvents, unsubscription_message).await.expect("Failed to unsubscribe");

    client.disconnect().await.expect("Failed to disconnect");
}