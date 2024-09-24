use serde::{Deserialize, Serialize};
use std::vec;

#[derive(Debug, Serialize, Deserialize)]
pub struct APIResponse<T> {
    pub status: String,
    pub message: String,
    pub data: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletAggregatedPnL {
    #[serde(rename = "investedAmount")]
    pub invested_amount: f64,
    #[serde(rename = "pnlPercent")]
    pub pnl_percent: f64,
    #[serde(rename = "pnlUsd")]
    pub pnl_usd: f64,
    #[serde(rename = "soldAmount")]
    pub sold_amount: f64,
    #[serde(rename = "tokensTraded")]
    pub tokens_traded: i32,
    #[serde(rename = "totalPnlPercent")]
    pub total_pnl_percent: f64,
    #[serde(rename = "totalPnlUsd")]
    pub total_pnl_usd: f64,
    #[serde(rename = "unrealizedPnlPercent")]
    pub unrealized_pnl_percent: f64,
    #[serde(rename = "unrealizedPnlUsd")]
    pub unrealized_pnl_usd: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletHolding {
    #[serde(rename = "marketId")]
    pub market_id: String,
    #[serde(rename = "tokenHoldings")]
    pub token_holdings: f64,
    #[serde(rename = "tokenSymbol")]
    pub token_symbol: String,
    #[serde(rename = "usdValue")]
    pub usd_value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletPnL {
    #[serde(rename = "holderSince")]
    pub holder_since: i64,
    #[serde(rename = "investedAmount")]
    pub invested_amount: f64,
    #[serde(rename = "investedTxns")]
    pub invested_txns: i32,
    #[serde(rename = "pnlPercent")]
    pub pnl_percent: f64,
    #[serde(rename = "pnlUsd")]
    pub pnl_usd: f64,
    #[serde(rename = "remainingTokens")]
    pub remaining_tokens: f64,
    #[serde(rename = "remainingUsd")]
    pub remaining_usd: f64,
    #[serde(rename = "soldAmount")]
    pub sold_amount: f64,
    #[serde(rename = "soldTxns")]
    pub sold_txns: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TopTrader {
    #[serde(rename = "investedAmount_tokens")]
    pub invested_amount_tokens: f64,
    #[serde(rename = "investedAmount_usd")]
    pub invested_amount_usd: f64,
    #[serde(rename = "investedTxns")]
    pub invested_txns: i32,
    #[serde(rename = "pnlUsd")]
    pub pnl_usd: f64,
    #[serde(rename = "remainingTokens")]
    pub remaining_tokens: f64,
    #[serde(rename = "remainingUsd")]
    pub remaining_usd: f64,
    #[serde(rename = "soldAmountTokens")]
    pub sold_amount_tokens: f64,
    #[serde(rename = "soldAmountUsd")]
    pub sold_amount_usd: f64,
    #[serde(rename = "soldTxns")]
    pub sold_txns: i32,
    #[serde(rename = "walletAddress")]
    pub wallet_address: String,
    #[serde(rename = "walletTag")]
    pub wallet_tag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenSearchResult {
    #[serde(rename = "chainId")]
    pub chain_id: i32,
    #[serde(rename = "marketId")]
    pub market_id: String,
    #[serde(rename = "createdTimestamp")]
    pub created_timestamp: i64,
    pub name: String,
    pub symbol: String,
    #[serde(rename = "tokenMint")]
    pub token_mint: String,
    #[serde(rename = "tokenType")]
    pub token_type: String,
    #[serde(rename = "percentChange24h")]
    pub percent_change_24h: f64,
    #[serde(rename = "pooledAsset")]
    pub pooled_asset: f64,
    #[serde(rename = "tokenLiquidityUsd")]
    pub token_liquidity_usd: f64,
    #[serde(rename = "tokenMarketCapUsd")]
    pub token_market_cap_usd: f64,
    #[serde(rename = "tokenPriceUsd")]
    pub token_price_usd: f64,
    #[serde(rename = "volumeUsd")]
    pub volume_usd: f64,
    pub image: Option<String>,
    pub telegram: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenMarket {
    #[serde(rename = "marketCapUsd")]
    pub market_cap_usd: f64,
    #[serde(rename = "marketID")]
    pub market_id: String,
    #[serde(rename = "tokenLiquidityUsd")]
    pub token_liquidity_usd: f64,
    #[serde(rename = "tokenType")]
    pub token_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenMetadata {
    pub image: Option<String>,
    pub name: String,
    pub symbol: String,
    pub telegram: Option<String>,
    pub twitter: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenSymbol {
    pub symbol: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenHolder {
    #[serde(rename = "percentOwned")]
    pub percent_owned: f64,
    #[serde(rename = "tokenHoldings")]
    pub token_holdings: f64,
    #[serde(rename = "usdHoldings")]
    pub usd_holdings: f64,
    #[serde(rename = "walletAddress")]
    pub wallet_address: String,
    #[serde(rename = "walletTag")]
    pub wallet_tag: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenATH {
    #[serde(rename = "marketCapUsd")]
    pub market_cap_usd: f64,
    pub timestamp: i64,
    #[serde(rename = "tokenLiquidityUsd")]
    pub token_liquidity_usd: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MigrationState {
    #[serde(rename = "durationMinutes")]
    pub duration_minutes: i32,
    pub makers: i32,
    #[serde(rename = "migrationTimestamp")]
    pub migration_timestamp: i64,
    pub volume: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPair {
    pub abused: Option<bool>,
    #[serde(rename = "bondingCurvePercentage")]
    pub bonding_curve_percentage: Option<f64>,
    #[serde(rename = "buyTxnCount")]
    pub buy_txn_count: i32,
    #[serde(rename = "chainId")]
    pub chain_id: i32,
    #[serde(rename = "contractCreator")]
    pub contract_creator: String,
    #[serde(rename = "createdTimestamp")]
    pub created_timestamp: i64,
    pub description: Option<String>,
    #[serde(rename = "freezeAuthority")]
    pub freeze_authority: Option<bool>,
    pub image: Option<String>,
    #[serde(rename = "initialAssetLiquidity")]
    pub initial_asset_liquidity: f64,
    #[serde(rename = "initialUsdLiquidity")]
    pub initial_usd_liquidity: f64,
    #[serde(rename = "isMigrated")]
    pub is_migrated: Option<bool>,
    #[serde(rename = "lpBurned")]
    pub lp_burned: bool,
    #[serde(rename = "lpCreator")]
    pub lp_creator: String,
    #[serde(rename = "marketId")]
    pub market_id: String,
    #[serde(rename = "metadataUri")]
    pub metadata_uri: Option<String>,
    #[serde(rename = "migratedMarketId")]
    pub migrated_market_id: Option<String>,
    #[serde(rename = "migrationState")]
    pub migration_state: Option<MigrationState>,
    #[serde(rename = "mintAuthority")]
    pub mint_authority: Option<bool>,
    pub name: String,
    #[serde(rename = "pooledAsset")]
    pub pooled_asset: f64,
    #[serde(rename = "pooledToken")]
    pub pooled_token: f64,
    #[serde(rename = "priceChangePercent")]
    pub price_change_percent: f64,
    #[serde(rename = "sellTxnCount")]
    pub sell_txn_count: i32,
    pub symbol: String,
    pub telegram: Option<String>,
    #[serde(rename = "tokenLiquidityAsset")]
    pub token_liquidity_asset: f64,
    #[serde(rename = "tokenLiquidityUsd")]
    pub token_liquidity_usd: f64,
    #[serde(rename = "tokenMarketCapAsset")]
    pub token_market_cap_asset: f64,
    #[serde(rename = "tokenMarketCapUsd")]
    pub token_market_cap_usd: f64,
    #[serde(rename = "tokenMint")]
    pub token_mint: String,
    #[serde(rename = "tokenPriceAsset")]
    pub token_price_asset: f64,
    #[serde(rename = "tokenPriceUsd")]
    pub token_price_usd: f64,
    #[serde(rename = "tokenType")]
    pub token_type: String,
    #[serde(rename = "top10HoldingPercent")]
    pub top10_holding_percent: f64,
    #[serde(rename = "totalSupply")]
    pub total_supply: f64,
    #[serde(rename = "transactionCount")]
    pub transaction_count: i32,
    pub twitter: Option<String>,
    #[serde(rename = "volumeAsset")]
    pub volume_asset: f64,
    #[serde(rename = "volumeUsd")]
    pub volume_usd: f64,
    pub website: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPairs {
    #[serde(rename = "hasNext")]
    pub has_next: bool,
    pub pairs: Vec<TokenPair>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChainAction {
    pub signer: String,
    #[serde(rename = "tokenAccount")]
    pub token_account: Option<String>,
    #[serde(rename = "transactionId")]
    pub transaction_id: String,
    #[serde(rename = "tokenMint")]
    pub token_mint: Option<String>,
    #[serde(rename = "marketId")]
    pub market_id: String,
    #[serde(rename = "actionType")]
    pub action_type: String,
    #[serde(rename = "tokenAmount")]
    pub token_amount: f64,
    #[serde(rename = "assetAmount")]
    pub asset_amount: f64,
    #[serde(rename = "tokenPriceUsd")]
    pub token_price_usd: f64,
    #[serde(rename = "tokenPriceAsset")]
    pub token_price_asset: f64,
    #[serde(rename = "swapTotalUsd")]
    pub swap_total_usd: Option<f64>,
    #[serde(rename = "swapTotalAsset")]
    pub swap_total_asset: Option<f64>,
    #[serde(rename = "tokenMarketCapAsset")]
    pub token_market_cap_asset: f64,
    #[serde(rename = "tokenMarketCapUsd")]
    pub token_market_cap_usd: f64,
    #[serde(rename = "tokenLiquidityAsset")]
    pub token_liquidity_asset: f64,
    #[serde(rename = "tokenLiquidityUsd")]
    pub token_liquidity_usd: f64,
    #[serde(rename = "pooledToken")]
    pub pooled_token: f64,
    #[serde(rename = "pooledAsset")]
    pub pooled_asset: f64,
    #[serde(rename = "actionTimestamp")]
    pub action_timestamp: i64,
    #[serde(rename = "bondingCurvePercentage")]
    pub bonding_curve_percentage: Option<f64>,
    #[serde(rename = "botUsed")]
    pub bot_used: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenPairsParams {
    #[serde(rename = "atLeastOneSocial")]
    pub at_least_one_social: Option<bool>,
    #[serde(rename = "buysMax")]
    pub buys_max: Option<i32>,
    #[serde(rename = "buysMin")]
    pub buys_min: Option<i32>,
    #[serde(rename = "chainIds")]
    pub chain_ids: Option<Vec<i32>>,
    #[serde(rename = "freezeAuthDisabled")]
    pub freeze_auth_disabled: Option<bool>,
    #[serde(rename = "initialLiquidityMax")]
    pub initial_liquidity_max: Option<f64>,
    #[serde(rename = "initialLiquidityMin")]
    pub initial_liquidity_min: Option<f64>,
    pub interval: Option<String>,
    #[serde(rename = "liquidityMax")]
    pub liquidity_max: Option<f64>,
    #[serde(rename = "liquidityMin")]
    pub liquidity_min: Option<f64>,
    #[serde(rename = "lpBurned")]
    pub lp_burned: Option<bool>,
    #[serde(rename = "marketCapMax")]
    pub market_cap_max: Option<f64>,
    #[serde(rename = "marketCapMin")]
    pub market_cap_min: Option<f64>,
    #[serde(rename = "mintAuthDisabled")]
    pub mint_auth_disabled: Option<bool>,
    pub page: Option<i32>,
    #[serde(rename = "sellsMax")]
    pub sells_max: Option<i32>,
    #[serde(rename = "sellsMin")]
    pub sells_min: Option<i32>,
    pub sorting: Option<String>,
    #[serde(rename = "swapsMax")]
    pub swaps_max: Option<i32>,
    #[serde(rename = "swapsMin")]
    pub swaps_min: Option<i32>,
    #[serde(rename = "tokenTypes")]
    pub token_types: Option<Vec<String>>,
    #[serde(rename = "top10Holders")]
    pub top10_holders: Option<bool>,
    #[serde(rename = "volumeMax")]
    pub volume_max: Option<f64>,
    #[serde(rename = "volumeMin")]
    pub volume_min: Option<f64>,
}

impl IntoIterator for TokenPairsParams {
    type Item = (&'static str, String);
    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        let mut params = Vec::new();

        if let Some(v) = self.at_least_one_social {
            params.push(("atLeastOneSocial", v.to_string()));
        }
        if let Some(v) = self.buys_max {
            params.push(("buysMax", v.to_string()));
        }
        if let Some(v) = self.buys_min {
            params.push(("buysMin", v.to_string()));
        }
        if let Some(v) = self.chain_ids {
            params.push(("chainIds", v.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(",")));
        }
        if let Some(v) = self.freeze_auth_disabled {
            params.push(("freezeAuthDisabled", v.to_string()));
        }
        if let Some(v) = self.initial_liquidity_max {
            params.push(("initialLiquidityMax", v.to_string()));
        }
        if let Some(v) = self.initial_liquidity_min {
            params.push(("initialLiquidityMin", v.to_string()));
        }
        if let Some(v) = self.interval {
            params.push(("interval", v));
        }
        if let Some(v) = self.liquidity_max {
            params.push(("liquidityMax", v.to_string()));
        }
        if let Some(v) = self.liquidity_min {
            params.push(("liquidityMin", v.to_string()));
        }
        if let Some(v) = self.lp_burned {
            params.push(("lpBurned", v.to_string()));
        }
        if let Some(v) = self.market_cap_max {
            params.push(("marketCapMax", v.to_string()));
        }
        if let Some(v) = self.market_cap_min {
            params.push(("marketCapMin", v.to_string()));
        }
        if let Some(v) = self.mint_auth_disabled {
            params.push(("mintAuthDisabled", v.to_string()));
        }
        if let Some(v) = self.page {
            params.push(("page", v.to_string()));
        }
        if let Some(v) = self.sells_max {
            params.push(("sellsMax", v.to_string()));
        }
        if let Some(v) = self.sells_min {
            params.push(("sellsMin", v.to_string()));
        }
        if let Some(v) = self.sorting {
            params.push(("sorting", v));
        }
        if let Some(v) = self.swaps_max {
            params.push(("swapsMax", v.to_string()));
        }
        if let Some(v) = self.swaps_min {
            params.push(("swapsMin", v.to_string()));
        }
        if let Some(v) = self.token_types {
            params.push(("tokenTypes", v.join(",")));
        }
        if let Some(v) = self.top10_holders {
            params.push(("top10Holders", v.to_string()));
        }
        if let Some(v) = self.volume_max {
            params.push(("volumeMax", v.to_string()));
        }
        if let Some(v) = self.volume_min {
            params.push(("volumeMin", v.to_string()));
        }

        params.into_iter()
    }
}

impl Default for TokenPairsParams {
    fn default() -> Self {
        TokenPairsParams {
            at_least_one_social: None,
            buys_max: None,
            buys_min: None,
            chain_ids: None,
            freeze_auth_disabled: None,
            initial_liquidity_max: None,
            initial_liquidity_min: None,
            interval: None,
            liquidity_max: None,
            liquidity_min: None,
            lp_burned: None,
            market_cap_max: None,
            market_cap_min: None,
            mint_auth_disabled: None,
            page: None,
            sells_max: None,
            sells_min: None,
            sorting: None,
            swaps_max: None,
            swaps_min: None,
            token_types: None,
            top10_holders: None,
            volume_max: None,
            volume_min: None,
        }
    }
}