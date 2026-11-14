use std::collections::HashMap;

use primitives::{AssetBasic, Chain};

use crate::providers::{mapper::is_native_token, model::AssetImage};

#[derive(Debug, Clone, PartialEq)]
pub struct MissingAsset {
    pub chain: Chain,
    pub token_id: String,
    pub symbol: String,
}

pub(super) fn map_missing_assets(assets: Vec<AssetBasic>) -> Vec<MissingAsset> {
    assets
        .into_iter()
        .filter(|asset| !asset.properties.has_image)
        .filter_map(|asset| {
            let chain = asset.asset.id.chain;
            let token_id = asset.asset.id.token_id?.trim().to_string();
            let placeholder = AssetImage {
                chain,
                token_id: token_id.clone(),
                image_url: String::new(),
            };
            if token_id.is_empty() || is_native_token(&placeholder) {
                return None;
            }
            Some(MissingAsset { chain, token_id, symbol: asset.asset.symbol })
        })
        .collect()
}

pub(super) fn has_contract(platforms: &HashMap<String, Option<String>>, platform: &str, token_id: &str) -> bool {
    platforms
        .get(platform)
        .and_then(|address| address.as_ref())
        .is_some_and(|address| !address.is_empty() && address.eq_ignore_ascii_case(token_id))
}

pub(super) fn map_coin_image(chain: Chain, token_id: &str, image_url: String) -> Vec<AssetImage> {
    if image_url.is_empty() {
        return vec![];
    }
    vec![AssetImage {
        chain,
        token_id: token_id.to_string(),
        image_url,
    }]
}

#[cfg(test)]
mod tests {
    use super::*;
    use primitives::Chain;

    #[test]
    fn test_map_missing_assets_keeps_only_unimaged_tokens() {
        let assets: Vec<AssetBasic> = serde_json::from_str(include_str!("../../../testdata/wallet_search.json")).unwrap();
        assert_eq!(
            map_missing_assets(assets),
            vec![
                MissingAsset {
                    chain: Chain::Solana,
                    token_id: "7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU".to_string(),
                    symbol: "NOLOGO".to_string(),
                },
                MissingAsset {
                    chain: Chain::Ethereum,
                    token_id: "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984".to_string(),
                    symbol: "UNI".to_string(),
                },
            ]
        );
    }

    #[test]
    fn test_has_contract_matches_platform_and_address() {
        let platforms = HashMap::from([("ethereum".to_string(), Some("0x1F9840A85D5aF5bf1D1762F925BDADdC4201F984".to_string())), ("solana".to_string(), None)]);
        assert!(has_contract(&platforms, "ethereum", "0x1f9840a85d5aF5bf1D1762F925BDADdC4201F984"));
        assert!(!has_contract(&platforms, "ethereum", "0x0000000000000000000000000000000000000001"));
        assert!(!has_contract(&platforms, "solana", "token"));
        assert!(!has_contract(&platforms, "arbitrum", "token"));
    }

    #[test]
    fn test_map_coin_image_skips_missing_url() {
        assert_eq!(map_coin_image(Chain::Ethereum, "token", String::new()), vec![]);
        assert_eq!(
            map_coin_image(Chain::Ethereum, "token", "https://example.com/logo.png".to_string()),
            vec![AssetImage {
                chain: Chain::Ethereum,
                token_id: "token".to_string(),
                image_url: "https://example.com/logo.png".to_string(),
            }]
        );
    }
}
