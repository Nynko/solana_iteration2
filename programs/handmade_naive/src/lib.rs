pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("AbyfdCyuhqMm1YqF1exQUaRNph8GXhj495XcXWGLMmoR");

#[program]
pub mod handmade_naive {
    use super::*;

    // Wrapper instructions

    pub fn initialize_wrapper(
        ctx: Context<InitializeWrapper>,
        list_issuer: Vec<Pubkey>,
    ) -> Result<()> {
        wrapper::_initialize_wrapper(ctx, list_issuer)
    }

    pub fn add_issuers_wrapper(ctx: Context<AddWrapperIssuer>, issuer: Pubkey) -> Result<()> {
        wrapper::_add_issuers_wrapper(ctx, issuer)
    }

    pub fn remove_issuer_wrapper(ctx: Context<DeleteWrapperIssuer>) -> Result<()> {
        wrapper::_remove_issuer_wrapper(ctx)
    }

    pub fn initialize_mint(ctx: Context<WrapTokenHolder>) -> Result<()> {
        wrapper::_initialize_mint(ctx)
    }

    pub fn initialize_wrap_account(ctx: Context<InitializeWrappedAccount>) -> Result<()> {
        wrapper::_initialize_wrap_account(ctx)
    }

    pub fn wrap_tokens(ctx: Context<WrapTokens>, amount: u64, decimals: u8) -> Result<()> {
        wrapper::_wrap_tokens(ctx, amount, decimals)
    }

    // Idendity instructions

    pub fn approve_issuer(ctx: Context<ApproveIssuer>) -> Result<()> {
        idendity::_approve_issuer(ctx)
    }

    pub fn revoke_issuer(ctx: Context<RevokeIssuer>) -> Result<()> {
        idendity::_revoke_issuer(ctx)
    }

    pub fn initialize_id(ctx: Context<InitializeId>, id_validity_duration: i64) -> Result<()> {
        idendity::_initialize_id(ctx, id_validity_duration)
    }

    pub fn add_issuer_to_id(ctx: Context<AddIssuer>, id_validity_duration: i64) -> Result<()> {
        idendity::_add_issuer_to_id(ctx, id_validity_duration)
    }

    // Transfer instructions

    pub fn transfer(ctx: Context<Transfer>, amount: u64, decimals: u8) -> Result<()> {
        transfer::_transfer(ctx, amount, decimals)
    }
}
