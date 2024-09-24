use reqwest::Client as HttpClient;
use std::collections::HashMap;
use std::time::Duration;
use crate::types::*;
use crate::errors::*;

pub struct VyperClient {
    base_url: String,
    api_key: String,
    http_client: HttpClient,
}

impl VyperClient {
    pub fn new(api_key: &str) -> Self {
        Self {
            base_url: "https://api.vyper.trade".to_string(),
            api_key: api_key.to_string(),
            http_client: HttpClient::builder()
                .timeout(Duration::from_secs(10))
                .https_only(true) 
                .build()
                .unwrap(),
        }
    }

    async fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: reqwest::Method,
        endpoint: &str,
        params: Option<&[(&str, String)]>,
    ) -> Result<T, VyperError> {
        let url = format!("{}{}", self.base_url, endpoint);
        let mut request = self.http_client.request(method, &url);

        request = request.header("X-API-Key", &self.api_key);

        if let Some(query_params) = params {
            request = request.query(query_params);
        }

        let response = request.send().await?;
        let status = response.status();
        let headers = response.headers().clone();
        let body = response.text().await?;

        if status.is_success() {
            let api_response: APIResponse<T> = serde_json::from_str(&body)?;
            Ok(api_response.data)
        } else {
            let error_response: APIResponse<serde_json::Value> = serde_json::from_str(&body)?;
            match status.as_u16() {
                401 => Err(VyperError::AuthenticationError(error_response.message)),
                429 => Err(VyperError::RateLimitError {
                    message: error_response.message,
                    retry_after: headers.get("Retry-After")
                        .and_then(|h| h.to_str().ok())
                        .and_then(|s| s.parse().ok()),
                }),
                500..=599 => Err(VyperError::ServerError(error_response.message)),
                _ => Err(VyperError::ApiError(error_response.message, status.as_u16())),
            }
        }
    }

    pub async fn get_chain_ids(&self) -> Result<std::collections::HashMap<String, i32>, VyperError> {
        self.request(reqwest::Method::GET, "/api/v1/chain/ids", None).await
    }

    pub async fn get_token_ath(&self, chain_id: i32, market_id: &str) -> Result<TokenATH, VyperError> {
        let params = vec![
            ("chainID", chain_id.to_string()),
            ("marketID", market_id.to_string()),
        ];
        self.request(reqwest::Method::GET, "/api/v1/token/ath", Some(&params)).await
    }

    pub async fn get_token_market(&self, market_id: &str, chain_id: i32, interval: &str) -> Result<TokenPair, VyperError> {
        let params = vec![
            ("chainID", chain_id.to_string()),
            ("interval", interval.to_string()),
        ];
        self.request(reqwest::Method::GET, &format!("/api/v1/token/market/{}", market_id), Some(&params)).await
    }

    pub async fn get_token_holders(&self, market_id: &str, chain_id: i32) -> Result<(Vec<TokenHolder>, i32), VyperError> {
        let params = vec![
            ("marketID", market_id.to_string()),
            ("chainID", chain_id.to_string()),
        ];
        let result: HashMap<String, serde_json::Value> = self.request(reqwest::Method::GET, "/api/v1/token/holders", Some(&params)).await?;
        
        let holders: Vec<TokenHolder> = serde_json::from_value(result["holders"].clone())?;
        let total_holders: i32 = result["total_holders"].as_i64().unwrap() as i32;

        Ok((holders, total_holders))
    }

    pub async fn get_token_markets(&self, token_mint: &str, chain_id: i32) -> Result<Vec<TokenMarket>, VyperError> {
        let params = vec![
            ("tokenMint", token_mint.to_string()),
            ("chainID", chain_id.to_string()),
        ];
        self.request(reqwest::Method::GET, "/api/v1/token/markets", Some(&params)).await
    }

    pub async fn get_wallet_holdings(&self, wallet_address: &str, chain_id: i32) -> Result<Vec<WalletHolding>, VyperError> {
        let params = vec![
            ("walletAddress", wallet_address.to_string()),
            ("chainID", chain_id.to_string()),
        ];
        self.request(reqwest::Method::GET, "/api/v1/wallet/holdings", Some(&params)).await
    }

    pub async fn get_wallet_aggregated_pnl(&self, wallet_address: &str, chain_id: i32) -> Result<WalletAggregatedPnL, VyperError> {
        let params = vec![
            ("walletAddress", wallet_address.to_string()),
            ("chainID", chain_id.to_string()),
        ];
        self.request(reqwest::Method::GET, "/api/v1/wallet/aggregated-pnl", Some(&params)).await
    }

    pub async fn get_wallet_pnl(&self, wallet_address: &str, market_id: &str, chain_id: i32) -> Result<WalletPnL, VyperError> {
        let params = vec![
            ("walletAddress", wallet_address.to_string()),
            ("marketID", market_id.to_string()),
            ("chainID", chain_id.to_string()),
        ];
        self.request(reqwest::Method::GET, "/api/v1/wallet/pnl", Some(&params)).await
    }

    pub async fn get_token_metadata(&self, chain_id: i32, token_mint: &str) -> Result<TokenMetadata, VyperError> {
        let params = vec![
            ("chainID", chain_id.to_string()),
            ("tokenMint", token_mint.to_string()),
        ];
        self.request(reqwest::Method::GET, "/api/v1/token/metadata", Some(&params)).await
    }

    pub async fn get_token_symbol(&self, chain_id: i32, token_mint: &str) -> Result<TokenSymbol, VyperError> {
        let params = vec![
            ("chainID", chain_id.to_string()),
            ("tokenMint", token_mint.to_string()),
        ];
        self.request(reqwest::Method::GET, "/api/v1/token/symbol", Some(&params)).await
    }

    pub async fn get_top_traders(&self, market_id: &str, chain_id: i32) -> Result<Vec<TopTrader>, VyperError> {
        let params = vec![
            ("marketID", market_id.to_string()),
            ("chainID", chain_id.to_string()),
        ];
        self.request(reqwest::Method::GET, "/api/v1/token/top-traders", Some(&params)).await
    }

    pub async fn search_tokens(&self, criteria: &str, chain_id: Option<i32>) -> Result<Vec<TokenSearchResult>, VyperError> {
        let mut params = vec![("criteria", criteria.to_string())];
        if let Some(id) = chain_id {
            params.push(("chainID", id.to_string()));
        }
        self.request(reqwest::Method::GET, "/api/v1/token/search", Some(&params)).await
    }

    pub async fn get_token_pairs(&self, params: TokenPairsParams) -> Result<TokenPairs, VyperError> {
        let query_params: Vec<(&str, String)> = params.into_iter().collect();
        self.request(reqwest::Method::GET, "/api/v1/token/pairs", Some(&query_params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};
    use serde_json::json;

    fn setup_client() -> VyperClient {
        let mock_api_key = "test_api_key";
        let mut client = VyperClient::new(mock_api_key);
        client.base_url = server_url();
        client
    }

    #[tokio::test]
    async fn test_get_chain_ids() {
        let client = setup_client();
        let mock_response = json!({
            "status": "success",
            "message": "Chain IDs retrieved successfully",
            "data": {
                "solana":   900,
				"tron":     1000,
				"ethereum": 1,
				"base":     8453,
				"arbitrum": 42161,
				"bsc":      56,
				"blast":    81457,
            }
        });

        let _m = mock("GET", "/api/v1/chain/ids")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create();

        let result = client.get_chain_ids().await;
        assert!(result.is_ok());
        let chain_ids = result.unwrap();
        assert_eq!(chain_ids.get("ethereum"), Some(&1));
        assert_eq!(chain_ids.get("solana"), Some(&900));
    }

    #[tokio::test]
    async fn test_get_token_ath() {
        let client = setup_client();
        let mock_response = json!({
            "status": "success",
            "message": "Token ATH retrieved successfully",
            "data": {
                "marketCapUsd": 1000000.0,
                "timestamp": 1632825600,
                "tokenLiquidityUsd": 500000.0
            }
        });

        let _m = mock("GET", "/api/v1/token/ath")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("chainID".into(), "1".into()),
                mockito::Matcher::UrlEncoded("marketID".into(), "test-market".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create();

        let result = client.get_token_ath(1, "test-market").await;
        assert!(result.is_ok());
        let token_ath = result.unwrap();
        assert_eq!(token_ath.market_cap_usd, 1000000.0);
        assert_eq!(token_ath.timestamp, 1632825600);
        assert_eq!(token_ath.token_liquidity_usd, 500000.0);
    }

    #[tokio::test]
    async fn test_get_token_market() {
        let client = setup_client();
        
        let mock_response = r#"{
            "status": "success",
            "message": "Token market data retrieved successfully",
            "data": {
                "abused": null,
                "bondingCurvePercentage": null,
                "buyTxnCount": 1000,
                "chainId": 1,
                "contractCreator": "0x1234567890123456789012345678901234567890",
                "createdTimestamp": 1609459200,
                "description": null,
                "freezeAuthority": null,
                "image": null,
                "initialAssetLiquidity": 1000000.0,
                "initialUsdLiquidity": 50000000.0,
                "isMigrated": null,
                "lpBurned": false,
                "lpCreator": "0x0987654321098765432109876543210987654321",
                "marketId": "test-market",
                "metadataUri": null,
                "migratedMarketId": null,
                "migrationState": null,
                "mintAuthority": null,
                "name": "Bitcoin",
                "pooledAsset": 1000000.0,
                "pooledToken": 20.0,
                "priceChangePercent": 2.5,
                "sellTxnCount": 800,
                "symbol": "BTC",
                "telegram": null,
                "tokenLiquidityAsset": 5000000.0,
                "tokenLiquidityUsd": 5000000000.0,
                "tokenMarketCapAsset": 1000000.0,
                "tokenMarketCapUsd": 1000000000000.0,
                "tokenMint": "0x...",
                "tokenPriceAsset": 50000.0,
                "tokenPriceUsd": 50000.0,
                "tokenType": "token",
                "top10HoldingPercent": 25.0,
                "totalSupply": 21000000.0,
                "transactionCount": 1800,
                "twitter": null,
                "volumeAsset": 10000.0,
                "volumeUsd": 500000000.0,
                "website": null
            }
        }"#;
    
        let _m = mock("GET", "/api/v1/token/market/test-market")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("chainID".into(), "1".into()),
                mockito::Matcher::UrlEncoded("interval".into(), "1d".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response)
            .create();
    
        let result = client.get_token_market("test-market", 1, "1d").await;
        if let Err(e) = &result {
            println!("Error: {:?}", e);
        }
        assert!(result.is_ok());
        let token_pair = result.unwrap();
        assert_eq!(token_pair.market_id, "test-market");
        assert_eq!(token_pair.chain_id, 1);
        assert_eq!(token_pair.name, "Bitcoin");
        assert_eq!(token_pair.symbol, "BTC");
    }

    #[tokio::test]
    async fn test_get_token_holders() {
        let client = setup_client();
        let mock_response = json!({
            "status": "success",
            "message": "Token holders retrieved successfully",
            "data": {
                "holders": [
                    {
                        "walletAddress": "0x123...",
                        "tokenHoldings": 100.0,
                        "usdHoldings": 5000000.0,
                        "percentOwned": 0.01
                    }
                ],
                "total_holders": 1000
            }
        });

        let _m = mock("GET", "/api/v1/token/holders")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("marketID".into(), "test-market".into()),
                mockito::Matcher::UrlEncoded("chainID".into(), "1".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create();

        let result = client.get_token_holders("test-market", 1).await;
        assert!(result.is_ok());
        let (holders, total_holders) = result.unwrap();
        assert_eq!(holders.len(), 1);
        assert_eq!(holders[0].wallet_address, "0x123...");
        assert_eq!(total_holders, 1000);
    }

    #[tokio::test]
    async fn test_get_token_markets() {
        let client = setup_client();
        let mock_response = json!({
            "status": "success",
            "message": "Token markets retrieved successfully",
            "data": [
                {
                    "marketCapUsd": 1000000000.0,
                    "marketID": "test-market",
                    "tokenLiquidityUsd": 500000000.0,
                    "tokenType": "token"
                }
            ]
        });

        let _m = mock("GET", "/api/v1/token/markets")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("tokenMint".into(), "0x123...".into()),
                mockito::Matcher::UrlEncoded("chainID".into(), "1".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create();

        let result = client.get_token_markets("0x123...", 1).await;
        assert!(result.is_ok());
        let token_markets = result.unwrap();
        assert_eq!(token_markets.len(), 1);
        assert_eq!(token_markets[0].market_id, "test-market");
    }

    #[tokio::test]
    async fn test_get_wallet_holdings() {
        let client = setup_client();
        let mock_response = json!({
            "status": "success",
            "message": "Wallet holdings retrieved successfully",
            "data": [
                {
                    "marketId": "test-market",
                    "tokenHoldings": 1.5,
                    "tokenSymbol": "BTC",
                    "usdValue": 75000.0
                }
            ]
        });

        let _m = mock("GET", "/api/v1/wallet/holdings")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("walletAddress".into(), "0xabc...".into()),
                mockito::Matcher::UrlEncoded("chainID".into(), "1".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create();

        let result = client.get_wallet_holdings("0xabc...", 1).await;
        assert!(result.is_ok());
        let holdings = result.unwrap();
        assert_eq!(holdings.len(), 1);
        assert_eq!(holdings[0].market_id, "test-market");
    }

    #[tokio::test]
    async fn test_get_wallet_aggregated_pnl() {
        let client = setup_client();
        let mock_response = json!({
            "status": "success",
            "message": "Wallet aggregated PnL retrieved successfully",
            "data": {
                "investedAmount": 100000.0,
                "pnlPercent": 10.0,
                "pnlUsd": 10000.0,
                "soldAmount": 50000.0,
                "tokensTraded": 5,
                "totalPnlPercent": 15.0,
                "totalPnlUsd": 15000.0,
                "unrealizedPnlPercent": 5.0,
                "unrealizedPnlUsd": 5000.0
            }
        });

        let _m = mock("GET", "/api/v1/wallet/aggregated-pnl")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("walletAddress".into(), "0xabc...".into()),
                mockito::Matcher::UrlEncoded("chainID".into(), "1".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create();

        let result = client.get_wallet_aggregated_pnl("0xabc...", 1).await;
        assert!(result.is_ok());
        let pnl = result.unwrap();
        assert_eq!(pnl.invested_amount, 100000.0);
        assert_eq!(pnl.pnl_percent, 10.0);
    }

    #[tokio::test]
    async fn test_get_wallet_pnl() {
        let client = setup_client();
        let mock_response = json!({
            "status": "success",
            "message": "Wallet PnL retrieved successfully",
            "data": {
                "holderSince": 1609459200,
                "investedAmount": 50000.0,
                "investedTxns": 10,
                "pnlPercent": 20.0,
                "pnlUsd": 10000.0,
                "remainingTokens": 0.5,
                "remainingUsd": 25000.0,
                "soldAmount": 35000.0,
                "soldTxns": 5
            }
        });

        let _m = mock("GET", "/api/v1/wallet/pnl")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("walletAddress".into(), "0xabc...".into()),
                mockito::Matcher::UrlEncoded("marketID".into(), "test-market".into()),
                mockito::Matcher::UrlEncoded("chainID".into(), "1".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create();

        let result = client.get_wallet_pnl("0xabc...", "test-market", 1).await;
        assert!(result.is_ok());
        let pnl = result.unwrap();
        assert_eq!(pnl.holder_since, 1609459200);
        assert_eq!(pnl.invested_amount, 50000.0);
    }

    #[tokio::test]
    async fn test_get_token_metadata() {
        let client = setup_client();
        let mock_response = json!({
            "status": "success",
            "message": "Token metadata retrieved successfully",
            "data": {
                "image": "https://example.com/token.png",
                "name": "Example Token",
                "symbol": "EXT",
                "telegram": "https://t.me/exampletoken",
                "twitter": "https://twitter.com/exampletoken",
                "website": "https://exampletoken.com"
            }
        });

        let _m = mock("GET", "/api/v1/token/metadata")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("chainID".into(), "1".into()),
                mockito::Matcher::UrlEncoded("tokenMint".into(), "0x123...".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create();

        let result = client.get_token_metadata(1, "0x123...").await;
        assert!(result.is_ok());
        let metadata = result.unwrap();
        assert_eq!(metadata.name, "Example Token");
        assert_eq!(metadata.symbol, "EXT");
    }

    #[tokio::test]
    async fn test_get_token_symbol() {
        let client = setup_client();
        let mock_response = json!({
            "status": "success",
            "message": "Token symbol retrieved successfully",
            "data": {
                "symbol": "EXT"
            }
        });

        let _m = mock("GET", "/api/v1/token/symbol")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("chainID".into(), "1".into()),
                mockito::Matcher::UrlEncoded("tokenMint".into(), "0x123...".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create();

        let result = client.get_token_symbol(1, "0x123...").await;
        assert!(result.is_ok());
        let symbol = result.unwrap();
        assert_eq!(symbol.symbol, "EXT");
    }

    #[tokio::test]
    async fn test_get_top_traders() {
        let client = setup_client();
        let mock_response = json!({
            "status": "success",
            "message": "Top traders retrieved successfully",
            "data": [{
                "investedAmount_tokens": 100.0,
                "investedAmount_usd": 50000.0,
                "investedTxns": 10,
                "pnlUsd": 10000.0,
                "remainingTokens": 50.0,
                "remainingUsd": 25000.0,
                "soldAmountTokens": 50.0,
                "soldAmountUsd": 35000.0,
                "soldTxns": 5,
                "walletAddress": "0xabc...",
                "walletTag": null
            }]
        });

        let _m = mock("GET", "/api/v1/token/top-traders")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("marketID".into(), "test-market".into()),
                mockito::Matcher::UrlEncoded("chainID".into(), "1".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create();

        let result = client.get_top_traders("test-market", 1).await;
        assert!(result.is_ok());
        let top_traders = result.unwrap();
        assert_eq!(top_traders.len(), 1);
        assert_eq!(top_traders[0].wallet_address, "0xabc...");
    }

    #[tokio::test]
    async fn test_search_tokens() {
        let client = setup_client();
        let mock_response = json!({
            "status": "success",
            "message": "Tokens searched successfully",
            "data": [{
                "chainId": 1,
                "marketId": "test-market",
                "createdTimestamp": 1609459200,
                "name": "Example Token",
                "symbol": "EXT",
                "tokenMint": "0x123...",
                "tokenType": "token",
                "percentChange24h": 5.0,
                "pooledAsset": 1000000.0,
                "tokenLiquidityUsd": 500000.0,
                "tokenMarketCapUsd": 10000000.0,
                "tokenPriceUsd": 1.0,
                "volumeUsd": 1000000.0,
                "image": "https://example.com/token.png",
                "telegram": "https://t.me/exampletoken",
                "twitter": "https://twitter.com/exampletoken",
                "website": "https://exampletoken.com"
            }]
        });

        let _m = mock("GET", "/api/v1/token/search")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("criteria".into(), "EXT".into()),
                mockito::Matcher::UrlEncoded("chainID".into(), "1".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response.to_string())
            .create();

        let result = client.search_tokens("EXT", Some(1)).await;
        assert!(result.is_ok());
        let search_results = result.unwrap();
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].symbol, "EXT");
    }

    #[tokio::test]
    async fn test_get_token_pairs() {
        let client = setup_client();
        let mock_response = r#"{
            "status": "success",
            "message": "Token pairs retrieved successfully",
            "data": {
                "hasNext": false,
                "pairs": [{
                    "abused": null,
                    "bondingCurvePercentage": null,
                    "buyTxnCount": 1000,
                    "chainId": 1,
                    "contractCreator": "0x1234567890123456789012345678901234567890",
                    "createdTimestamp": 1609459200,
                    "description": null,
                    "freezeAuthority": null,
                    "image": "https://example.com/token.png",
                    "initialAssetLiquidity": 1000000.0,
                    "initialUsdLiquidity": 50000000.0,
                    "isMigrated": null,
                    "lpBurned": false,
                    "lpCreator": "0x0987654321098765432109876543210987654321",
                    "marketId": "test-market",
                    "metadataUri": null,
                    "migratedMarketId": null,
                    "migrationState": null,
                    "mintAuthority": null,
                    "name": "Example Token",
                    "pooledAsset": 1000000.0,
                    "pooledToken": 20.0,
                    "priceChangePercent": 2.5,
                    "sellTxnCount": 800,
                    "symbol": "EXT",
                    "telegram": "https://t.me/exampletoken",
                    "tokenLiquidityAsset": 5000000.0,
                    "tokenLiquidityUsd": 5000000000.0,
                    "tokenMarketCapAsset": 1000000.0,
                    "tokenMarketCapUsd": 1000000000000.0,
                    "tokenMint": "0x...",
                    "tokenPriceAsset": 50000.0,
                    "tokenPriceUsd": 50000.0,
                    "tokenType": "token",
                    "top10HoldingPercent": 25.0,
                    "totalSupply": 21000000.0,
                    "transactionCount": 1800,
                    "twitter": "https://twitter.com/exampletoken",
                    "volumeAsset": 10000.0,
                    "volumeUsd": 500000000.0,
                    "website": "https://exampletoken.com"
                }]
            }
        }"#;

        let _m = mock("GET", "/api/v1/token/pairs")
            .match_query(mockito::Matcher::AllOf(vec![
                mockito::Matcher::UrlEncoded("chainIds".into(), "1".into()),
                mockito::Matcher::UrlEncoded("page".into(), "1".into()),
            ]))
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mock_response)
            .create();

        let params = TokenPairsParams {
            chain_ids: Some(vec![1]),
            page: Some(1),
            ..Default::default()
        };

        let result = client.get_token_pairs(params).await;
        assert!(result.is_ok());
        let token_pairs = result.unwrap();
        assert_eq!(token_pairs.has_next, false);
        assert_eq!(token_pairs.pairs.len(), 1);
        assert_eq!(token_pairs.pairs[0].market_id, "EXT-USD");
        assert_eq!(token_pairs.pairs[0].symbol, "EXT");
    }
}