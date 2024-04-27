use anchor_lang::prelude::*;

use crate::{RecoveryAuthorities, RecoveryAuthority};


#[derive(Accounts)]
#[instruction(recovery_delegates : Vec<RecoveryAuthority>)]
pub struct InitializeRecovery<'info> {
    #[account(init, seeds = [b"recovery", owner.key().as_ref()], bump, payer = payer, space = RecoveryAuthorities::get_init_len(&recovery_delegates))]
    pub recovery_authority: Account<'info, RecoveryAuthorities>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(seeds=[b"wrapper", approver.key().as_ref()], bump)]
    pub wrapper_account: Account<'info, WrapperAccount>,
    /// CHECK: The approver of the wrapper
    pub approver: UncheckedAccount<'info>,
    #[account(mut, seeds=[b"wrapped_token", wrapper_account.key().as_ref(), mint.key().as_ref(), owner.key().as_ref()], bump, has_one= owner, has_one=wrapper_account, has_one = mint)]
    pub user_wrapped_token_account: Account<'info, WrappedTokenAccount>,
    #[account(constraint = owner.key() == user_wrapped_token_account.owner)]
    pub owner: Signer<'info>,
    pub two_auth_entity: Option<Signer<'info>>,
    #[account(mut, seeds=[b"two_auth", wrapper_account.key().as_ref(), owner.key().as_ref()], bump)]
    pub two_auth: Account<'info,TwoAuth>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct RecoverAccount<'info> {
    #[account(init, seeds = [b"recovery", owner.key().as_ref()], bump, payer = payer, space = RecoveryAuthorities::get_init_len(&recovery_delegates))]
    pub recovery_authority: Account<'info, RecoveryAuthorities>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(seeds=[b"wrapper", approver.key().as_ref()], bump)]
    pub wrapper_account: Account<'info, WrapperAccount>,
    /// CHECK: The approver of the wrapper
    pub approver: UncheckedAccount<'info>,
    #[account(mut, seeds=[b"wrapped_token", wrapper_account.key().as_ref(), mint.key().as_ref(), owner.key().as_ref()], bump, has_one= owner, has_one=wrapper_account, has_one = mint)]
    pub user_wrapped_token_account: Account<'info, WrappedTokenAccount>,
    #[account(constraint = owner.key() == user_wrapped_token_account.owner)]
    pub owner: Signer<'info>,
    pub two_auth_entity: Option<Signer<'info>>,
    #[account(mut, seeds=[b"two_auth", wrapper_account.key().as_ref(), owner.key().as_ref()], bump)]
    pub two_auth: Account<'info,TwoAuth>,
    #[account(seeds = [b"identity", owner.key().as_ref()], bump)]
    pub idendity: Account<'info, IdAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
}

// #[derive(Accounts)]
// pub struct RecoverAccount<'info> {
//     #[account(mut)]
//     pub new_owner: Signer<'info>, // Does it needs to be a signer?
//     #[account(mut, token::authority = new_owner.key())]
//     pub new_token_account: InterfaceAccount<'info, TokenAccount>,
//     #[account(seeds = [b"recovery_authority", owner.key().as_ref()], bump)]
//     pub recovery_authority: Account<'info, RecoveryAuthority>,
//     #[account(seeds = [b"last_tx", owner.key().as_ref()], bump)]
//     pub last_tx: Account<'info, LastTx>,
//     #[account(mut, seeds = [b"identity", token_account.key().as_ref()], bump, realloc = 80 + 49 * idendity.issuers.len() + 32, realloc::payer = new_owner, realloc::zero= false)]
//     pub idendity: Account<'info, IdAccount>,
//     /// CHECK: Account to recover
//     pub owner: AccountInfo<'info>,
//     #[account(mut, seeds = [b"mint"], bump)]
//     pub mint: InterfaceAccount<'info, Mint>,
//     #[account(mut, token::authority = owner.key())]
//     pub token_account: InterfaceAccount<'info, TokenAccount>,
//     pub token_program: Interface<'info, TokenInterface>,
//     pub system_program: Program<'info, System>,
// }



pub fn _initialize_recovery(
    ctx: Context<InitializeRecovery>,
    recovery_delegates: Vec<RecoveryAuthority>,
) -> Result<()> {
    let last_tx = &mut ctx.accounts.last_tx;
    last_tx.last_tx_timestamp = Clock::get()?.unix_timestamp;

    let recovery_authority = &mut ctx.accounts.recovery_authority;
    recovery_authority.authorities = recovery_delegates;
    recovery_authority.minimum_signatures = minimum_signatures;

    Ok(())
}



/// NOTES : Recovery should recover the ID but also the funds on a specific account, 
/// But we can recover several wrapped accounts with the same ID
/// So we need to do the wrapped accounts one by one (but the ID only the first time)

// pub fn _recovering_account(ctx: Context<RecoverAccount>) -> Result<()> {
//     let recovery_authority = &ctx.accounts.recovery_authority;

//     let signers: Vec<_> = ctx
//         .remaining_accounts
//         .iter()
//         .filter(|account| account.is_signer)
//         .collect();
//     let mut number_of_signatures = 0;
//     for authority in recovery_authority.authorities.iter() {
//         if signers.iter().any(|signer| signer.key == authority) {
//             number_of_signatures += 1;
//         }
//         if number_of_signatures >= recovery_authority.minimum_signatures {
//             break;
//         }
//     }

//     if number_of_signatures < recovery_authority.minimum_signatures {
//         return Err(RecoveryError::NotEnoughSignatures.into());
//     }

//     let last_tx = &mut ctx.accounts.last_tx;
//     let idendity = &mut ctx.accounts.idendity;

//     if last_tx.last_tx_timestamp + 0 > Clock::get()?.unix_timestamp {
//         return Err(RecoveryError::RecoveryTimeNotPassed.into());
//     }
//     if idendity.recovered_token_address.len() > 0 {
//         return Err(IdendityError::IdendityAlreadyRecovered.into());
//     }
//     idendity
//         .recovered_token_address
//         .push(ctx.accounts.new_token_account.key().clone());

//     let seeds: &[&[&[u8]]] = &[&[b"mint", &[ctx.bumps.mint]]];

//     let amount = ctx.accounts.token_account.amount;

//     burn_tokens(&ctx, seeds, amount)?;
//     mint_tokens(&ctx, seeds, amount)?;

//     let token_account = &ctx.accounts.token_account;

//     if token_account.close_authority == Some(ctx.accounts.mint.key()).into() {
//         close_token_account(&ctx, seeds)?;
//     }
//     // This is optional: only possible with account that were created by this program
//     // or for which the program has the authority to close the account
//     // It allows to recover the rent of the account

//     Ok(())
// }
