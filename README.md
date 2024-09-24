# Vyper API Rust SDK

![Vyper](https://images.vyper.trade/0000/vyper-header)

A Rust SDK for interacting with the [Vyper API](https://build.vyper.trade/). This library allows developers to integrate Vyper's HTTP and WebSocket API into their Rust applications with ease.

## Table of Contents

- [Vyper API Rust SDK](#vyper-api-rust-sdk)
  - [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Quick Start](#quick-start)
  - [Usage](#usage)
    - [REST API Example](#rest-api-example)
    - [WebSocket API Example](#websocket-api-example)
  - [API Documentation](#api-documentation)

## Installation

To install the Vyper API Rust SDK, add the following to your `Cargo.toml:`

```bash
[dependencies]
vyper-client_rs = "0.1.0" # Replace with the actual version
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

## Quick Start

Here's a simple example to get you started:

```rust
use vyper_client_rs::client::VyperClient;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the client with your API key
    let client = VyperClient::new("your_api_key_here");

    // Get the list of chain IDs supported by Vyper
    let chain_ids = client.get_chain_ids().await?;
    println!("Supported chain IDs: {:?}", chain_ids);

    Ok(())
}
```

## Usage

### REST API Example

Retrieve the market data for a specific token:

```rust
use vyper_client_rs::client::VyperClient;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = VyperClient::new("your_api_key_here");

    // Fetch the All-Time High (ATH) data for a token
    let token_ath = client
        .get_token_ath(1, "AVs9TA4nWDzfPJE9gGVNJMVhcQy3V9PGazuz33BfG2RA")
        .await?;

    println!("Market Cap USD: {}", token_ath.market_cap_usd);
    println!("Timestamp: {}", token_ath.timestamp);

    Ok(())
}
```

### WebSocket API Example

```rust
use vyper_client_rs::websocket::{VyperWebsocketClient, FeedType, SubscriptionType, TokenSubscriptionMessage, SubscriptionMessageType};
use serde_json::Value;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ws_client = VyperWebsocketClient::new("your_api_key_here".to_string());

    // Define a message handler
    let handler = move |message: Value| {
        println!("Received message: {:?}", message);
    };

    ws_client.set_message_handler(handler);

    // Connect to the WebSocket and subscribe to token events
    ws_client.connect(FeedType::TokenEvents).await?;
    ws_client
        .subscribe(
            FeedType::TokenEvents,
            TokenSubscriptionMessage {
                action: SubscriptionMessageType::Subscribe,
                types: vec![SubscriptionType::PumpfunTokens],
            },
        )
        .await?;
    println!("Subscribed to token events");

    // Start listening for messages
    ws_client.listen().await?;

    Ok(())
}
```

## API Documentation

For detailed information on the Vyper API, refer to the official documentation:

-   API Dashboard: [Vyper Dashboard](https://build.vyper.trade/)
-   API Documentation: [Vyper API Docs](https://docs.vyper.trade/)
