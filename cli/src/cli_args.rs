use std::path::PathBuf;

const USAGE: &str = "usage: cli [--source coingecko|coinmarketcap|jupiter|dexscreener|hyperliquid] [--mode top|trending] [--id <provider id>] [--folder <path>]
       cli check <image>...

--id takes a CoinGecko coin ID, CoinMarketCap ID or symbol, Jupiter mint, DexScreener chain_token-address or Hyperliquid market, and overrides --mode.";

#[derive(Debug, PartialEq)]
pub enum Command {
    Check { images: Vec<PathBuf> },
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageSource {
    Coingecko,
    Coinmarketcap,
    Jupiter,
    Dexscreener,
    Hyperliquid,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ImageMode {
    Top,
    Trending,
}

#[derive(Debug, PartialEq)]
pub struct Args {
    pub command: Option<Command>,
    pub source: ImageSource,
    pub mode: ImageMode,
    pub folder: Option<String>,
    pub id: String,
}

impl Args {
    pub fn parse() -> Result<Self, String> {
        Self::parse_from(std::env::args().skip(1))
    }

    fn parse_from(arguments: impl IntoIterator<Item = String>) -> Result<Self, String> {
        let mut arguments = arguments.into_iter();
        let mut args = Self {
            command: None,
            source: ImageSource::Coingecko,
            mode: ImageMode::Top,
            folder: None,
            id: String::new(),
        };
        while let Some(argument) = arguments.next() {
            if argument == "check" {
                args.command = Some(Command::Check {
                    images: arguments.map(PathBuf::from).collect(),
                });
                break;
            }
            let value = arguments.next().ok_or_else(|| format!("{argument} needs a value\n\n{USAGE}"))?;
            match argument.as_str() {
                "--source" => args.source = ImageSource::parse(&value).ok_or_else(|| format!("unknown source: {value}\n\n{USAGE}"))?,
                "--mode" => args.mode = ImageMode::parse(&value).ok_or_else(|| format!("unknown mode: {value}\n\n{USAGE}"))?,
                "--folder" | "-f" => args.folder = Some(value),
                "--id" => args.id = value,
                _ => return Err(format!("unknown argument: {argument}\n\n{USAGE}")),
            }
        }
        Ok(args)
    }
}

impl ImageSource {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "coingecko" => Some(Self::Coingecko),
            "coinmarketcap" => Some(Self::Coinmarketcap),
            "jupiter" => Some(Self::Jupiter),
            "dexscreener" => Some(Self::Dexscreener),
            "hyperliquid" => Some(Self::Hyperliquid),
            _ => None,
        }
    }
}

impl ImageMode {
    fn parse(value: &str) -> Option<Self> {
        match value {
            "top" => Some(Self::Top),
            "trending" => Some(Self::Trending),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(arguments: &[&str]) -> Result<Args, String> {
        Args::parse_from(arguments.iter().map(|argument| argument.to_string()))
    }

    #[test]
    fn test_parse_download() {
        assert_eq!(
            parse(&["--source", "hyperliquid", "--mode", "trending", "--id", "xyz:TSLA", "--folder", "out"]).unwrap(),
            Args {
                command: None,
                source: ImageSource::Hyperliquid,
                mode: ImageMode::Trending,
                folder: Some("out".to_string()),
                id: "xyz:TSLA".to_string(),
            }
        );
        assert_eq!(parse(&["--id", ""]).unwrap().source, ImageSource::Coingecko);
    }

    #[test]
    fn test_parse_check() {
        assert_eq!(
            parse(&["check", "a.png", "b.png"]).unwrap().command,
            Some(Command::Check {
                images: vec![PathBuf::from("a.png"), PathBuf::from("b.png")],
            })
        );
    }

    #[test]
    fn test_parse_rejects_unknown_input() {
        assert!(parse(&["--source", "uniswap"]).is_err());
        assert!(parse(&["--mode"]).is_err());
        assert!(parse(&["--limit", "5"]).is_err());
    }
}
