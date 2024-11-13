pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

// pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("Gm46kq7uRZyLLELC4vpcPk3eX3PXU14RWovU5a89VUtF");

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

    // TODO: Unwrap tokens

    // Idendity instructions

    pub fn initialize_id(ctx: Context<InitializeId>, id_validity_duration: i64) -> Result<()> {
        idendity::_initialize_id(ctx, id_validity_duration)
    }

    pub fn add_issuer_to_id(ctx: Context<AddIssuer>, id_validity_duration: i64) -> Result<()> {
        idendity::_add_issuer_to_id(ctx, id_validity_duration)
    }

    // TwoAuth instructions

    pub fn initialize_two_auth(
        ctx: Context<InitTwoAuth>,
        two_auth: Option<TwoAuthArgs>,
    ) -> Result<()> {
        two_auth::_initialize_two_auth(ctx, two_auth)
    }

    pub fn update_two_auth(
        ctx: Context<UpdateTwoAuth>,
        two_auth: Option<TwoAuthArgs>,
    ) -> Result<()> {
        two_auth::_update_two_auth(ctx, two_auth)
    }

    // Recovery instructions

    // Transfer instructions

    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        transfer::_transfer(ctx, amount)
    }

    // Bridge with external world

    // pub fn bridge_contract(ctx: Context<Transfer>, instruction_data: &[u8]) -> ProgramResult {
    //     Ok(())
    // }
}
