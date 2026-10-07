use std::{path::Path, time::Duration};

use config::{Config, ConfigError, Environment, File};
use primitives::ImageType;
use serde::Deserialize;
use serde_serializers::duration;

#[derive(Debug, Deserialize, Clone)]
pub struct ImgDownloaderConfig {
    pub folder: String,
    #[serde(deserialize_with = "duration::deserialize")]
    pub delay: Duration,
    pub image: ImageConfig,
    pub coingecko: CoingeckoConfig,
    pub coinmarketcap: CoinMarketCapConfig,
    pub jupiter: JupiterConfig,
    pub dexscreener: DexScreenerConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ImageConfig {
    pub size: u32,
    pub asset_types: Vec<ImageType>,
    pub types: Vec<ImageType>,
    pub request: ImageRequestConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ImageRequestConfig {
    #[serde(deserialize_with = "duration::deserialize")]
    pub timeout: Duration,
    pub retries: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CoingeckoConfig {
    pub url: String,
    pub key: KeyConfig,
    pub top: TopConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CoinMarketCapConfig {
    pub url: String,
    pub key: KeyConfig,
    pub top: TopConfig,
    pub trending: TopConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JupiterConfig {
    pub key: KeyConfig,
    pub top: TopConfig,
    pub trending: TrendingConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KeyConfig {
    pub secret: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DexScreenerConfig {
    pub top: TopConfig,
    pub trending: TopConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TopConfig {
    pub count: usize,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TrendingConfig {
    pub count: usize,
    pub interval: String,
}

impl ImgDownloaderConfig {
    pub fn load() -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::from(Path::new("config.yml")))
            .add_source(Environment::default().separator("_").ignore_empty(true))
            .build()?
            .try_deserialize()
    }
}
