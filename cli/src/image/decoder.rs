use crate::error::ImageDownloadError;
use image::{DynamicImage, RgbaImage};
use primitives::ImageType;
use resvg::{tiny_skia, usvg};
use std::error::Error;

pub fn ensure_url_supported(url: &str, supported_types: &[ImageType]) -> std::result::Result<(), ImageDownloadError> {
    if let Some(image_type) = url.split('?').next().and_then(ImageType::from_extension) {
        ensure_supported_type(image_type, supported_types)?;
    }
    Ok(())
}

pub fn decode(url: &str, content_type: Option<&str>, bytes: &[u8], supported_types: &[ImageType], image_size: u32) -> Result<DynamicImage, Box<dyn Error + Send + Sync>> {
    let image_type = image_type(url, content_type, bytes).ok_or(ImageDownloadError::UnsupportedType(None))?;
    decode_image(bytes, image_type, supported_types, Some(image_size))
}

pub fn decode_bytes(bytes: &[u8], supported_types: &[ImageType]) -> Result<DynamicImage, Box<dyn Error + Send + Sync>> {
    let image_type = bytes_image_type(bytes).ok_or(ImageDownloadError::UnsupportedType(None))?;
    decode_image(bytes, image_type, supported_types, None)
}

fn decode_image(bytes: &[u8], image_type: ImageType, supported_types: &[ImageType], svg_size: Option<u32>) -> Result<DynamicImage, Box<dyn Error + Send + Sync>> {
    ensure_supported_type(image_type, supported_types)?;
    if image_type == ImageType::Svg {
        decode_svg(bytes, svg_size).map_err(|_| ImageDownloadError::InvalidImage(image_type).into())
    } else {
        image::load_from_memory(bytes).map_err(|_| ImageDownloadError::InvalidImage(image_type).into())
    }
}

fn ensure_supported_type(image_type: ImageType, supported_types: &[ImageType]) -> std::result::Result<(), ImageDownloadError> {
    if supported_types.contains(&image_type) { Ok(()) } else { Err(ImageDownloadError::UnsupportedType(Some(image_type))) }
}

fn image_type(url: &str, content_type: Option<&str>, bytes: &[u8]) -> Option<ImageType> {
    content_type
        .and_then(content_type_image_type)
        .or_else(|| url.split('?').next().and_then(ImageType::from_extension))
        .or_else(|| bytes_image_type(bytes))
}

fn bytes_image_type(bytes: &[u8]) -> Option<ImageType> {
    ImageType::from_magic_bytes(bytes).or_else(|| bytes.get(..bytes.len().min(128)).and_then(svg_prefix_image_type))
}

fn content_type_image_type(content_type: &str) -> Option<ImageType> {
    let label = content_type.split(';').next()?.rsplit_once('/')?.1;
    if label == "svg+xml" { ImageType::from_label("svg") } else { ImageType::from_label(label) }
}

fn svg_prefix_image_type(bytes: &[u8]) -> Option<ImageType> {
    std::str::from_utf8(bytes).ok().is_some_and(|prefix| prefix.contains("<svg")).then_some(ImageType::Svg)
}

// Renders with the longer side at `size`, so a small SVG is drawn sharp rather than upscaled as a bitmap.
fn decode_svg(bytes: &[u8], size: Option<u32>) -> Result<DynamicImage, Box<dyn Error + Send + Sync>> {
    let tree = usvg::Tree::from_data(bytes, &usvg::Options::default())?;
    let (width, height) = (tree.size().width(), tree.size().height());
    let scale = size.map_or(1.0, |size| size as f32 / width.max(height));
    let mut pixmap = tiny_skia::Pixmap::new((width * scale).ceil() as u32, (height * scale).ceil() as u32).ok_or("invalid svg size")?;
    resvg::render(&tree, tiny_skia::Transform::from_scale(scale, scale), &mut pixmap.as_mut());
    let pixels = pixmap
        .pixels()
        .iter()
        .flat_map(|pixel| {
            let color = pixel.demultiply();
            [color.red(), color.green(), color.blue(), color.alpha()]
        })
        .collect();
    let image = RgbaImage::from_raw(pixmap.width(), pixmap.height(), pixels).ok_or("invalid svg image")?;
    Ok(DynamicImage::ImageRgba8(image))
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::GenericImageView;

    #[test]
    fn test_decode_svg_renders_at_image_size_with_straight_alpha() {
        let svg = br##"<svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 32 32"><rect width="32" height="32" fill="#ff0000" fill-opacity="0.5"/></svg>"##;
        let image = decode("logo.svg", Some("image/svg+xml"), svg, &[ImageType::Svg], 256).unwrap();

        assert_eq!(image.dimensions(), (256, 256));
        assert_eq!(image.get_pixel(128, 128).0, [255, 0, 0, 128]);
    }
}
