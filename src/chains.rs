//! Comprehensive Chain enum for all GoldRush-supported blockchain networks.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Represents a blockchain network supported by the GoldRush API.
///
/// Each variant maps to a specific chain slug used in API paths.
///
/// # Example
///
/// ```rust
/// use goldrush_sdk::Chain;
///
/// let chain = Chain::EthereumMainnet;
/// assert_eq!(chain.as_ref(), "eth-mainnet");
/// assert_eq!(chain.chain_id(), 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Chain {
    // Ethereum
    #[serde(rename = "eth-mainnet")]
    EthereumMainnet,
    #[serde(rename = "eth-sepolia")]
    EthereumSepolia,
    #[serde(rename = "eth-holesky")]
    EthereumHolesky,

    // Polygon
    #[serde(rename = "matic-mainnet")]
    PolygonMainnet,
    #[serde(rename = "matic-amoy-testnet")]
    PolygonAmoyTestnet,

    // BSC
    #[serde(rename = "bsc-mainnet")]
    BscMainnet,
    #[serde(rename = "bsc-testnet")]
    BscTestnet,

    // Avalanche
    #[serde(rename = "avalanche-mainnet")]
    AvalancheMainnet,
    #[serde(rename = "avalanche-testnet")]
    AvalancheTestnet,

    // Arbitrum
    #[serde(rename = "arbitrum-mainnet")]
    ArbitrumMainnet,
    #[serde(rename = "arbitrum-sepolia")]
    ArbitrumSepolia,
    #[serde(rename = "arbitrum-nova-mainnet")]
    ArbitrumNovaMainnet,

    // Optimism
    #[serde(rename = "optimism-mainnet")]
    OptimismMainnet,
    #[serde(rename = "optimism-sepolia")]
    OptimismSepolia,

    // Base
    #[serde(rename = "base-mainnet")]
    BaseMainnet,
    #[serde(rename = "base-sepolia")]
    BaseSepolia,

    // Fantom
    #[serde(rename = "fantom-mainnet")]
    FantomMainnet,
    #[serde(rename = "fantom-testnet")]
    FantomTestnet,

    // Gnosis
    #[serde(rename = "gnosis-mainnet")]
    GnosisMainnet,
    #[serde(rename = "gnosis-testnet")]
    GnosisTestnet,

    // Cronos
    #[serde(rename = "cronos-mainnet")]
    CronosMainnet,

    // Moonbeam
    #[serde(rename = "moonbeam-mainnet")]
    MoonbeamMainnet,
    #[serde(rename = "moonbeam-moonriver")]
    MoonbeamMoonriver,
    #[serde(rename = "moonbeam-moonbase-alpha")]
    MoonbeamMoonbaseAlpha,

    // Celo
    #[serde(rename = "celo-mainnet")]
    CeloMainnet,
    #[serde(rename = "celo-alfajores")]
    CeloAlfajores,

    // Harmony
    #[serde(rename = "harmony-mainnet")]
    HarmonyMainnet,

    // Aurora
    #[serde(rename = "aurora-mainnet")]
    AuroraMainnet,
    #[serde(rename = "aurora-testnet")]
    AuroraTestnet,

    // Linea
    #[serde(rename = "linea-mainnet")]
    LineaMainnet,
    #[serde(rename = "linea-testnet")]
    LineaTestnet,

    // Scroll
    #[serde(rename = "scroll-mainnet")]
    ScrollMainnet,
    #[serde(rename = "scroll-sepolia-testnet")]
    ScrollSepoliaTestnet,

    // zkSync Era
    #[serde(rename = "zksync-mainnet")]
    ZksyncMainnet,
    #[serde(rename = "zksync-testnet")]
    ZksyncTestnet,

    // Mantle
    #[serde(rename = "mantle-mainnet")]
    MantleMainnet,
    #[serde(rename = "mantle-testnet")]
    MantleTestnet,

    // Polygon zkEVM
    #[serde(rename = "polygon-zkevm-mainnet")]
    PolygonZkevmMainnet,
    #[serde(rename = "polygon-zkevm-testnet")]
    PolygonZkevmTestnet,

    // Zora
    #[serde(rename = "zora-mainnet")]
    ZoraMainnet,

    // Blast
    #[serde(rename = "blast-mainnet")]
    BlastMainnet,
    #[serde(rename = "blast-sepolia")]
    BlastSepolia,

    // Mode
    #[serde(rename = "mode-mainnet")]
    ModeMainnet,
    #[serde(rename = "mode-testnet")]
    ModeTestnet,

    // Lisk
    #[serde(rename = "lisk-mainnet")]
    LiskMainnet,

    // Merlin
    #[serde(rename = "merlin-mainnet")]
    MerlinMainnet,

    // BOB
    #[serde(rename = "bob-mainnet")]
    BobMainnet,

    // Bitcoin
    #[serde(rename = "btc-mainnet")]
    BtcMainnet,

    // Solana
    #[serde(rename = "solana-mainnet")]
    SolanaMainnet,

    // Sei
    #[serde(rename = "sei-mainnet")]
    SeiMainnet,

    // Taiko
    #[serde(rename = "taiko-mainnet")]
    TaikoMainnet,

    // Worldchain
    #[serde(rename = "worldchain-mainnet")]
    WorldchainMainnet,

    // Berachain
    #[serde(rename = "berachain-bartio")]
    BerachainBartio,

    // Immutable zkEVM
    #[serde(rename = "immutable-zkevm-mainnet")]
    ImmutableZkevmMainnet,

    // Apechain
    #[serde(rename = "apechain-mainnet")]
    ApechainMainnet,

    // Ink
    #[serde(rename = "ink-mainnet")]
    InkMainnet,

    // Soneium
    #[serde(rename = "soneium-mainnet")]
    SoneiumMainnet,

    // Abstract
    #[serde(rename = "abstract-mainnet")]
    AbstractMainnet,

    // Unichain
    #[serde(rename = "unichain-mainnet")]
    UnichainMainnet,

    // Sonic
    #[serde(rename = "sonic-mainnet")]
    SonicMainnet,
}

impl Chain {
    /// Returns the chain slug used in API paths.
    pub fn slug(&self) -> &'static str {
        match self {
            // Ethereum
            Chain::EthereumMainnet => "eth-mainnet",
            Chain::EthereumSepolia => "eth-sepolia",
            Chain::EthereumHolesky => "eth-holesky",
            // Polygon
            Chain::PolygonMainnet => "matic-mainnet",
            Chain::PolygonAmoyTestnet => "matic-amoy-testnet",
            // BSC
            Chain::BscMainnet => "bsc-mainnet",
            Chain::BscTestnet => "bsc-testnet",
            // Avalanche
            Chain::AvalancheMainnet => "avalanche-mainnet",
            Chain::AvalancheTestnet => "avalanche-testnet",
            // Arbitrum
            Chain::ArbitrumMainnet => "arbitrum-mainnet",
            Chain::ArbitrumSepolia => "arbitrum-sepolia",
            Chain::ArbitrumNovaMainnet => "arbitrum-nova-mainnet",
            // Optimism
            Chain::OptimismMainnet => "optimism-mainnet",
            Chain::OptimismSepolia => "optimism-sepolia",
            // Base
            Chain::BaseMainnet => "base-mainnet",
            Chain::BaseSepolia => "base-sepolia",
            // Fantom
            Chain::FantomMainnet => "fantom-mainnet",
            Chain::FantomTestnet => "fantom-testnet",
            // Gnosis
            Chain::GnosisMainnet => "gnosis-mainnet",
            Chain::GnosisTestnet => "gnosis-testnet",
            // Cronos
            Chain::CronosMainnet => "cronos-mainnet",
            // Moonbeam
            Chain::MoonbeamMainnet => "moonbeam-mainnet",
            Chain::MoonbeamMoonriver => "moonbeam-moonriver",
            Chain::MoonbeamMoonbaseAlpha => "moonbeam-moonbase-alpha",
            // Celo
            Chain::CeloMainnet => "celo-mainnet",
            Chain::CeloAlfajores => "celo-alfajores",
            // Harmony
            Chain::HarmonyMainnet => "harmony-mainnet",
            // Aurora
            Chain::AuroraMainnet => "aurora-mainnet",
            Chain::AuroraTestnet => "aurora-testnet",
            // Linea
            Chain::LineaMainnet => "linea-mainnet",
            Chain::LineaTestnet => "linea-testnet",
            // Scroll
            Chain::ScrollMainnet => "scroll-mainnet",
            Chain::ScrollSepoliaTestnet => "scroll-sepolia-testnet",
            // zkSync
            Chain::ZksyncMainnet => "zksync-mainnet",
            Chain::ZksyncTestnet => "zksync-testnet",
            // Mantle
            Chain::MantleMainnet => "mantle-mainnet",
            Chain::MantleTestnet => "mantle-testnet",
            // Polygon zkEVM
            Chain::PolygonZkevmMainnet => "polygon-zkevm-mainnet",
            Chain::PolygonZkevmTestnet => "polygon-zkevm-testnet",
            // Zora
            Chain::ZoraMainnet => "zora-mainnet",
            // Blast
            Chain::BlastMainnet => "blast-mainnet",
            Chain::BlastSepolia => "blast-sepolia",
            // Mode
            Chain::ModeMainnet => "mode-mainnet",
            Chain::ModeTestnet => "mode-testnet",
            // Lisk
            Chain::LiskMainnet => "lisk-mainnet",
            // Merlin
            Chain::MerlinMainnet => "merlin-mainnet",
            // BOB
            Chain::BobMainnet => "bob-mainnet",
            // Bitcoin
            Chain::BtcMainnet => "btc-mainnet",
            // Solana
            Chain::SolanaMainnet => "solana-mainnet",
            // Sei
            Chain::SeiMainnet => "sei-mainnet",
            // Taiko
            Chain::TaikoMainnet => "taiko-mainnet",
            // Worldchain
            Chain::WorldchainMainnet => "worldchain-mainnet",
            // Berachain
            Chain::BerachainBartio => "berachain-bartio",
            // Immutable
            Chain::ImmutableZkevmMainnet => "immutable-zkevm-mainnet",
            // Apechain
            Chain::ApechainMainnet => "apechain-mainnet",
            // Ink
            Chain::InkMainnet => "ink-mainnet",
            // Soneium
            Chain::SoneiumMainnet => "soneium-mainnet",
            // Abstract
            Chain::AbstractMainnet => "abstract-mainnet",
            // Unichain
            Chain::UnichainMainnet => "unichain-mainnet",
            // Sonic
            Chain::SonicMainnet => "sonic-mainnet",
        }
    }

    /// Returns the numeric chain ID for the network.
    pub fn chain_id(&self) -> u64 {
        match self {
            Chain::EthereumMainnet => 1,
            Chain::EthereumSepolia => 11155111,
            Chain::EthereumHolesky => 17000,
            Chain::PolygonMainnet => 137,
            Chain::PolygonAmoyTestnet => 80002,
            Chain::BscMainnet => 56,
            Chain::BscTestnet => 97,
            Chain::AvalancheMainnet => 43114,
            Chain::AvalancheTestnet => 43113,
            Chain::ArbitrumMainnet => 42161,
            Chain::ArbitrumSepolia => 421614,
            Chain::ArbitrumNovaMainnet => 42170,
            Chain::OptimismMainnet => 10,
            Chain::OptimismSepolia => 11155420,
            Chain::BaseMainnet => 8453,
            Chain::BaseSepolia => 84532,
            Chain::FantomMainnet => 250,
            Chain::FantomTestnet => 4002,
            Chain::GnosisMainnet => 100,
            Chain::GnosisTestnet => 10200,
            Chain::CronosMainnet => 25,
            Chain::MoonbeamMainnet => 1284,
            Chain::MoonbeamMoonriver => 1285,
            Chain::MoonbeamMoonbaseAlpha => 1287,
            Chain::CeloMainnet => 42220,
            Chain::CeloAlfajores => 44787,
            Chain::HarmonyMainnet => 1666600000,
            Chain::AuroraMainnet => 1313161554,
            Chain::AuroraTestnet => 1313161555,
            Chain::LineaMainnet => 59144,
            Chain::LineaTestnet => 59140,
            Chain::ScrollMainnet => 534352,
            Chain::ScrollSepoliaTestnet => 534351,
            Chain::ZksyncMainnet => 324,
            Chain::ZksyncTestnet => 280,
            Chain::MantleMainnet => 5000,
            Chain::MantleTestnet => 5001,
            Chain::PolygonZkevmMainnet => 1101,
            Chain::PolygonZkevmTestnet => 1442,
            Chain::ZoraMainnet => 7777777,
            Chain::BlastMainnet => 81457,
            Chain::BlastSepolia => 168587773,
            Chain::ModeMainnet => 34443,
            Chain::ModeTestnet => 919,
            Chain::LiskMainnet => 1135,
            Chain::MerlinMainnet => 4200,
            Chain::BobMainnet => 60808,
            Chain::BtcMainnet => 0,
            Chain::SolanaMainnet => 0,
            Chain::SeiMainnet => 1329,
            Chain::TaikoMainnet => 167000,
            Chain::WorldchainMainnet => 480,
            Chain::BerachainBartio => 80084,
            Chain::ImmutableZkevmMainnet => 13371,
            Chain::ApechainMainnet => 33139,
            Chain::InkMainnet => 57073,
            Chain::SoneiumMainnet => 1868,
            Chain::AbstractMainnet => 2741,
            Chain::UnichainMainnet => 130,
            Chain::SonicMainnet => 146,
        }
    }
}

impl fmt::Display for Chain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.slug())
    }
}

impl AsRef<str> for Chain {
    fn as_ref(&self) -> &str {
        self.slug()
    }
}

impl FromStr for Chain {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "eth-mainnet" => Ok(Chain::EthereumMainnet),
            "eth-sepolia" => Ok(Chain::EthereumSepolia),
            "eth-holesky" => Ok(Chain::EthereumHolesky),
            "matic-mainnet" => Ok(Chain::PolygonMainnet),
            "matic-amoy-testnet" => Ok(Chain::PolygonAmoyTestnet),
            "bsc-mainnet" => Ok(Chain::BscMainnet),
            "bsc-testnet" => Ok(Chain::BscTestnet),
            "avalanche-mainnet" => Ok(Chain::AvalancheMainnet),
            "avalanche-testnet" => Ok(Chain::AvalancheTestnet),
            "arbitrum-mainnet" => Ok(Chain::ArbitrumMainnet),
            "arbitrum-sepolia" => Ok(Chain::ArbitrumSepolia),
            "arbitrum-nova-mainnet" => Ok(Chain::ArbitrumNovaMainnet),
            "optimism-mainnet" => Ok(Chain::OptimismMainnet),
            "optimism-sepolia" => Ok(Chain::OptimismSepolia),
            "base-mainnet" => Ok(Chain::BaseMainnet),
            "base-sepolia" => Ok(Chain::BaseSepolia),
            "fantom-mainnet" => Ok(Chain::FantomMainnet),
            "fantom-testnet" => Ok(Chain::FantomTestnet),
            "gnosis-mainnet" => Ok(Chain::GnosisMainnet),
            "gnosis-testnet" => Ok(Chain::GnosisTestnet),
            "cronos-mainnet" => Ok(Chain::CronosMainnet),
            "moonbeam-mainnet" => Ok(Chain::MoonbeamMainnet),
            "moonbeam-moonriver" => Ok(Chain::MoonbeamMoonriver),
            "moonbeam-moonbase-alpha" => Ok(Chain::MoonbeamMoonbaseAlpha),
            "celo-mainnet" => Ok(Chain::CeloMainnet),
            "celo-alfajores" => Ok(Chain::CeloAlfajores),
            "harmony-mainnet" => Ok(Chain::HarmonyMainnet),
            "aurora-mainnet" => Ok(Chain::AuroraMainnet),
            "aurora-testnet" => Ok(Chain::AuroraTestnet),
            "linea-mainnet" => Ok(Chain::LineaMainnet),
            "linea-testnet" => Ok(Chain::LineaTestnet),
            "scroll-mainnet" => Ok(Chain::ScrollMainnet),
            "scroll-sepolia-testnet" => Ok(Chain::ScrollSepoliaTestnet),
            "zksync-mainnet" => Ok(Chain::ZksyncMainnet),
            "zksync-testnet" => Ok(Chain::ZksyncTestnet),
            "mantle-mainnet" => Ok(Chain::MantleMainnet),
            "mantle-testnet" => Ok(Chain::MantleTestnet),
            "polygon-zkevm-mainnet" => Ok(Chain::PolygonZkevmMainnet),
            "polygon-zkevm-testnet" => Ok(Chain::PolygonZkevmTestnet),
            "zora-mainnet" => Ok(Chain::ZoraMainnet),
            "blast-mainnet" => Ok(Chain::BlastMainnet),
            "blast-sepolia" => Ok(Chain::BlastSepolia),
            "mode-mainnet" => Ok(Chain::ModeMainnet),
            "mode-testnet" => Ok(Chain::ModeTestnet),
            "lisk-mainnet" => Ok(Chain::LiskMainnet),
            "merlin-mainnet" => Ok(Chain::MerlinMainnet),
            "bob-mainnet" => Ok(Chain::BobMainnet),
            "btc-mainnet" => Ok(Chain::BtcMainnet),
            "solana-mainnet" => Ok(Chain::SolanaMainnet),
            "sei-mainnet" => Ok(Chain::SeiMainnet),
            "taiko-mainnet" => Ok(Chain::TaikoMainnet),
            "worldchain-mainnet" => Ok(Chain::WorldchainMainnet),
            "berachain-bartio" => Ok(Chain::BerachainBartio),
            "immutable-zkevm-mainnet" => Ok(Chain::ImmutableZkevmMainnet),
            "apechain-mainnet" => Ok(Chain::ApechainMainnet),
            "ink-mainnet" => Ok(Chain::InkMainnet),
            "soneium-mainnet" => Ok(Chain::SoneiumMainnet),
            "abstract-mainnet" => Ok(Chain::AbstractMainnet),
            "unichain-mainnet" => Ok(Chain::UnichainMainnet),
            "sonic-mainnet" => Ok(Chain::SonicMainnet),
            _ => Err(format!("Unknown chain: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_slug() {
        assert_eq!(Chain::EthereumMainnet.slug(), "eth-mainnet");
        assert_eq!(Chain::PolygonMainnet.slug(), "matic-mainnet");
        assert_eq!(Chain::BscMainnet.slug(), "bsc-mainnet");
    }

    #[test]
    fn test_chain_display() {
        assert_eq!(format!("{}", Chain::EthereumMainnet), "eth-mainnet");
        assert_eq!(format!("{}", Chain::ArbitrumMainnet), "arbitrum-mainnet");
    }

    #[test]
    fn test_chain_as_ref() {
        let chain: &str = Chain::BaseMainnet.as_ref();
        assert_eq!(chain, "base-mainnet");
    }

    #[test]
    fn test_chain_from_str() {
        assert_eq!("eth-mainnet".parse::<Chain>().unwrap(), Chain::EthereumMainnet);
        assert_eq!("matic-mainnet".parse::<Chain>().unwrap(), Chain::PolygonMainnet);
        assert!("unknown-chain".parse::<Chain>().is_err());
    }

    #[test]
    fn test_chain_id() {
        assert_eq!(Chain::EthereumMainnet.chain_id(), 1);
        assert_eq!(Chain::PolygonMainnet.chain_id(), 137);
        assert_eq!(Chain::BscMainnet.chain_id(), 56);
        assert_eq!(Chain::ArbitrumMainnet.chain_id(), 42161);
        assert_eq!(Chain::BaseMainnet.chain_id(), 8453);
    }

    #[test]
    fn test_chain_serde() {
        let chain = Chain::EthereumMainnet;
        let json = serde_json::to_string(&chain).unwrap();
        assert_eq!(json, "\"eth-mainnet\"");

        let deserialized: Chain = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Chain::EthereumMainnet);
    }
}
