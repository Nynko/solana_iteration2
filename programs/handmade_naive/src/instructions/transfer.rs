use anchor_lang::prelude::*;

use crate::{
    check_idendity_not_recovered, error::{IdendityError, TransferError, TwoAuthError}, two_auth, IdAccount, Issuer, TwoAuth, TwoAuthParameters, WrappedTokenAccount, WrapperAccount
};

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut, has_one= wrapper_account, constraint= source_wrapped_account.wrapper_account.key() == destination_wrapped_account.wrapper_account.key())]
    pub source_wrapped_account: Account<'info, WrappedTokenAccount>,
    #[account(constraint = source_wrapped_account.owner == source_owner.key())]
    pub source_owner: Signer<'info>,
    #[account(seeds = [b"identity", source_owner.key().as_ref()], bump)]
    pub idendity_sender: Account<'info, IdAccount>,
    #[account(mut, seeds=[b"two_auth", wrapper_account.key().as_ref(), source_owner.key().as_ref()], bump)]
    pub two_auth: Account<'info,TwoAuth>,
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

    check_idendity_not_recovered(&ctx.accounts.idendity_sender)?;
    if !self_transfer{
        check_idendity_not_recovered(&ctx.accounts.idendity_receiver)?;
    }
    let two_auth = &mut ctx.accounts.two_auth.two_auth;
    let two_auth_signer = &ctx.accounts.two_auth_signer;

    let current_time = Clock::get()?.unix_timestamp;

    if !self_transfer{
        check_two_auth(two_auth, two_auth_signer, amount, current_time, ctx.accounts.idendity_receiver.key())?;
    }
    let sender_issuers = &ctx.accounts.idendity_sender.issuers;
    let receiver_issuers = &ctx.accounts.idendity_receiver.issuers;
    let allowed_issuers =  &ctx.accounts.wrapper_account.list_issuer;


    check_idendities(sender_issuers, allowed_issuers,current_time)?;
    if !self_transfer{
        check_idendities(receiver_issuers, allowed_issuers,current_time)?;
    }


    source.last_tx = current_time;

    if self_transfer{ // Otherwise the source and destination are treated as different entities which leads to different amount
        return Ok(());
    }

    source.amount = source.amount.checked_sub(amount).ok_or(TransferError::InsufficientFunds)?;
    destination.amount = destination.amount.checked_add(amount).ok_or(TransferError::Overflow)?;

    Ok(())
}


/*
Check that at least one of the idendity issuer is active and not expired and among the allowed issuers
*/
#[inline(always)]
pub fn check_idendities(user_issuers: &Vec<Issuer>, allowed_issuers: &Vec<Pubkey>, current_time: i64) -> Result<()> {
    for issuer in user_issuers{
        if allowed_issuers.contains(&issuer.key) && issuer.active && issuer.expires_at > current_time {
            return Ok(()); // At least one of the issuer is valid
        }
    }

    return Err(IdendityError::InvalidIdendity.into());
}


#[inline(always)]
pub fn check_two_auth(two_auth: &mut Option<TwoAuthParameters>, two_auth_signer: &Option<Signer>, amount: u64, current_time: i64, receiver: Pubkey) -> Result<()> {
    if two_auth.is_some() {
        let two_auth_parameters = two_auth.as_mut().unwrap();
        let functions  = &mut two_auth_parameters.functions;

        match two_auth_signer {
            Some(signer) => {
                if two_auth_parameters.two_auth_entity != signer.key() {
                    return Err(TwoAuthError::WrongApproval.into());
                } 
                // if we have a proper two auth signature, no need to check if the two auth is needed 
            }
            None => {if two_auth::apply_two_auth_functions(amount, functions, current_time, receiver) {
                        return Err(TwoAuthError::NeedTwoAuthApproval.into());
                        }
                        return Ok(()); // No need for two auth
                    }
        }
    }
    Ok(())
}