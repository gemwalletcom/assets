# Asset Repository Instructions

This repository stores blockchain and token image assets.

- Token logos belong at `blockchains/<network>/assets/<address>/logo.png`.
- Blockchain logos belong at `blockchains/<network>/logo.png`.
- Images must be PNG or WEBP; never commit JPEG.
- All images must be square: `width == height`.
- Do not resize an image just to change its dimensions; keep the source resolution.
- Compress PNG files before committing.
- EVM token asset directories must use checksum addresses.
- Keep non-EVM token identifiers exactly as provided unless the repository already uses a different canonical form for that asset.
- Do not add unrelated metadata or tokenlist changes unless explicitly requested.
