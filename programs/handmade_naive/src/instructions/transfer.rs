use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::{
    error::{IdendityError, TransferError},
    IdAccount, Issuer, WrappedTokenAccount,
};

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut, seeds=[b"wrapped_token", mint.key().as_ref(), source_owner.key().as_ref()], bump)]
    pub source_wrapped_account: Account<'info, WrappedTokenAccount>,
    pub source_owner: Signer<'info>,
    #[account(seeds = [b"identity", source_owner.key().as_ref()], bump)]
    pub idendity_sender: Account<'info, IdAccount>,
    #[account(mut, seeds=[b"wrapped_token", mint.key().as_ref(), destination_owner.key().as_ref()], bump)]
    pub destination_wrapped_account: Account<'info, WrappedTokenAccount>,
    /// CHECK:
    pub destination_owner: AccountInfo<'info>,
    #[account(seeds = [b"identity", destination_owner.key().as_ref()], bump)]
    pub idendity_receiver: Account<'info, IdAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub two_auth_signer: Option<Signer<'info>>,
}

pub fn _transfer(ctx: Context<Transfer>, amount: u64, decimals: u8) -> Result<()> {
    let source = &mut ctx.accounts.source_wrapped_account;
    let destination = &mut ctx.accounts.destination_wrapped_account;

    if ctx.accounts.mint.decimals != decimals {
        return Err(TransferError::InvalidDecimals.into());
    }

    if amount > source.amount {
        return Err(TransferError::InsufficientFunds.into());
    }

    let sender_issuer = &ctx.accounts.idendity_sender.issuers[0]; // Need to select the main issuer
    let receiver = &ctx.accounts.idendity_receiver.issuers[0]; // Need to select issuer depending on list of accepted issuers

    check_idendities(sender_issuer, receiver)?;

    source.amount -= amount;
    destination.amount += amount;

    source.last_tx = Clock::get()?.unix_timestamp;

    Ok(())
}
#[inline(always)]
pub fn check_idendities(sender: &Issuer, receiver: &Issuer) -> Result<()> {
    if sender.active == false {
        return Err(IdendityError::IdendityNotActive.into());
    }
    if sender.expires_at < Clock::get()?.unix_timestamp {
        return Err(IdendityError::IdendityExpired.into());
    }

    if receiver.active == false {
        return Err(IdendityError::IdendityNotActive.into());
    }
    if receiver.expires_at < Clock::get()?.unix_timestamp {
        return Err(IdendityError::IdendityExpired.into());
    }
    Ok(())
}
