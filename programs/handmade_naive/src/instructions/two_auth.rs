use anchor_lang::prelude::*;

use crate::{check_idendity_not_recovered, error::TwoAuthError, CircularTimeWindow, IdAccount, TwoAuth, TwoAuthArgs, TwoAuthFunction, TwoAuthParameters, WrapperAccount};

#[derive(Accounts)]
#[instruction(two_auth_args: Option<TwoAuthArgs>)]
pub struct InitTwoAuth<'info> {
    #[account(seeds = [b"identity", owner.key().as_ref()], bump)]
    pub idendity: Account<'info, IdAccount>,
    #[account(seeds=[b"wrapper", approver.key().as_ref()], bump)]
    pub wrapper_account: Account<'info, WrapperAccount>,
    /// CHECK: The approver of the wrapper
    pub approver: UncheckedAccount<'info>,
    #[account(init, seeds=[b"two_auth", wrapper_account.key().as_ref(), owner.key().as_ref()], bump, payer=payer, space=TwoAuth::get_init_len(&two_auth_args))]
    pub two_auth: Account<'info,TwoAuth>,
    pub two_auth_entity: Option<Signer<'info>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(two_auth_args: Option<TwoAuthArgs>)]
pub struct UpdateTwoAuth<'info> {
    #[account(seeds = [b"identity", owner.key().as_ref()], bump)]
    pub idendity: Account<'info, IdAccount>,
    #[account(seeds=[b"wrapper", approver.key().as_ref()], bump)]
    pub wrapper_account: Account<'info, WrapperAccount>,
    /// CHECK: The approver of the wrapper
    pub approver: UncheckedAccount<'info>,
    #[account(mut, seeds=[b"two_auth", wrapper_account.key().as_ref(), owner.key().as_ref()], bump, realloc=TwoAuth::get_init_len(&two_auth_args), realloc::payer=owner, realloc::zero=true)]
    pub two_auth: Account<'info,TwoAuth>,
    pub two_auth_entity: Option<Signer<'info>>,
    pub old_two_auth_entity: Option<Signer<'info>>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}


pub fn _initialize_two_auth(
    ctx: Context<InitTwoAuth>,
    two_auth_args: Option<TwoAuthArgs>
) -> Result<()> {

    let idendity = &ctx.accounts.idendity;
    check_idendity_not_recovered(idendity)?;

    let two_auth = &mut ctx.accounts.two_auth;

    match two_auth_args {
        Some(two_auth_args) => {

            let two_auth_entity = &ctx.accounts.two_auth_entity;
            if two_auth_entity.is_none(){
                return Err(TwoAuthError::NeedTwoAuthApproval.into());
            }
            let time = Clock::get()?.unix_timestamp;
            let two_auth_parameters = TwoAuthParameters{
                functions: two_auth_args.functions.iter().map(|function| init_functions(function, time)).collect(),
                two_auth_entity: two_auth_entity.as_ref().unwrap().key(),
                allowed_issuers: two_auth_args.allowed_issuers.clone(),
            };
            two_auth.two_auth = Some(two_auth_parameters);
        }
        None => {
            two_auth.two_auth = None;
        }
    }

    Ok(())
}



pub fn _update_two_auth(
    ctx: Context<UpdateTwoAuth>,
    two_auth_args: Option<TwoAuthArgs>
) -> Result<()> {

    let idendity = &ctx.accounts.idendity;
    check_idendity_not_recovered(idendity)?;

    let two_auth = &mut ctx.accounts.two_auth;
    let old_two_auth_entity = &ctx.accounts.old_two_auth_entity;

    check_authorization_old_two_auth_entity(old_two_auth_entity, two_auth)?;



    match two_auth_args {
        Some(two_auth_args) => {
            let time = Clock::get()?.unix_timestamp;
            let two_auth_entity = &ctx.accounts.two_auth_entity;
            if two_auth_entity.is_none(){
                return Err(TwoAuthError::NeedTwoAuthApproval.into());
            }
            let two_auth_parameters = TwoAuthParameters{
                functions: two_auth_args.functions.iter().map(|function| init_functions(function, time)).collect(),
                two_auth_entity: two_auth_entity.as_ref().unwrap().key(),
                allowed_issuers: two_auth_args.allowed_issuers.clone(),
            };
            two_auth.two_auth = Some(two_auth_parameters);
        }
        None => {
            two_auth.two_auth = None;
        }
    }

    Ok(())
}

#[inline(always)]
pub fn check_authorization_old_two_auth_entity(
    old_two_auth_entity: &Option<Signer>,
    two_auth: &TwoAuth,
) -> Result<()> {
    if two_auth.two_auth.is_some() {
        let two_auth_parameters = &two_auth.two_auth.as_ref().unwrap();
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

#[inline(always)]
pub fn init_functions(function: &TwoAuthFunction, time: i64) -> TwoAuthFunction{
    match function {
        TwoAuthFunction::CounterResetOnMax { counter: _, max } => {
            TwoAuthFunction::CounterResetOnMax { counter: 0, max: *max }
        }
        TwoAuthFunction::CounterResetOnTime { counter: _, last_reset_time: _, max, duration } => {
            TwoAuthFunction::CounterResetOnTime { counter: 0, last_reset_time: time, max: *max, duration: duration.clone() }
        }
        TwoAuthFunction::CounterWithTimeWindow { window, max } => {
            let duration = window.get_duration();
            TwoAuthFunction::CounterWithTimeWindow { window: CircularTimeWindow::new(duration,time), max: *max }

        }
        _ => function.clone(),
    }
}


// Functions from TwoAuthFunction

/*
    Returns true if there is need for two auth
*/
pub fn apply_two_auth_functions(amount: u64, functions: &mut  Vec<TwoAuthFunction>, time: i64, receiver: Pubkey) -> bool {
    let mut need_two_auth;
    for function in functions.iter_mut() {
        need_two_auth = match_functions(amount, function, time, receiver);
        if need_two_auth.is_some() {
            return need_two_auth.unwrap();
        }
    }
    return false;
}

/*
Returns either a boolean or None if we should continue checking the other functions
*/

pub fn match_functions(amount: u64, function: &mut TwoAuthFunction, time: i64, receiver: Pubkey) -> Option<bool> {
    match function {
        TwoAuthFunction::Always => Some(true),
        TwoAuthFunction::OnMax { max} => if amount >= *max {Some(true)} else {None},
        TwoAuthFunction::CounterResetOnMax { max, counter } => {
            let new_counter = counter.checked_add(amount);
            if new_counter.is_none() || new_counter.is_some() && new_counter.unwrap() >= *max {
                *counter = 0;
                return Some(true);
            }
            return None;
        }
        TwoAuthFunction::CounterResetOnTime { max, duration, counter, last_reset_time } => {
            let diff = time.checked_sub(*last_reset_time);
            if  diff.is_some() && diff.unwrap() > duration.get() as i64{
                *counter = 0;
                *last_reset_time = time;
            }
            let new_counter = counter.checked_add(amount);
            if new_counter.is_some() && new_counter.unwrap() >= *max {
                *counter = new_counter.unwrap();
                return Some(true);
            } else if new_counter.is_none(){
                *counter = 0;
                return Some(true);
            }
            return None;
        }
        TwoAuthFunction::CounterWithTimeWindow { max, window } => {
            window.add(time, amount);
            let count = window.get_count();
            if count >= *max {
                return Some(true);
            }
            return None;
        }
        TwoAuthFunction::DeactivateForUserSpecificWhiteList { white_list } => {
            if white_list.contains(&receiver) {
                return Some(false);
            }
            return None;
        }
    }
}

