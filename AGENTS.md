# Asset Repository Instructions

Image assets served by Gem Wallet. This repository holds **data only** — nearly every file is an image. Agents change images and JSON lists here; they do not add application code (see Tooling).

## Repository layout

```
blockchains/<network>/logo.png                        blockchain logo
blockchains/<network>/assets/<token>/logo.png         token logo
blockchains/<network>/assets.json                     generated address list (do not hand-edit)
blockchains/<network>/validators/<address>/logo.png   validator logo
blockchains/<network>/tokenlist.json                  per-chain token list
lists/                                                curated collection images (e.g. coingecko/)
tokenlists/                                           aggregated per-chain token lists
```

Around 70 networks and 16k token logos. `<token>` is an EVM address on EVM chains, or the chain's native identifier otherwise (a mint, a `module::struct` path, a `T...` contract, and so on).

## Image rules

- A logo must be a **PNG or WEBP** — never JPEG — square (`width == height`), and decodable. Those are the `asset_types` in `img-downloader`'s `config.yml`, shared by the checker and the downloader, so changing that list changes what is allowed. The extension is not evidence: several `.png` files in this repository's history have held JPEG bytes, so use `file <path>`, which reports the true type regardless of name.
- Downloaders write 256×256 (`image.size` in `config.yml`). Existing assets vary in size and that is fine — **do not resize an image just to change its dimensions.** A square logo at 128 or 512 is valid; re-encoding it only costs quality and churns the diff.
- Preserve transparency, aspect ratio and colour profiles. When an image has to become square, fit it inside a transparent square canvas — never stretch or crop it.
- Compress images before committing. `img-downloader` encodes everything it fetches, but an image added by hand is easy to commit far larger than it needs to be.
- EVM token directories use **checksum (EIP-55) addresses**.
- Keep non-EVM identifiers exactly as provided, unless the repository already uses a different canonical form for that asset.
- Do not add unrelated metadata or tokenlist changes unless explicitly requested.

## Tooling

All tooling lives in the **[gemwalletcom/wallet](https://github.com/gemwalletcom/wallet)** repository, in the `bin/img-downloader` crate of the `wallet/core` workspace. `.github/actions/setup-wallet` checks that repository out to `wallet/` and installs Rust; CI runs the downloader from `wallet/core`.

If new tooling is needed it should be Rust, like `img-downloader`, or extending that crate. Keeping the tooling in one place is also what keeps this repository free of runtime dependencies.

The crate needs a sibling `wallet` checkout to build. A `justfile` wraps the common case:

```bash
just check-images
```

That builds `img-downloader` in release mode from `../wallet/core` and runs its `check` subcommand over every `blockchains/*/logo.png` and `blockchains/*/assets/*/logo.png`, in batches of 500. It prints the path of each invalid image and exits non-zero if any fail. Validator logos are **not** covered by this command.

To drive the binary directly:

```bash
cd ../wallet/core
cargo build --release --package img-downloader
./target/release/img-downloader check path/to/logo.png ...
```

To fetch or refresh assets from a provider (writes into `blockchains/`):

```bash
cd ../wallet/core
cargo run --package img-downloader -- --source coingecko --mode top --folder ../../blockchains
cargo run --package img-downloader -- --source dexscreener --folder ../../blockchains --id arbitrum_0x...
```

`--source` is one of `coingecko`, `coinmarketcap`, `jupiter` or `dexscreener`. `--mode` is `top` or `trending` and is ignored when `--id` is supplied. Provider API keys come from the environment — see the `[Get Assets]` workflows for the variable names.

## Automation

- `[Get Assets]` workflows (`coingecko.yml`, `jupiter.yml`, …) run on a schedule or on manual dispatch, download images through `img-downloader`, regenerate `assets.json` and commit to `master`. `.github/actions/download-assets` is the shared implementation.
- `[CI] Publish Assets` (`upload.yml`) runs on every push to `master`, syncing `blockchains/` and `lists/` to Cloudflare R2 with `--size-only --delete` and purging the changed paths from cache. **Anything merged to `master` is published to production**, so validate before pushing.

## Generated files

`blockchains/<network>/assets.json` is regenerated from the directory tree by `.github/scripts/generate-assets-list.sh`, which lists the subdirectories containing a `logo.png`. Do not edit it by hand — add the image and let the script or the automation update the list.

## When making image changes

- Check what you are replacing before overwriting: `file <path>` reports the true format, and `just check-images` reports rule violations.
- Prefer the original source over re-encoding. If a logo is visibly upscaled and low quality, look for a vector or larger source from the project, or from an explorer that serves the token by **address** — do not substitute an image found by ticker alone, and never invent a logo.
- Leave images alone when nothing is wrong with them. A change that only re-encodes already-valid images is churn.
- Report images you could not fix confidently rather than guessing.
