use anchor_lang::prelude::*;

use crate::{TwoAuthFunction, TwoAuthParameters, WrappedTokenAccount};

// #[derive(Accounts)]
// #[instruction(functions: Vec<TwoAuthFunction>, allowed_issuers: Vec<Pubkey>)]
// pub struct UpdateTwoAuth<'info> {
//     #[account(init, seeds=[b"two_auth", token_account.key().as_ref()], bump, payer=owner, space= TwoAuthParameters::get_init_len(functions, allowed_issuers))]
//     pub two_auth_parameters: Account<'info, WrappedTokenAccount>,
//     #[account(init, seeds=[b"transaction_approval", owner.key().as_ref()], bump, payer=owner, space= 8 + 81 )]
//     pub transaction_approval: Account<'info, TransactionAproval>,
//     pub two_auth_entity: Signer<'info>,
//     #[account(mut)]
//     pub owner: Signer<'info>,
//     #[account(mut, seeds = [b"mint"], bump)]
//     pub mint: InterfaceAccount<'info, Mint>,
//     #[account(
//         token::mint = mint,
//         token::authority = owner,
//     )]
//     pub token_account: InterfaceAccount<'info, TokenAccount>,
//     pub system_program: Program<'info, System>,
// }

// #[derive(Accounts)]
// pub struct ApproveTransaction<'info> {
//     #[account(seeds=[b"two_auth", token_account.key().as_ref()], bump)]
//     pub two_auth_parameters: Account<'info, TwoAuthParameters>,
//     #[account(mut, seeds=[b"transaction_approval", owner.key().as_ref()], bump)]
//     pub transaction_approval: Account<'info, TransactionAproval>,
//     #[account(mut)]
//     pub approver: Signer<'info>,
//     #[account(seeds = [b"mint"], bump)]
//     pub mint: InterfaceAccount<'info, Mint>,
//     #[account(
//         token::mint = mint,
//         token::authority = owner,
//     )]
//     pub token_account: InterfaceAccount<'info, TokenAccount>,
//     ///  CHECK : Owner of the token account
//     pub owner: AccountInfo<'info>,
//     pub system_program: Program<'info, System>,
// }

// #[error_code]
// pub enum TwoAuthError {
//     #[msg("Not authorized to approve this transaction")]
//     NotAuthorized,
//     #[msg("The Approval has expired")]
//     ExpiredApproval,
// }

// pub fn _initialize_two_auth(
//     ctx: Context<InitializeTwoAuth>,
//     functions: Vec<TwoAuthFunction>,
//     allowed_issuers: Vec<Pubkey>,
// ) -> Result<()> {
//     let two_auth_parameters = &mut ctx.accounts.two_auth_parameters;
//     two_auth_parameters.functions = functions;
//     two_auth_parameters.two_auth_entity = ctx.accounts.two_auth_entity.key();
//     two_auth_parameters.allowed_issuers = allowed_issuers;

//     Ok(())
// }

// pub fn _approve_transaction(
//     ctx: Context<ApproveTransaction>,
//     transaction: TransactionRepresentation,
// ) -> Result<()> {
//     let two_auth_parameters = &ctx.accounts.two_auth_parameters;
//     let two_auth_entity = &two_auth_parameters.two_auth_entity;
//     let approver = &ctx.accounts.approver.key();

//     if !two_auth_entity.eq(approver) {
//         return Err(TwoAuthError::NotAuthorized.into());
//     }

//     let transaction_approval = &mut ctx.accounts.transaction_approval;
//     transaction_approval.transaction = transaction;
//     transaction_approval.active = true;
//     Ok(())
// }

// Functions from TwoAuthFunction

/*
    Returns true if there is need for two auth
*/
pub fn apply_two_auth_functions(amount: u64, functions: &Vec<TwoAuthFunction>) -> bool {
    return functions
        .iter()
        .all(|function| match_functions(amount, function));
}

pub fn match_functions(amount: u64, function: &TwoAuthFunction) -> bool {
    match function {
        TwoAuthFunction::Always => true,
        TwoAuthFunction::Never => false,
        TwoAuthFunction::OnMax { max } => amount >= *max,
        _ => true,
    }
}

pub fn on_max(amount: u64, max: u64) -> bool {
    if amount >= max {
        return true;
    } else {
        return false;
    }
}
