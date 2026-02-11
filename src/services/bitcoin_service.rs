use crate::Error;
use crate::models::bitcoin::*;
use crate::models::balances::BalancesResponse;
use crate::services::ServiceContext;
use std::sync::Arc;

/// Service for Bitcoin-specific API endpoints.
pub struct BitcoinService {
    ctx: Arc<ServiceContext>,
}

impl BitcoinService {
    pub(crate) fn new(ctx: Arc<ServiceContext>) -> Self {
        Self { ctx }
    }

    /// Get Bitcoin HD wallet balances.
    pub async fn get_bitcoin_hd_wallet_balances(
        &self,
        address: &str,
    ) -> Result<BtcHdWalletResponse, Error> {
        let path = format!("/v1/btc-mainnet/address/{}/hd_wallets/", address);
        self.ctx.send_with_retry(self.ctx.get(&path)).await
    }

    /// Get transactions for a Bitcoin address.
    pub async fn get_transactions_for_btc_address(
        &self,
        address: &str,
    ) -> Result<BtcTransactionsResponse, Error> {
        let path = format!("/v1/btc-mainnet/address/{}/transactions_v3/", address);
        self.ctx.send_with_retry(self.ctx.get(&path)).await
    }

    /// Get Bitcoin non-HD wallet balances.
    pub async fn get_bitcoin_non_hd_wallet_balances(
        &self,
        address: &str,
    ) -> Result<BalancesResponse, Error> {
        let path = format!("/v1/btc-mainnet/address/{}/balances_v2/", address);
        self.ctx.send_with_retry(self.ctx.get(&path)).await
    }
}
