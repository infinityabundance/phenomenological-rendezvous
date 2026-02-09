//! CLI scaffolding for offline testing.

use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use serde_json::json;

use crate::matching::{MatchingConfig, Matcher};
use crate::pattern::SubmodalityPattern;
use crate::srt::{pattern_from_srt, SemanticRendezvousToken};

/// Command-line interface for Phenomenological Rendezvous experiments.
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Derive a target pattern from SRT + salt and write JSON output.
    EncodeTarget {
        /// SRT hex string (64 hex chars).
        #[arg(long)]
        srt_hex: String,
        /// Salt as hex string.
        #[arg(long, conflicts_with = "salt_string")]
        salt_hex: Option<String>,
        /// Salt as UTF-8 string.
        #[arg(long)]
        salt_string: Option<String>,
        /// Output file (defaults to stdout).
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Match a stream of measured patterns against a derived target.
    MatchStream {
        /// SRT hex string (64 hex chars).
        #[arg(long)]
        srt_hex: String,
        /// Salt as hex string.
        #[arg(long, conflicts_with = "salt_string")]
        salt_hex: Option<String>,
        /// Salt as UTF-8 string.
        #[arg(long)]
        salt_string: Option<String>,
        /// Matching threshold in normalized space.
        #[arg(long)]
        epsilon: f32,
        /// Number of consecutive samples required to match.
        #[arg(long)]
        window_size: usize,
        /// Input JSONL file with SubmodalityPattern entries. Use "-" for stdin.
        #[arg(long)]
        input: PathBuf,
    },
}

pub fn run() -> Result<(), CliError> {
    let args = CliArgs::parse();

    match args.command {
        Commands::EncodeTarget {
            srt_hex,
            salt_hex,
            salt_string,
            output,
        } => {
            let srt = SemanticRendezvousToken::from_hex(&srt_hex)?;
            let salt = resolve_salt(salt_hex, salt_string)?;
            let pattern = pattern_from_srt(&srt, &salt);
            let json = serde_json::to_string_pretty(&pattern)?;

            match output {
                Some(path) => {
                    let mut file = File::create(path)?;
                    file.write_all(json.as_bytes())?;
                    file.write_all(b"\n")?;
                }
                None => {
                    let mut out = io::stdout().lock();
                    out.write_all(json.as_bytes())?;
                    out.write_all(b"\n")?;
                }
            }
        }
        Commands::MatchStream {
            srt_hex,
            salt_hex,
            salt_string,
            epsilon,
            window_size,
            input,
        } => {
            let srt = SemanticRendezvousToken::from_hex(&srt_hex)?;
            let salt = resolve_salt(salt_hex, salt_string)?;
            let target = pattern_from_srt(&srt, &salt);
            let mut matcher = Matcher::new(MatchingConfig::new(epsilon, window_size));

            let reader: Box<dyn BufRead> = if input.as_os_str() == "-" {
                Box::new(BufReader::new(io::stdin().lock()))
            } else {
                Box::new(BufReader::new(File::open(input)?))
            };

            for (index, line) in reader.lines().enumerate() {
                let line = line?;
                if line.trim().is_empty() {
                    continue;
                }
                let measured: SubmodalityPattern = serde_json::from_str(&line)?;
                let matched = matcher.observe(&measured, &target);
                let output = json!({
                    "index": index,
                    "match": matched,
                });
                println!("{}", output);
            }
        }
    }

    Ok(())
}

fn resolve_salt(salt_hex: Option<String>, salt_string: Option<String>) -> Result<Vec<u8>, CliError> {
    match (salt_hex, salt_string) {
        (Some(hex), None) => parse_hex_bytes(&hex),
        (None, Some(text)) => Ok(text.into_bytes()),
        (None, None) => Err(CliError::MissingSalt),
        (Some(_), Some(_)) => Err(CliError::ConflictingSalt),
    }
}

fn parse_hex_bytes(input: &str) -> Result<Vec<u8>, CliError> {
    let trimmed = input.trim();
    if trimmed.len() % 2 != 0 {
        return Err(CliError::InvalidHexLength(trimmed.len()));
    }
    let mut bytes = Vec::with_capacity(trimmed.len() / 2);
    let mut iter = trimmed.as_bytes().chunks(2);
    while let Some(chunk) = iter.next() {
        let hi = decode_hex_nibble(chunk[0])?;
        let lo = decode_hex_nibble(chunk[1])?;
        bytes.push((hi << 4) | lo);
    }
    Ok(bytes)
}

fn decode_hex_nibble(byte: u8) -> Result<u8, CliError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        _ => Err(CliError::InvalidHexCharacter(byte as char)),
    }
}

#[derive(Debug)]
pub enum CliError {
    MissingSalt,
    ConflictingSalt,
    InvalidHexLength(usize),
    InvalidHexCharacter(char),
    SrtError(crate::srt::SrtParseError),
    Io(std::io::Error),
    Json(serde_json::Error),
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingSalt => write!(f, "missing salt (provide --salt-hex or --salt-string)"),
            Self::ConflictingSalt => {
                write!(f, "provide only one of --salt-hex or --salt-string")
            }
            Self::InvalidHexLength(len) => write!(f, "invalid hex length: {len}"),
            Self::InvalidHexCharacter(ch) => write!(f, "invalid hex character: '{ch}'"),
            Self::SrtError(err) => write!(f, "{err}"),
            Self::Io(err) => write!(f, "{err}"),
            Self::Json(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for CliError {}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<serde_json::Error> for CliError {
    fn from(err: serde_json::Error) -> Self {
        Self::Json(err)
    }
}

impl From<crate::srt::SrtParseError> for CliError {
    fn from(err: crate::srt::SrtParseError) -> Self {
        Self::SrtError(err)
    }
}
