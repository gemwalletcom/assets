# Gem Wallet Assets Info

Forked from [trustwallet/assets](https://github.com/trustwallet/assets)

## Check token images

Install the dependencies with `python3 -m pip install -r scripts/requirements.txt`, then run `python3 scripts/check_token_images.py` from the repository root. Use `--root /path/to/assets` to check another checkout.

The script scans `blockchains/*/assets/*/logo.png` and reports each logo that is not square, or whose contents are JPEG, with its dimensions, format, and path. PNG and WEBP are both accepted. Blockchain and validator logos are excluded. Images are never modified.

Any square size is accepted, so a logo is never resampled just to change its dimensions.

Unreadable images are reported separately. Exit status is `0` when all logos pass, `1` when issues are found, and `2` when no token logos are found. Dimension and format counts can overlap.

The `[CI] Check Token Images` workflow runs the same script on pull requests and pushes that touch token logos, the script, its requirements, or the workflow, and can also be dispatched manually. It installs Python and a pinned Pillow, fails on non-square logos, JPEG content, or unreadable images, and uploads the full report as a `token-image-report` artifact even when validation fails. It never edits or commits images.

License

[MIT](./LICENSE)
