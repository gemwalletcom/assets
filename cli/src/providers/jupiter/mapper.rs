use ::jupiter::Token;
use primitives::Chain;

use crate::providers::model::AssetImage;

pub(super) fn map_tokens(tokens: Vec<Token>) -> Vec<AssetImage> {
    tokens
        .into_iter()
        .filter(Token::is_verified)
        .filter_map(|token| {
            let image_url = normalize_icon_url(token.icon?);
            Some(AssetImage {
                chain: Chain::Solana,
                token_id: token.id,
                image_url,
            })
        })
        .collect()
}

fn normalize_icon_url(url: String) -> String {
    match url.strip_prefix("https://arweave.app/tx/") {
        Some(tx_id) => format!("https://arweave.net/{tx_id}"),
        None => url,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::model::AssetImage;
    use serde_json::json;

    #[test]
    fn test_map_tokens_rewrites_arweave_app_transaction_icon() {
        let tokens: Vec<Token> = serde_json::from_value(json!([
            { "id": "yieldSOL", "icon": "https://arweave.app/tx/FsvU6_bOaju6cJEmmNTLzy7ZT7gN_YVIvLdTciHb47A", "isVerified": true },
            { "id": "other", "icon": "https://arweave.net/FsvU6_bOaju6cJEmmNTLzy7ZT7gN_YVIvLdTciHb47A", "isVerified": true }
        ]))
        .unwrap();

        let images: Vec<AssetImage> = map_tokens(tokens);
        assert_eq!(images[0].image_url, "https://arweave.net/FsvU6_bOaju6cJEmmNTLzy7ZT7gN_YVIvLdTciHb47A");
        assert_eq!(images[1].image_url, "https://arweave.net/FsvU6_bOaju6cJEmmNTLzy7ZT7gN_YVIvLdTciHb47A");
    }

    #[test]
    fn test_normalize_icon_url_leaves_other_urls() {
        assert_eq!(normalize_icon_url("https://ipfs.io/ipfs/Qmabc".to_string()), "https://ipfs.io/ipfs/Qmabc");
        assert_eq!(normalize_icon_url("https://arweave.app/tx/".to_string()), "https://arweave.net/");
    }
}
