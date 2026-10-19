use std::error::Error;

use async_trait::async_trait;
use chain_traits::ChainPerpetual;
use gem_client::ReqwestClient;
use gem_hypercore::rpc::client::HyperCoreClient;

use super::mapper;
use crate::config::HyperliquidConfig;
use crate::providers::{ImageListProvider, ImageProvider, model::AssetImage};

pub struct HyperliquidProvider {
    client: HyperCoreClient<ReqwestClient>,
    config: HyperliquidConfig,
}

impl HyperliquidProvider {
    pub fn new(client: HyperCoreClient<ReqwestClient>, config: HyperliquidConfig) -> Self {
        Self { client, config }
    }
}

#[async_trait]
impl ImageProvider for HyperliquidProvider {
    async fn get_asset_images(&self, coin: &str) -> Result<Vec<AssetImage>, Box<dyn Error + Send + Sync>> {
        Ok(mapper::map_coin_image(coin).into_iter().collect())
    }
}

#[async_trait]
impl ImageListProvider for HyperliquidProvider {
    async fn get_top_asset_images(&self) -> Result<Vec<AssetImage>, Box<dyn Error + Send + Sync>> {
        Ok(mapper::map_top_images(self.client.get_perpetuals_data().await?, self.config.top.count))
    }

    async fn get_trending_asset_images(&self) -> Result<Vec<AssetImage>, Box<dyn Error + Send + Sync>> {
        Ok(mapper::map_newest_images(self.client.get_perpetuals_data().await?, self.config.trending.count))
    }
}
