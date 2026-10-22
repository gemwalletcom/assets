use gem_hypercore::models::{metadata::perpetual_asset_id, perp_dex::PerpDex};
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

    #[test]
    fn test_newest_images_start_from_each_dex_latest_listing() {
        assert_eq!(
            token_ids(&map_newest_images(markets(), &[PerpDex::Main, PerpDex::Xyz], 2)),
            vec!["perpetual::USELESS", "perpetual::COMP", "perpetual::xyz:GLW", "perpetual::xyz:TSLA"]
        );
        assert_eq!(token_ids(&map_newest_images(markets(), &[PerpDex::Main], 2)), vec!["perpetual::USELESS", "perpetual::COMP"]);
    }
}
