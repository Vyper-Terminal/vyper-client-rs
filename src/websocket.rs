use std::sync::Arc;
use tokio::sync::Mutex;
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream, MaybeTlsStream};
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use crate::types::{ChainAction, TokenPair};
use crate::errors::VyperError;  

#[derive(Debug, Clone, PartialEq)]
pub enum FeedType {
    TokenEvents,
    MigrationEvents,
    WalletEvents,
}

impl ToString for FeedType {
    fn to_string(&self) -> String {
        match self {
            FeedType::TokenEvents => "token-events".to_string(),
            FeedType::MigrationEvents => "migration-events".to_string(),
            FeedType::WalletEvents => "wallet-events".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SubscriptionMessageType {
    Subscribe,
    Unsubscribe,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SubscriptionType {
    PumpfunTokens,
    RaydiumAmmTokens,
    RaydiumCpmmTokens,
    RaydiumClmmTokens,
}

#[derive(Debug, Clone, Serialize)]
pub struct TokenSubscriptionMessage {
    pub action: SubscriptionMessageType,
    pub types: Vec<SubscriptionType>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WalletSubscriptionMessage {
    pub action: SubscriptionMessageType,
    pub wallets: Vec<String>,
}

type MessageHandler = Arc<Mutex<dyn FnMut(Value) + Send + Sync>>;

#[async_trait]
pub trait WebSocketConnection: Send + Sync {
    async fn connect(&mut self, url: &str) -> Result<(), VyperError>;
    async fn send(&mut self, data: &str) -> Result<(), VyperError>;
    async fn receive(&mut self) -> Result<String, VyperError>;
    async fn close(&mut self) -> Result<(), VyperError>;
}

pub struct VyperWebsocketClient {
    base_url: String,
    api_key: String,
    conn: Arc<Mutex<Option<Box<dyn WebSocketConnection>>>>,
    message_handler: Option<MessageHandler>,
    current_feed_type: Arc<Mutex<Option<FeedType>>>,
}

impl VyperWebsocketClient {
    pub fn new(api_key: String) -> Self {
        Self {
            base_url: "wss://api.vyper.trade/api/v1/ws".to_string(),
            api_key,
            conn: Arc::new(Mutex::new(None)),
            message_handler: None,
            current_feed_type: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn connect(&self, feed_type: FeedType) -> Result<(), VyperError> {
        let mut conn_guard = self.conn.lock().await;
        let url = format!("{}/{}?apiKey={}", self.base_url, feed_type.to_string(), self.api_key);

        if conn_guard.is_none() {
            let mut new_conn: Box<dyn WebSocketConnection> = Box::new(WebSocketImpl::new());
            new_conn.connect(&url).await.map_err(|e| VyperError::WebsocketError {
                message: e.to_string(),
                status_code: None,
                connection_info: Some(url.clone()),
            })?;
            *conn_guard = Some(new_conn);
        } else {
            conn_guard.as_mut().unwrap().connect(&url).await.map_err(|e| VyperError::WebsocketError {
                message: e.to_string(),
                status_code: None,
                connection_info: Some(url.clone()),
            })?;
        }

        let mut feed_type_guard = self.current_feed_type.lock().await;
        *feed_type_guard = Some(feed_type);

        Ok(())
    }

    pub async fn subscribe<M: Serialize>(&self, feed_type: FeedType, message: M) -> Result<(), VyperError> {
        let mut conn_guard = self.conn.lock().await;
        let feed_type_guard = self.current_feed_type.lock().await;

        if conn_guard.is_none() {
            return Err(VyperError::WebsocketError {
                message: "Not connected".to_string(),
                status_code: None,
                connection_info: None,
            });
        }

        if feed_type != *feed_type_guard.as_ref().unwrap() {
            return Err(VyperError::WebsocketError {
                message: "Feed type mismatch".to_string(),
                status_code: None,
                connection_info: None,
            });
        }

        let data = serde_json::to_string(&message).map_err(|e| VyperError::DeserializeError(e))?;
        conn_guard.as_mut().unwrap().send(&data).await.map_err(|e| VyperError::WebsocketError {
            message: e.to_string(),
            status_code: None,
            connection_info: None,
        })?;

        Ok(())
    }

    pub async fn unsubscribe<M: Serialize>(&self, feed_type: FeedType, message: M) -> Result<(), VyperError> {
        self.subscribe(feed_type, message).await
    }

    pub async fn listen(&self) -> Result<(), VyperError> {
        let mut conn_guard = self.conn.lock().await;
        let feed_type_guard = self.current_feed_type.lock().await;

        if conn_guard.is_none() {
            return Err(VyperError::WebsocketError {
                message: "Not connected".to_string(),
                status_code: None,
                connection_info: None,
            });
        }

        while let Ok(msg) = conn_guard.as_mut().unwrap().receive().await {
            let raw_data: Value = serde_json::from_str(&msg).map_err(|e| VyperError::DeserializeError(e))?;
            let converted_data = self.convert_message(&raw_data, &feed_type_guard).await;

            if let Err(_) = converted_data {
                continue;
            }

            if let Some(ref handler) = self.message_handler {
                let mut handler = handler.lock().await;
                handler(converted_data.unwrap());
            }
        }

        Ok(())
    }

    async fn convert_message(&self, data: &Value, feed_type: &Option<FeedType>) -> Result<Value, VyperError> {
        if feed_type.is_none() {
            return Err(VyperError::WebsocketError {
                message: "Feed type is not set".to_string(),
                status_code: None,
                connection_info: None,
            });
        }
    
        match feed_type.as_ref().unwrap() {
            FeedType::WalletEvents => {
                let chain_action: ChainAction = serde_json::from_value(data.clone()).map_err(|e| VyperError::DeserializeError(e))?;
                Ok(serde_json::to_value(chain_action).map_err(|e| VyperError::DeserializeError(e))?)
            },
            FeedType::MigrationEvents | FeedType::TokenEvents => {
                let token_pair: TokenPair = serde_json::from_value(data.clone()).map_err(|e| VyperError::DeserializeError(e))?;
                Ok(serde_json::to_value(token_pair).map_err(|e| VyperError::DeserializeError(e))?)
            },
        }
    }

    pub async fn disconnect(&self) -> Result<(), VyperError> {
        let mut conn_guard = self.conn.lock().await;
        if let Some(ref mut ws) = conn_guard.as_mut() {
            ws.close().await.map_err(|e| VyperError::WebsocketError {
                message: e.to_string(),
                status_code: None,
                connection_info: None,
            })?;
        }

        *conn_guard = None;
        let mut feed_type_guard = self.current_feed_type.lock().await;
        *feed_type_guard = None;

        Ok(())
    }

    pub async fn ping(&self) -> Result<(), VyperError> {
        let mut conn_guard = self.conn.lock().await;
        if let Some(ref mut ws) = conn_guard.as_mut() {
            ws.send("ping").await.map_err(|e| VyperError::WebsocketError {
                message: e.to_string(),
                status_code: None,
                connection_info: None,
            })?;
        }
        Ok(())
    }

    pub fn set_message_handler<F>(&mut self, handler: F)
    where
        F: Fn(Value) + Send + Sync + 'static,
    {
        self.message_handler = Some(Arc::new(Mutex::new(handler)));
    }
}

struct WebSocketImpl {
    ws_stream: Option<WebSocketStream<MaybeTlsStream<TcpStream>>>,
}

impl WebSocketImpl {
    fn new() -> Self {
        Self {
            ws_stream: None,
        }
    }
}

#[async_trait]
impl WebSocketConnection for WebSocketImpl {
    async fn connect(&mut self, url: &str) -> Result<(), VyperError> {
        let (ws_stream, _) = connect_async(url).await.map_err(|e| VyperError::WebsocketError {
            message: e.to_string(),
            status_code: None,
            connection_info: Some(url.to_string()),
        })?;
        self.ws_stream = Some(ws_stream);
        Ok(())
    }

    async fn send(&mut self, data: &str) -> Result<(), VyperError> {
        if let Some(ws_stream) = &mut self.ws_stream {
            ws_stream.send(Message::Text(data.to_string())).await.map_err(|e| VyperError::WebsocketError {
                message: e.to_string(),
                status_code: None,
                connection_info: None,
            })?;
            Ok(())
        } else {
            Err(VyperError::WebsocketError {
                message: "WebSocket is not connected".to_string(),
                status_code: None,
                connection_info: None,
            })
        }
    }

    async fn receive(&mut self) -> Result<String, VyperError> {
        if let Some(ws_stream) = &mut self.ws_stream {
            if let Some(message) = ws_stream.next().await {
                match message {
                    Ok(message) => match message {
                        Message::Text(text) => Ok(text),
                        _ => Err(VyperError::WebsocketError {
                            message: "Received non-text message".to_string(),
                            status_code: None,
                            connection_info: None,
                        }),
                    },
                    Err(e) => Err(VyperError::WebsocketError {
                        message: e.to_string(),
                        status_code: None,
                        connection_info: None,
                    }),
                }
            } else {
                Err(VyperError::WebsocketError {
                    message: "WebSocket stream ended".to_string(),
                    status_code: None,
                    connection_info: None,
                })
            }
        } else {
            Err(VyperError::WebsocketError {
                message: "WebSocket is not connected".to_string(),
                status_code: None,
                connection_info: None,
            })
        }
    }

    async fn close(&mut self) -> Result<(), VyperError> {
        if let Some(ws_stream) = &mut self.ws_stream {
            ws_stream.close(None).await.map_err(|e| VyperError::WebsocketError {
                message: e.to_string(),
                status_code: None,
                connection_info: None,
            })?;
            self.ws_stream = None;
            Ok(())
        } else {
            Err(VyperError::WebsocketError {
                message: "WebSocket is not connected".to_string(),
                status_code: None,
                connection_info: None,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;
    use mockall::mock;
    use std::sync::Arc;

    mock! {
        WebSocket {}
        #[async_trait]
        impl WebSocketConnection for WebSocket {
            async fn connect(&mut self, url: &str) -> Result<(), VyperError>;
            async fn send(&mut self, data: &str) -> Result<(), VyperError>;
            async fn receive(&mut self) -> Result<String, VyperError>;
            async fn close(&mut self) -> Result<(), VyperError>;
        }
    }    

    #[tokio::test]
    async fn test_new_client() {
        let client = VyperWebsocketClient::new("test_api_key".to_string());
        assert_eq!(client.base_url, "wss://api.vyper.trade/api/v1/ws");
        assert_eq!(client.api_key, "test_api_key");
    }

    #[tokio::test]
    async fn test_connect() {
        let mut mock_ws = MockWebSocket::new();
        
        mock_ws.expect_connect()
            .with(eq("wss://api.vyper.trade/api/v1/ws/TokenEvents?apiKey=test_api_key"))
            .times(1)
            .returning(|_| Ok(()));

        let client = Arc::new(VyperWebsocketClient::new("test_api_key".to_string()));

        {
            let mut conn_guard = client.conn.lock().await;
            *conn_guard = Some(Box::new(mock_ws));
        }

        let _ = client.connect(FeedType::TokenEvents).await;

        let feed_type_guard = client.current_feed_type.lock().await;
        assert_eq!(*feed_type_guard, Some(FeedType::TokenEvents));
    }

    #[tokio::test]
    async fn test_disconnect() {
        let mut mock_ws = MockWebSocket::new();
        mock_ws.expect_close()
            .returning(|| Ok(()));

        let client = VyperWebsocketClient::new("test_api_key".to_string());
        *client.conn.lock().await = Some(Box::new(mock_ws));
        
        let result = client.disconnect().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_subscribe() {
        let mut mock_ws = MockWebSocket::new();
        mock_ws.expect_send()
            .returning(|_| Ok(()));

        let client = VyperWebsocketClient::new("test_api_key".to_string());
        *client.conn.lock().await = Some(Box::new(mock_ws));
        *client.current_feed_type.lock().await = Some(FeedType::TokenEvents);

        let message = TokenSubscriptionMessage {
            action: SubscriptionMessageType::Subscribe,
            types: vec![SubscriptionType::PumpfunTokens],
        };

        let result = client.subscribe(FeedType::TokenEvents, message).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_unsubscribe() {
        let mut mock_ws = MockWebSocket::new();
        mock_ws.expect_send()
            .returning(|_| Ok(()));

        let client = VyperWebsocketClient::new("test_api_key".to_string());
        *client.conn.lock().await = Some(Box::new(mock_ws));
        *client.current_feed_type.lock().await = Some(FeedType::TokenEvents);

        let message = TokenSubscriptionMessage {
            action: SubscriptionMessageType::Unsubscribe,
            types: vec![SubscriptionType::PumpfunTokens],
        };

        let result = client.unsubscribe(FeedType::TokenEvents, message).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_ping() {
        let mut mock_ws = MockWebSocket::new();
        mock_ws.expect_send()
            .with(eq("ping"))
            .returning(|_| Ok(()));

        let client = VyperWebsocketClient::new("test_api_key".to_string());
        *client.conn.lock().await = Some(Box::new(mock_ws));
        
        let result = client.ping().await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_listen() {
        let mut mock_ws = MockWebSocket::new();
        mock_ws.expect_receive()
            .returning(|| Ok(r#"{
                "event": "tokenPair",
                "chainId": 1,
                "tokenMint": "0x123",
                "name": "Test Token",
                "symbol": "TEST",
                "buyTxnCount": 10,
                "sellTxnCount": 5,
                "tokenPriceUsd": 1.5,
                "tokenPriceAsset": 0.1,
                "volumeUsd": 1000.0,
                "volumeAsset": 100.0,
                "tokenMarketCapUsd": 1000000.0,
                "tokenMarketCapAsset": 100000.0,
                "tokenLiquidityUsd": 500000.0,
                "tokenLiquidityAsset": 50000.0,
                "transactionCount": 15,
                "contractCreator": "0x456",
                "lpCreator": "0x789",
                "createdTimestamp": 1631234567,
                "totalSupply": 1000000.0,
                "pooledToken": 500000.0,
                "pooledAsset": 50000.0,
                "initialUsdLiquidity": 100000.0,
                "initialAssetLiquidity": 10000.0,
                "priceChangePercent": 5.0,
                "top10HoldingPercent": 60.0,
                "lpBurned": false,
                "tokenType": "SPL",
                "marketId": "market1"
            }"#.to_string()));
        mock_ws.expect_receive()
            .returning(|| Err("Connection closed".into()));

        let mut client = VyperWebsocketClient::new("test_api_key".to_string());
        *client.conn.lock().await = Some(Box::new(mock_ws));
        *client.current_feed_type.lock().await = Some(FeedType::TokenEvents);
        
        let received_message = Arc::new(Mutex::new(false));
        let received_message_clone = received_message.clone();
        
        client.set_message_handler(move |msg| {
            let received_message_clone = received_message_clone.clone();
            tokio::spawn(async move {
                assert!(msg.get("chainId").is_some(), "chainId field is missing");
                assert!(msg.get("tokenMint").is_some(), "tokenMint field is missing");
                assert!(msg.get("name").is_some(), "name field is missing");
                assert!(msg.get("symbol").is_some(), "symbol field is missing");
                let mut received = received_message_clone.lock().await;
                *received = true;
            });
        });

        let listen_future = client.listen();
        tokio::time::timeout(std::time::Duration::from_secs(1), listen_future).await.unwrap_or(Ok(())).unwrap();
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let received = received_message.lock().await;
        assert!(*received, "Message was not received and processed");
    }

    #[tokio::test]
    async fn test_error_handling() {
        let mut mock_ws = MockWebSocket::new();
        mock_ws.expect_connect()
            .returning(|_| Err("Connection failed".into()));

        let client = VyperWebsocketClient::new("test_api_key".to_string());
        *client.conn.lock().await = Some(Box::new(mock_ws));
        
        let result = client.connect(FeedType::TokenEvents).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_set_message_handler() {
        let client = VyperWebsocketClient::new("test_api_key".to_string());
        let mut client = client;
        client.set_message_handler(|_| {
            // Do nothing, just testing if it sets without error
        });
        assert!(client.message_handler.is_some());
    }

    #[tokio::test]
    async fn test_feed_type_mismatch() {
        let mut mock_ws = MockWebSocket::new();
        mock_ws.expect_send()
            .returning(|_| Ok(()));

        let client = VyperWebsocketClient::new("test_api_key".to_string());
        *client.conn.lock().await = Some(Box::new(mock_ws));
        *client.current_feed_type.lock().await = Some(FeedType::TokenEvents);

        let message = WalletSubscriptionMessage {
            action: SubscriptionMessageType::Subscribe,
            wallets: vec!["0x123".to_string()],
        };

        let result = client.subscribe(FeedType::WalletEvents, message).await;
        assert!(result.is_err());
    }
}