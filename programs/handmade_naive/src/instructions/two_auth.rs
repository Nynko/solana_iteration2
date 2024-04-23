use anchor_lang::prelude::*;
use anchor_spl::token_interface::Mint;

use crate::{check_idendity_not_recovered, error::TwoAuthError, IdAccount, TwoAuthFunction, TwoAuthParameters, WrappedTokenAccount, WrapperAccount};



#[derive(Accounts)]
#[instruction(functions: Vec<TwoAuthFunction>, allowed_issuers: Vec<Pubkey>)]
pub struct UpdateTwoAuth<'info> {
    #[account(seeds = [b"identity", owner.key().as_ref()], bump)]
    pub idendity: Account<'info, IdAccount>,
    #[account(seeds=[b"wrapper", approver.key().as_ref()], bump)]
    pub wrapper_account: Account<'info, WrapperAccount>,
    /// CHECK: The approver of the wrapper
    pub approver: UncheckedAccount<'info>,
    #[account(mut, has_one= owner, has_one=wrapper_account, has_one = mint, realloc=WrappedTokenAccount::get_init_len(Some((&functions , &allowed_issuers))), realloc::payer=owner, realloc::zero=false)]
    pub user_wrapped_token_account: Account<'info, WrappedTokenAccount>,
    pub two_auth_entity: Signer<'info>,
    pub old_two_auth_entity: Option<Signer<'info>>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct RemoveTwoAuth<'info> {
    #[account(seeds = [b"identity", owner.key().as_ref()], bump)]
    pub idendity: Account<'info, IdAccount>,
    #[account(seeds=[b"wrapper", approver.key().as_ref()], bump)]
    pub wrapper_account: Account<'info, WrapperAccount>,
    /// CHECK: The approver of the wrapper
    pub approver: UncheckedAccount<'info>,
    #[account(mut, has_one= owner, has_one=wrapper_account, has_one = mint, realloc=WrappedTokenAccount::get_init_len(None), realloc::payer=owner, realloc::zero=true)]
    pub user_wrapped_token_account: Account<'info, WrappedTokenAccount>,
    pub two_auth_entity: Signer<'info>,
    pub old_two_auth_entity: Option<Signer<'info>>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}



pub fn _update_two_auth(
    ctx: Context<UpdateTwoAuth>,
    functions: Vec<TwoAuthFunction>,
    allowed_issuers: Vec<Pubkey>,
) -> Result<()> {

    let idendity = &ctx.accounts.idendity;
    check_idendity_not_recovered(idendity)?;

    let w_token_account = &mut ctx.accounts.user_wrapped_token_account;
    let old_two_auth_entity = &ctx.accounts.old_two_auth_entity;

    check_authorization_old_two_auth_entity(old_two_auth_entity, w_token_account)?;

    let two_auth_parameters = TwoAuthParameters{
        functions: functions.clone(),
        two_auth_entity: ctx.accounts.two_auth_entity.key(),
        allowed_issuers: allowed_issuers.clone(),
    };

    w_token_account.two_auth = Some(two_auth_parameters);

    Ok(())
}

#[inline(always)]
pub fn check_authorization_old_two_auth_entity(
    old_two_auth_entity: &Option<Signer>,
    w_token_account: &WrappedTokenAccount,
) -> Result<()> {
    if w_token_account.two_auth.is_some() {
        let two_auth_parameters = &w_token_account.two_auth.as_ref().unwrap();
        match old_two_auth_entity {
            Some(old_auth_entity) => {
                if two_auth_parameters.two_auth_entity != old_auth_entity.key() {
                    return Err(TwoAuthError::NotAuthorized.into());
                }
            }
            None => {return Err(TwoAuthError::NeedTwoAuthApproval.into());}
        }
    }
    Ok(())
}



pub fn _remove_two_auth(
    ctx: Context<RemoveTwoAuth>,
) -> Result<()> {

    let idendity = &ctx.accounts.idendity;
    check_idendity_not_recovered(idendity)?;

    let w_token_account = &mut ctx.accounts.user_wrapped_token_account;
    let old_two_auth_entity = &ctx.accounts.old_two_auth_entity;

    check_authorization_old_two_auth_entity(old_two_auth_entity, w_token_account)?;

    w_token_account.two_auth = None;

    Ok(())
}


// Functions from TwoAuthFunction

/*
    Returns true if there is need for two auth
*/
pub fn apply_two_auth_functions(amount: u64, functions: &Vec<TwoAuthFunction>) -> bool {

    for function in functions.iter() {
        if match_functions(amount, function) {
            return true;
        }
    }
    return false;
}

pub fn match_functions(amount: u64, function: &TwoAuthFunction) -> bool {
    match function {
        TwoAuthFunction::Always => true,
        TwoAuthFunction::OnMax { max } => amount >= *max,
        _ => true,
    }
}

