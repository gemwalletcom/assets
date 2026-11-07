use std::{env, error::Error, fs, time::Duration};

use primitives::ImageType;
use serde::Deserialize;
use serde_json::Value;
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
    pub hyperliquid: HyperliquidConfig,
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
pub struct HyperliquidConfig {
    pub url: String,
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
    pub fn load() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let mut value = serde_json::from_str(&fs::read_to_string("config.json")?)?;
        override_values(&mut value, "", &|name| env::var(name).ok());
        Ok(serde_json::from_value(value)?)
    }
}

// The environment variable named after a value's path overrides it, such as COINGECKO_TOP_COUNT for coingecko.top.count; an empty one is ignored.
fn override_values(value: &mut Value, path: &str, lookup: &impl Fn(&str) -> Option<String>) {
    if let Value::Object(fields) = value {
        for (key, field) in fields {
            let path = if path.is_empty() { key.to_uppercase() } else { format!("{path}_{}", key.to_uppercase()) };
            override_values(field, &path, lookup);
        }
    } else if let Some(text) = lookup(path).filter(|text| !text.is_empty()) {
        *value = match value {
            Value::Number(_) => text.parse().map(Value::Number).unwrap_or(Value::String(text)),
            _ => Value::String(text),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_downloads_every_asset_type() {
        let config = ImgDownloaderConfig::load().unwrap();

        assert!(config.image.asset_types.iter().all(|image_type| config.image.types.contains(image_type)));
    }

    #[test]
    fn test_override_values_from_environment() {
        let mut value = serde_json::json!({"coingecko": {"key": {"secret": ""}, "top": {"count": 250}}, "delay": "1s"});
        let lookup = |name: &str| match name {
            "COINGECKO_KEY_SECRET" => Some("key".to_string()),
            "COINGECKO_TOP_COUNT" => Some("1500".to_string()),
            "DELAY" => Some(String::new()),
            _ => None,
        };

        override_values(&mut value, "", &lookup);

        assert_eq!(value, serde_json::json!({"coingecko": {"key": {"secret": "key"}, "top": {"count": 1500}}, "delay": "1s"}));
    }
}
