use crate::Error;
use crate::models::approvals::*;
use crate::services::ServiceContext;
use std::sync::Arc;

/// Service for security/approval-related API endpoints.
pub struct SecurityService {
    ctx: Arc<ServiceContext>,
}

impl SecurityService {
    pub(crate) fn new(ctx: Arc<ServiceContext>) -> Self {
        Self { ctx }
    }

    /// Get ERC20 token approvals for an address.
    pub async fn get_approvals(
        &self,
        chain_name: impl AsRef<str>,
        address: &str,
    ) -> Result<ApprovalsResponse, Error> {
        let path = format!("/v1/{}/approvals/{}/", chain_name.as_ref(), address);
        self.ctx.send_with_retry(self.ctx.get(&path)).await
    }

    /// Get NFT approvals for an address.
    pub async fn get_nft_approvals(
        &self,
        chain_name: impl AsRef<str>,
        address: &str,
    ) -> Result<NftApprovalsResponse, Error> {
        let path = format!("/v1/{}/nft/approvals/{}/", chain_name.as_ref(), address);
        self.ctx.send_with_retry(self.ctx.get(&path)).await
    }
}
