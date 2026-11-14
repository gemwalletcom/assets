use std::{collections::HashMap, error::Error};

use primitives::AssetBasic;
use reqwest::Client;
use serde::Deserialize;

const COINGECKO_PUBLIC_URL: &str = "https://api.coingecko.com/api/v3";

#[derive(Debug, Deserialize)]
pub struct SearchCoin {
    pub id: String,
}

#[derive(Debug, Deserialize)]
struct SearchResponse {
    #[serde(default)]
    coins: Vec<SearchCoin>,
}

#[derive(Debug, Deserialize)]
pub struct PublicCoin {
    #[serde(default)]
    pub platforms: HashMap<String, Option<String>>,
    #[serde(default)]
    pub image: PublicCoinImage,
}

#[derive(Debug, Default, Deserialize)]
pub struct PublicCoinImage {
    #[serde(default)]
    pub large: Option<String>,
}

pub struct WalletClient {
    http: Client,
    url: String,
}

impl WalletClient {
    pub fn new_with_reqwest_client(http: Client, url: String) -> Self {
        Self { http, url }
    }

    pub async fn get_tag_assets(&self, tag: &str) -> Result<Vec<AssetBasic>, Box<dyn Error + Send + Sync>> {
        let url = format!("{}/v1/assets/search", self.url);
        let response = self.http.get(url).query(&[("query", ""), ("tags", tag)]).send().await?.error_for_status()?;
        Ok(response.json().await?)
    }

    pub async fn search_coins(&self, query: &str) -> Result<Vec<SearchCoin>, Box<dyn Error + Send + Sync>> {
        let response = self.http.get(format!("{COINGECKO_PUBLIC_URL}/search")).query(&[("query", query)]).send().await?.error_for_status()?;
        Ok(response.json::<SearchResponse>().await?.coins)
    }

    pub async fn get_coin_by_contract(&self, platform: &str, token_id: &str) -> Result<PublicCoin, Box<dyn Error + Send + Sync>> {
        self.get_coin(&format!("{platform}/contract/{token_id}")).await
    }

    pub async fn get_coin(&self, id: &str) -> Result<PublicCoin, Box<dyn Error + Send + Sync>> {
        let url = format!("{COINGECKO_PUBLIC_URL}/coins/{}", id.trim_start_matches('/'));
        let response = self
            .http
            .get(url)
            .query(&[("localization", "false"), ("tickers", "false"), ("market_data", "false"), ("community_data", "false"), ("developer_data", "false")])
            .send()
            .await?
            .error_for_status()?;
        Ok(response.json().await?)
    }
}
