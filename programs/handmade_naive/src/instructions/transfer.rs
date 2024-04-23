use anchor_lang::prelude::*;

use crate::{
    error::{IdendityError, TransferError},
    IdAccount, Issuer, WrappedTokenAccount, WrapperAccount,
};

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut, constraint= source_wrapped_account.wrapper_account.key() == destination_wrapped_account.wrapper_account.key())]
    pub source_wrapped_account: Account<'info, WrappedTokenAccount>,
    #[account(constraint = source_wrapped_account.owner == source_owner.key())]
    pub source_owner: Signer<'info>,
    #[account(seeds = [b"identity", source_owner.key().as_ref()], bump)]
    pub idendity_sender: Account<'info, IdAccount>,
    #[account(mut, constraint = destination_wrapped_account.mint.key() == source_wrapped_account.mint.key())]
    pub destination_wrapped_account: Account<'info, WrappedTokenAccount>,
    /// CHECK: The owner of the destination account
    #[account(constraint = destination_wrapped_account.owner == destination_owner.key())]
    pub destination_owner: AccountInfo<'info>,
    #[account(seeds = [b"identity", destination_owner.key().as_ref()], bump)]
    pub idendity_receiver: Account<'info, IdAccount>,
    pub two_auth_signer: Option<Signer<'info>>,
    pub wrapper_account: Account<'info, WrapperAccount>,
}

pub fn _transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
    let source = &mut ctx.accounts.source_wrapped_account;
    let destination = &mut ctx.accounts.destination_wrapped_account;

    let self_transfer = source.key() == destination.key();

    if amount > source.amount {
        return Err(TransferError::InsufficientFunds.into());
    }

    let sender_issuers = &ctx.accounts.idendity_sender.issuers;
    let receiver_issuers = &ctx.accounts.idendity_receiver.issuers;
    let allowed_issuers =  &ctx.accounts.wrapper_account.list_issuer;

    check_idendities(sender_issuers, allowed_issuers)?;
    if !self_transfer{
        check_idendities(receiver_issuers, allowed_issuers)?;
    }


    if self_transfer{ // Otherwise the source and destination are treated as different entities which leads to different amount
        return Ok(());
    }

    source.amount = source.amount.checked_sub(amount).ok_or(TransferError::Overflow)?;
    destination.amount = destination.amount.checked_add(amount).ok_or(TransferError::Overflow)?;

    source.last_tx = Clock::get()?.unix_timestamp;

    Ok(())
}


/*
Check that at least one of the idendity issuer is active and not expired and among the allowed issuers
*/
#[inline(always)]
pub fn check_idendities(user_issuers: &Vec<Issuer>, allowed_issuers: &Vec<Pubkey>) -> Result<()> {
    let current_time = Clock::get()?.unix_timestamp;
    for issuer in user_issuers{
        if allowed_issuers.contains(&issuer.key) && issuer.active && issuer.expires_at > current_time {
            return Ok(()); // At least one of the issuer is valid
        }
    }

    return Err(IdendityError::InvalidIdendity.into());
}
