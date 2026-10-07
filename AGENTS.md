# Asset Repository Instructions

Image assets served by Gem Wallet. Apart from the `cli/` tool, this repository holds **data only** — nearly every file is an image. Agents change images and JSON lists here; they do not add application code outside that tool (see Tooling).

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

- A logo must be a **PNG or WEBP** — never JPEG — square (`width == height`), and decodable. Those are the `asset_types` in `cli/config.yml`, shared by the checker and the downloader, so changing that list changes what is allowed. The extension is not evidence: several `.png` files in this repository's history have held JPEG bytes, so use `file <path>`, which reports the true type regardless of name.
- Downloaders write 256×256 (`image.size` in `config.yml`). Existing assets vary in size and that is fine — **do not resize an image just to change its dimensions.** A square logo at 128 or 512 is valid; re-encoding it only costs quality and churns the diff.
- Preserve transparency, aspect ratio and colour profiles. When an image has to become square, fit it inside a transparent square canvas — never stretch or crop it.
- Compress images before committing. The `cli` downloader encodes everything it fetches, but an image added by hand is easy to commit far larger than it needs to be.
- EVM token directories use **checksum (EIP-55) addresses**.
- Keep non-EVM identifiers exactly as provided, unless the repository already uses a different canonical form for that asset.
- Do not add unrelated metadata or tokenlist changes unless explicitly requested.

## Tooling

All tooling lives in `cli/`, a standalone Rust crate that owns image download, conversion and validation. Provider clients (`coingecko`, `coinmarketcap`, `jupiter`, `dexscreener`), `primitives` and the other shared crates stay in the Core workspace of **[gemwalletcom/wallet](https://github.com/gemwalletcom/wallet)** and come in as git dependencies pinned to a release tag. To pick up wallet changes such as a new chain or a provider mapping, bump the tag on every wallet dependency in `cli/Cargo.toml` (Dependabot's `wallet` group does this weekly), build once in `cli/` to refresh `Cargo.lock`, and commit both. The crate builds with the toolchain pinned in `cli/rust-toolchain.toml`, matching the wallet.

If new tooling is needed it should be Rust, extending `cli`. Keeping the tooling in one crate is also what keeps the rest of this repository free of runtime dependencies.

A `justfile` wraps the common commands:

```bash
just check-images
just download coingecko top
just download dexscreener top arbitrum_0x...
just lint
just test
```

`check-images` builds `cli` in release mode and runs its `check` subcommand over every `blockchains/*/logo.png` and `blockchains/*/assets/*/logo.png`, in batches of 500. It prints the path of each invalid image and exits non-zero if any fail. Validator logos are **not** covered by this command.

The binary reads `config.yml` from the working directory, so run it from `cli/`; downloads go to `../blockchains` unless `--folder` is given:

```bash
cd cli
cargo run -- check ../blockchains/ethereum/logo.png
cargo run -- --source coingecko --mode top
```

`--source` is one of `coingecko`, `coinmarketcap`, `jupiter` or `dexscreener`. `--mode` is `top` or `trending` and is ignored when `--id` is supplied. Provider API keys come from the environment (`COINGECKO_KEY_SECRET`, `COINMARKETCAP_KEY_SECRET`, `JUPITER_KEY_SECRET`); nested `config.yml` values whose keys contain no underscore can be overridden the same way, for example `COINGECKO_TOP_COUNT`.

## Automation

- `[Get Assets]` workflows (`coingecko.yml`, `jupiter.yml`, …) run on a schedule or on manual dispatch, download images through this repository's `cli`, regenerate `assets.json` and commit to `master`. `.github/actions/download-assets` is the shared implementation.
- `[CI] Assets CLI` (`cli.yml`) runs clippy and the tests when `cli/` changes.
- `[CI] Publish Assets` (`upload.yml`) runs on every push to `master`, syncing `blockchains/` and `lists/` to Cloudflare R2 with `--size-only --delete` and purging the changed paths from cache. **Anything merged to `master` is published to production**, so validate before pushing.

## Generated files

`blockchains/<network>/assets.json` is regenerated from the directory tree by `.github/scripts/generate-assets-list.sh`, which lists the subdirectories containing a `logo.png`. Do not edit it by hand — add the image and let the script or the automation update the list.

## When making image changes

- Check what you are replacing before overwriting: `file <path>` reports the true format, and `just check-images` reports rule violations.
- Prefer the original source over re-encoding. If a logo is visibly upscaled and low quality, look for a vector or larger source from the project, or from an explorer that serves the token by **address** — do not substitute an image found by ticker alone, and never invent a logo.
- Leave images alone when nothing is wrong with them. A change that only re-encodes already-valid images is churn.
- Report images you could not fix confidently rather than guessing.
