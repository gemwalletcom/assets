use std::collections::HashSet;

use gem_hypercore::models::{metadata::perpetual_asset_id, perp_dex::PerpDex, spot::SpotMeta, token::SpotToken};
use primitives::{Asset, AssetId, Chain, perpetual::PerpetualData};

use crate::providers::model::AssetImage;

const COIN_IMAGE_URL: &str = "https://app.hyperliquid.xyz/coins";

pub(super) fn map_coin_image(coin: &str) -> Option<AssetImage> {
    map_image(perpetual_asset_id(coin), coin)
}

pub(super) fn map_top_images(markets: Vec<PerpetualData>, count: usize) -> Vec<AssetImage> {
    let mut markets = listed_markets(markets);
    markets.sort_by(|a, b| b.perpetual.volume_24h.total_cmp(&a.perpetual.volume_24h));
    markets.truncate(count);
    map_images(markets)
}

// Hyperliquid appends markets to each dex's universe as it lists them, so each dex's newest come last.
pub(super) fn map_newest_images(markets: Vec<PerpetualData>, dexes: &[PerpDex], count: usize) -> Vec<AssetImage> {
    let markets = listed_markets(markets);
    let newest = dexes.iter().flat_map(|dex| markets.iter().filter(|market| dex.holds(&market.perpetual.name)).rev().take(count));
    map_images(newest.cloned().collect())
}

// The wallet lists markets that traded in the last 24 hours, and draws a market named after a chain's native coin with that chain's bundled icon.
fn listed_markets(markets: Vec<PerpetualData>) -> Vec<PerpetualData> {
    markets.into_iter().filter(|market| market.perpetual.volume_24h > 0.0 && !has_chain_icon(&market.perpetual.name)).collect()
}

fn has_chain_icon(coin: &str) -> bool {
    Chain::all().into_iter().any(|chain| Asset::from_chain(chain).symbol == coin)
}

// The wallet lists every traded spot token beside its market's coin icon, so spot images need no volume or chain-icon filter.
pub(super) fn map_top_spot_images(meta: SpotMeta, count: usize) -> Vec<AssetImage> {
    map_spot_images(market_tokens(&meta).into_iter(), count)
}

// Hyperliquid appends spot markets to its universe as it lists them, so the newest come last.
pub(super) fn map_newest_spot_images(meta: SpotMeta, count: usize) -> Vec<AssetImage> {
    map_spot_images(market_tokens(&meta).into_iter().rev(), count)
}

fn market_tokens(meta: &SpotMeta) -> Vec<&SpotToken> {
    let mut seen = HashSet::new();
    meta.universe
        .iter()
        .flat_map(|market| market.tokens.iter())
        .filter_map(|index| meta.tokens.get(*index as usize))
        .filter(|token| seen.insert(token.index))
        .collect()
}

fn map_spot_images<'a>(tokens: impl Iterator<Item = &'a SpotToken>, count: usize) -> Vec<AssetImage> {
    tokens.filter_map(map_spot_image).take(count).collect()
}

fn map_spot_image(token: &SpotToken) -> Option<AssetImage> {
    let asset_id = token.asset_id(Chain::HyperCore);
    Some(AssetImage {
        chain: asset_id.chain,
        token_id: asset_id.token_id?,
        image_url: format!("{COIN_IMAGE_URL}/{}.svg", token.name),
    })
}

fn map_images(markets: Vec<PerpetualData>) -> Vec<AssetImage> {
    markets.into_iter().filter_map(|market| map_image(market.perpetual.asset_id, &market.perpetual.name)).collect()
}

fn map_image(asset_id: AssetId, coin: &str) -> Option<AssetImage> {
    Some(AssetImage {
        chain: asset_id.chain,
        token_id: asset_id.token_id?,
        image_url: format!("{COIN_IMAGE_URL}/{coin}.svg"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use gem_hypercore::{models::metadata::HypercoreMetadataResponse, provider::perpetual_mapper::map_perpetuals_data};

    fn markets() -> Vec<PerpetualData> {
        let main: HypercoreMetadataResponse = serde_json::from_str(include_str!("../../../testdata/hyperliquid_meta_and_asset_ctxs.json")).unwrap();
        let xyz: HypercoreMetadataResponse = serde_json::from_str(include_str!("../../../testdata/hyperliquid_xyz_meta_and_asset_ctxs.json")).unwrap();
        [map_perpetuals_data(main, PerpDex::Main), map_perpetuals_data(xyz, PerpDex::Xyz)].concat()
    }

    fn token_ids(images: &[AssetImage]) -> Vec<&str> {
        images.iter().map(|image| image.token_id.as_str()).collect()
    }

    #[test]
    fn test_map_coin_image() {
        assert_eq!(
            map_coin_image("COMP"),
            Some(AssetImage {
                chain: Chain::HyperCore,
                token_id: "perpetual::COMP".to_string(),
                image_url: "https://app.hyperliquid.xyz/coins/COMP.svg".to_string(),
            })
        );
        assert_eq!(
            map_coin_image("xyz:TSLA"),
            Some(AssetImage {
                chain: Chain::HyperCore,
                token_id: "perpetual::xyz:TSLA".to_string(),
                image_url: "https://app.hyperliquid.xyz/coins/xyz:TSLA.svg".to_string(),
            })
        );
    }

    #[test]
    fn test_top_images_rank_listed_markets_of_every_dex_by_volume() {
        let images = map_top_images(markets(), 10);
        assert_eq!(
            token_ids(&images),
            vec!["perpetual::xyz:XYZ100", "perpetual::xyz:TSLA", "perpetual::AAVE", "perpetual::USELESS", "perpetual::COMP", "perpetual::xyz:GLW"]
        );
        assert_eq!(map_top_images(markets(), 1), images[..1]);
        assert_eq!(map_top_images(markets(), 0), vec![]);
    }

    fn spot_meta() -> SpotMeta {
        serde_json::from_value(serde_json::json!({
            "tokens": [
                {"name": "USDC", "weiDecimals": 6, "index": 0, "tokenId": "0x0000000000000000000000000000000000000000000000000000000000000000", "szDecimals": 2},
                {"name": "PURR", "weiDecimals": 0, "index": 1, "tokenId": "0x0000000000000000000000000000000000000000000000000000000000000001", "szDecimals": 0},
                {"name": "HYPE", "weiDecimals": 6, "index": 2, "tokenId": "0x0d01dc56dcaaca66ad901c959b4011ec", "szDecimals": 1}
            ],
            "universe": [
                {"tokens": [1, 0], "name": "PURR/USDC", "index": 0},
                {"tokens": [2, 0], "name": "HYPE/USDC", "index": 1}
            ]
        }))
        .unwrap()
    }

    #[test]
    fn test_top_spot_images_cover_every_market_token_once_in_listing_order() {
        let images = map_top_spot_images(spot_meta(), 10);
        assert_eq!(
            token_ids(&images),
            vec![
                "PURR::0x0000000000000000000000000000000000000000000000000000000000000001::1",
                "USDC::0x0000000000000000000000000000000000000000000000000000000000000000::0",
                "HYPE::0x0d01dc56dcaaca66ad901c959b4011ec::2"
            ]
        );
        assert_eq!(map_top_spot_images(spot_meta(), 2), images[..2]);
        assert_eq!(map_top_spot_images(spot_meta(), 0), vec![]);
    }

    #[test]
    fn test_newest_spot_images_start_from_latest_spot_listing() {
        assert_eq!(
            token_ids(&map_newest_spot_images(spot_meta(), 2)),
            vec!["HYPE::0x0d01dc56dcaaca66ad901c959b4011ec::2", "USDC::0x0000000000000000000000000000000000000000000000000000000000000000::0"]
        );
    }

    #[test]
    fn test_newest_images_start_from_each_dex_latest_listing() {
        assert_eq!(
            token_ids(&map_newest_images(markets(), &[PerpDex::Main, PerpDex::Xyz], 2)),
            vec!["perpetual::USELESS", "perpetual::COMP", "perpetual::xyz:GLW", "perpetual::xyz:TSLA"]
        );
        assert_eq!(token_ids(&map_newest_images(markets(), &[PerpDex::Main], 2)), vec!["perpetual::USELESS", "perpetual::COMP"]);
    }
}
