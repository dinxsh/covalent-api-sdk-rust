//! Shared types used across the GoldRush SDK.

use serde::{Deserialize, Serialize};
use std::fmt;

pub use crate::chains::Chain;

/// Quote currency for pricing data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum QuoteCurrency {
    USD,
    CAD,
    EUR,
    SGD,
    INR,
    JPY,
    VND,
    CNY,
    KRW,
    RUB,
    TRY,
    NGN,
    ARS,
    AUD,
    CHF,
    GBP,
    BTC,
    ETH,
}

impl fmt::Display for QuoteCurrency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            QuoteCurrency::USD => "USD",
            QuoteCurrency::CAD => "CAD",
            QuoteCurrency::EUR => "EUR",
            QuoteCurrency::SGD => "SGD",
            QuoteCurrency::INR => "INR",
            QuoteCurrency::JPY => "JPY",
            QuoteCurrency::VND => "VND",
            QuoteCurrency::CNY => "CNY",
            QuoteCurrency::KRW => "KRW",
            QuoteCurrency::RUB => "RUB",
            QuoteCurrency::TRY => "TRY",
            QuoteCurrency::NGN => "NGN",
            QuoteCurrency::ARS => "ARS",
            QuoteCurrency::AUD => "AUD",
            QuoteCurrency::CHF => "CHF",
            QuoteCurrency::GBP => "GBP",
            QuoteCurrency::BTC => "BTC",
            QuoteCurrency::ETH => "ETH",
        };
        write!(f, "{}", s)
    }
}

impl AsRef<str> for QuoteCurrency {
    fn as_ref(&self) -> &str {
        match self {
            QuoteCurrency::USD => "USD",
            QuoteCurrency::CAD => "CAD",
            QuoteCurrency::EUR => "EUR",
            QuoteCurrency::SGD => "SGD",
            QuoteCurrency::INR => "INR",
            QuoteCurrency::JPY => "JPY",
            QuoteCurrency::VND => "VND",
            QuoteCurrency::CNY => "CNY",
            QuoteCurrency::KRW => "KRW",
            QuoteCurrency::RUB => "RUB",
            QuoteCurrency::TRY => "TRY",
            QuoteCurrency::NGN => "NGN",
            QuoteCurrency::ARS => "ARS",
            QuoteCurrency::AUD => "AUD",
            QuoteCurrency::CHF => "CHF",
            QuoteCurrency::GBP => "GBP",
            QuoteCurrency::BTC => "BTC",
            QuoteCurrency::ETH => "ETH",
        }
    }
}

/// Gas event type for gas price queries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GasEventType {
    #[serde(rename = "erc20")]
    Erc20,
    #[serde(rename = "nativetokens")]
    NativeTokens,
    #[serde(rename = "uniswapv3")]
    UniswapV3,
}

impl fmt::Display for GasEventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            GasEventType::Erc20 => "erc20",
            GasEventType::NativeTokens => "nativetokens",
            GasEventType::UniswapV3 => "uniswapv3",
        };
        write!(f, "{}", s)
    }
}

impl AsRef<str> for GasEventType {
    fn as_ref(&self) -> &str {
        match self {
            GasEventType::Erc20 => "erc20",
            GasEventType::NativeTokens => "nativetokens",
            GasEventType::UniswapV3 => "uniswapv3",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quote_currency_display() {
        assert_eq!(QuoteCurrency::USD.to_string(), "USD");
        assert_eq!(QuoteCurrency::EUR.to_string(), "EUR");
        assert_eq!(QuoteCurrency::BTC.to_string(), "BTC");
    }

    #[test]
    fn test_gas_event_type_display() {
        assert_eq!(GasEventType::Erc20.to_string(), "erc20");
        assert_eq!(GasEventType::NativeTokens.to_string(), "nativetokens");
    }
}
