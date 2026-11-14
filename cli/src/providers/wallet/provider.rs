use std::error::Error;

use async_trait::async_trait;
use coingecko::get_coingecko_platform_id_for_chain;
use primitives::{AssetId, Chain};

use super::{client::WalletClient, mapper};
use crate::providers::{ImageListProvider, ImageProvider, JupiterProvider, model::AssetImage};

const SEARCH_CANDIDATES: usize = 5;

pub struct WalletProvider {
    client: WalletClient,
    jupiter: JupiterProvider,
}

impl WalletProvider {
    pub fn new(client: WalletClient, jupiter: JupiterProvider) -> Self {
        Self { client, jupiter }
    }

    async fn get_tag_images(&self, tag: &str) -> Result<Vec<AssetImage>, Box<dyn Error + Send + Sync>> {
        let assets = self.client.get_tag_assets(tag).await?;
        let missing = mapper::map_missing_assets(assets);
        println!("wallet images tag={tag} missing={}", missing.len());
        let mut images = Vec::new();
        for asset in &missing {
            images.extend(self.resolve_asset(asset).await);
        }
        Ok(images)
    }

    async fn resolve_asset(&self, asset: &mapper::MissingAsset) -> Vec<AssetImage> {
        if asset.chain == Chain::Solana {
            return match self.jupiter.get_asset_images(&asset.token_id).await {
                Ok(images) if !images.is_empty() => images,
                Ok(_) => {
                    skipped(asset, "no jupiter token");
                    Vec::new()
                }
                Err(error) => {
                    skipped(asset, &error.to_string());
                    Vec::new()
                }
            };
        }
        let Some(platform) = get_coingecko_platform_id_for_chain(asset.chain) else {
            skipped(asset, "no coingecko platform");
            return Vec::new();
        };
        if let Ok(coin) = self.client.get_coin_by_contract(platform, &asset.token_id).await {
            return mapper::map_coin_image(asset.chain, &asset.token_id, coin.image.large.unwrap_or_default());
        }
        if !asset.symbol.trim().is_empty()
            && let Ok(coins) = self.client.search_coins(&asset.symbol).await
        {
            for coin in coins.into_iter().take(SEARCH_CANDIDATES) {
                let Ok(info) = self.client.get_coin(&coin.id).await else { continue };
                if mapper::has_contract(&info.platforms, platform, &asset.token_id) {
                    return mapper::map_coin_image(asset.chain, &asset.token_id, info.image.large.unwrap_or_default());
                }
            }
        }
        skipped(asset, "no coingecko coin");
        Vec::new()
    }
}

fn skipped(asset: &mapper::MissingAsset, error: &str) {
    eprintln!("wallet image skipped error={error} chain={} token_id={}", asset.chain, asset.token_id);
}

#[async_trait]
impl ImageProvider for WalletProvider {
    async fn get_asset_images(&self, id: &str) -> Result<Vec<AssetImage>, Box<dyn Error + Send + Sync>> {
        let asset_id = AssetId::new(id).ok_or_else(|| format!("invalid asset id: {id}"))?;
        let token_id = asset_id.token_id.ok_or_else(|| format!("asset id has no token: {id}"))?;
        let asset = mapper::MissingAsset {
            chain: asset_id.chain,
            token_id,
            symbol: String::new(),
        };
        Ok(self.resolve_asset(&asset).await)
    }
}

#[async_trait]
impl ImageListProvider for WalletProvider {
    async fn get_top_asset_images(&self) -> Result<Vec<AssetImage>, Box<dyn Error + Send + Sync>> {
        self.get_tag_images("gainers").await
    }

    async fn get_trending_asset_images(&self) -> Result<Vec<AssetImage>, Box<dyn Error + Send + Sync>> {
        self.get_tag_images("trending").await
    }
}
