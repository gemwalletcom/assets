mod decoder;
mod encoder;

use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
};

use ::image::GenericImageView;
use primitives::ImageType;
use reqwest::{
    Client,
    header::{ACCEPT, CONTENT_TYPE},
};
use tokio::time::{Duration, sleep};

use crate::error::ImageDownloadError;

const IPFS_GATEWAY: &str = "https://gateway.pinata.cloud/ipfs/";

fn ipfs_fallback_url(url: &str) -> Option<String> {
    if url.starts_with(IPFS_GATEWAY) {
        return None;
    }
    let path = url.split_once("/ipfs/")?.1;
    (!path.is_empty()).then(|| format!("{IPFS_GATEWAY}{path}"))
}

pub fn invalid_image_paths(paths: &[PathBuf], supported_types: &[ImageType]) -> Vec<PathBuf> {
    paths.iter().filter(|path| !is_valid_asset_image(path, supported_types)).cloned().collect()
}

fn is_valid_asset_image(path: &Path, supported_types: &[ImageType]) -> bool {
    let Ok(bytes) = fs::read(path) else {
        return false;
    };
    let Ok(image) = decoder::decode_bytes(&bytes, supported_types) else {
        return false;
    };
    let (width, height) = image.dimensions();
    width > 0 && width == height
}

pub async fn download_image(client: &Client, url: &str, image_size: u32, retries: usize, supported_types: &[ImageType]) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
    decoder::ensure_url_supported(url, supported_types)?;

    let mut attempt = 0;
    loop {
        match download_and_convert(client, url, image_size, supported_types).await {
            Ok(bytes) => return Ok(bytes),
            Err(error) if attempt == retries || error.downcast_ref::<ImageDownloadError>().is_some() => return Err(error),
            Err(_) => {
                attempt += 1;
                sleep(Duration::from_millis(250 * attempt as u64)).await;
            }
        }
    }
}

async fn download_and_convert(client: &Client, url: &str, image_size: u32, supported_types: &[ImageType]) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
    let mut result = request_and_convert(client, url, image_size, supported_types).await;
    if result.is_err() {
        if let Some(fallback) = ipfs_fallback_url(url) {
            result = request_and_convert(client, &fallback, image_size, supported_types).await;
        }
    }
    result
}

async fn request_and_convert(client: &Client, url: &str, image_size: u32, supported_types: &[ImageType]) -> Result<Vec<u8>, Box<dyn Error + Send + Sync>> {
    let accept = supported_types.iter().map(|image_type| image_type.mime_type()).collect::<Vec<_>>().join(",");
    let response = client.get(url).header(ACCEPT, accept).send().await?;
    let status = response.status();
    if !status.is_success() {
        return Err(format!("image request failed: {status}").into());
    }

    let content_type = response.headers().get(CONTENT_TYPE).and_then(|value| value.to_str().ok()).map(str::to_owned);
    let bytes = response.bytes().await?;
    let image = decoder::decode(url, content_type.as_deref(), bytes.as_ref(), supported_types, image_size)?;
    encoder::encode_png(image, image_size)
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{DynamicImage, ImageFormat};
    use reqwest::Error as RequestError;
    use std::io::Cursor;

    #[test]
    fn test_is_valid_asset_image() {
        let valid_path = std::env::temp_dir().join(format!("cli-valid-{}.png", std::process::id()));
        let invalid_path = std::env::temp_dir().join(format!("cli-invalid-{}.png", std::process::id()));
        fs::write(&valid_path, encoder::encode_png(DynamicImage::new_rgba8(400, 200), 256).unwrap()).unwrap();
        let mut invalid_bytes = Cursor::new(Vec::new());
        DynamicImage::new_rgba8(400, 200).write_to(&mut invalid_bytes, ImageFormat::Png).unwrap();
        fs::write(&invalid_path, invalid_bytes.into_inner()).unwrap();

        assert_eq!(invalid_image_paths(&[valid_path.clone(), invalid_path.clone()], &[ImageType::Png]), vec![invalid_path.clone()]);

        fs::remove_file(valid_path).unwrap();
        fs::remove_file(invalid_path).unwrap();
    }

    #[tokio::test]
    async fn test_download_image_preserves_request_error() {
        let error = download_image(&Client::new(), "://invalid", 256, 0, &[ImageType::Png]).await.unwrap_err();
        assert_eq!(error.downcast_ref::<RequestError>().map(RequestError::is_builder), Some(true));
    }

    #[test]
    fn test_ipfs_fallback_url() {
        assert_eq!(ipfs_fallback_url("https://ipfs.io/ipfs/Qmabc").as_deref(), Some("https://gateway.pinata.cloud/ipfs/Qmabc"));
        assert_eq!(ipfs_fallback_url("https://cf-ipfs.com/ipfs/Qmabc/logo.png").as_deref(), Some("https://gateway.pinata.cloud/ipfs/Qmabc/logo.png"));
        assert_eq!(ipfs_fallback_url("https://gateway.pinata.cloud/ipfs/Qmabc"), None);
        assert_eq!(ipfs_fallback_url("https://example.com/logo.png"), None);
        assert_eq!(ipfs_fallback_url("https://example.com/ipfs/"), None);
    }

    #[tokio::test]
    #[ignore]
    async fn test_download_image_ipfs_fallback() {
        let bytes = download_image(&Client::new(), "https://cf-ipfs.com/ipfs/bafkreibbh4bttlq3jpsr6mry7rllbojszsjqpeh33uqir6rjsgtqspvty4", 256, 2, &[ImageType::Png, ImageType::Jpeg, ImageType::Svg, ImageType::Webp]).await.unwrap();
        assert!(is_valid_bytes(&bytes));
    }

    fn is_valid_bytes(bytes: &[u8]) -> bool {
        decoder::decode_bytes(bytes, &[ImageType::Png]).is_ok()
    }
}
