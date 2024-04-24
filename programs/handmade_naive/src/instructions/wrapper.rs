use anchor_lang::{prelude::*, solana_program::program};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::spl_token,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{error::WrapperError, WrappedTokenAccount, WrapperAccount};

#[derive(Accounts)]
#[instruction(list_issuer: Vec<Pubkey>)]
pub struct InitializeWrapper<'info> {
    #[account(init, seeds=[b"wrapper", approver.key().as_ref()], bump, payer=payer, space=WrapperAccount::get_init_len(list_issuer))]
    pub wrapper_account: Account<'info, WrapperAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub approver: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddWrapperIssuer<'info> {
    #[account(mut, seeds=[b"wrapper", approver.key().as_ref()], bump, realloc=wrapper_account.get_add_issuer_len(), realloc::payer=payer, realloc::zero=false)]
    pub wrapper_account: Account<'info, WrapperAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub approver: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DeleteWrapperIssuer<'info> {
    #[account(mut, seeds=[b"wrapper", approver.key().as_ref()], bump, realloc=wrapper_account.get_remove_issuer_len(), realloc::payer=payer, realloc::zero=true)]
    pub wrapper_account: Account<'info, WrapperAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub approver: Signer<'info>,
    /// CHECK: The issuer to be removed
    #[account(constraint = wrapper_account.list_issuer.contains(&issuer.key()))]
    pub issuer: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeWrappedAccount<'info> {
    #[account(init, seeds=[b"wrapped_token", wrapper_account.key().as_ref(), mint.key().as_ref(), owner.key().as_ref()], bump, payer=payer, space=WrappedTokenAccount::LEN)]
    pub wrapped_token_account: Account<'info, WrappedTokenAccount>,
    #[account(seeds=[b"wrapper", approver.key().as_ref()], bump)]
    pub wrapper_account: Account<'info, WrapperAccount>,
    /// CHECK: The approver of the wrapper
    pub approver: UncheckedAccount<'info>,
    pub owner: Signer<'info>,
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(mint::token_program = token_program)]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WrapTokenHolder<'info> {
    #[account(seeds=[b"wrapper", approver.key().as_ref()], bump)]
    pub wrapper_account: Account<'info, WrapperAccount>,
    #[account(
        init,
        payer = payer,
        associated_token::mint = mint,
        associated_token::authority = wrapper_account
    )]
    pub wrapper_associated_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: The approver of the wrapper
    pub approver: UncheckedAccount<'info>,
    #[account(mint::token_program = token_program)]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WrapTokens<'info> {
    #[account(seeds=[b"wrapper", approver.key().as_ref()], bump)]
    pub wrapper_account: Account<'info, WrapperAccount>,
    /// CHECK: The approver of the wrapper
    pub approver: UncheckedAccount<'info>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = wrapper_account,
    )]
    pub wrapper_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut, seeds=[b"wrapped_token", wrapper_account.key().as_ref(), mint.key().as_ref(), owner.key().as_ref()], bump, has_one=wrapper_account, has_one = mint)]
    pub user_wrapped_token_account: Account<'info, WrappedTokenAccount>,
    #[account(mut, token::authority = owner, token::mint = mint)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(constraint = owner.key() == user_wrapped_token_account.owner)]
    pub owner: Signer<'info>,
    #[account(mint::token_program = token_program)]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn _initialize_wrapper(
    ctx: Context<InitializeWrapper>,
    list_issuer: Vec<Pubkey>,
) -> Result<()> {
    let wrapper_account = &mut ctx.accounts.wrapper_account;
    wrapper_account.list_issuer = list_issuer;
    Ok(())
}

pub fn _add_issuers_wrapper(ctx: Context<AddWrapperIssuer>, issuer: Pubkey) -> Result<()> {
    let wrapper_account = &mut ctx.accounts.wrapper_account;
    wrapper_account.list_issuer.push(issuer);
    Ok(())
}

pub fn _remove_issuer_wrapper(ctx: Context<DeleteWrapperIssuer>) -> Result<()> {
    let wrapper_account = &mut ctx.accounts.wrapper_account;
    let index = wrapper_account
        .list_issuer
        .iter()
        .position(|x| *x == ctx.accounts.issuer.key())
        .unwrap();
    wrapper_account.list_issuer.remove(index);
    Ok(())
}

pub fn _initialize_mint(_ctx: Context<WrapTokenHolder>) -> Result<()> {
    Ok(())
}

pub fn _initialize_wrap_account(ctx: Context<InitializeWrappedAccount>) -> Result<()> {
    let wrapped_token_account = &mut ctx.accounts.wrapped_token_account;
    wrapped_token_account.amount = 0;
    wrapped_token_account.owner = ctx.accounts.owner.key();
    wrapped_token_account.wrapper_account = ctx.accounts.wrapper_account.key();
    wrapped_token_account.mint = ctx.accounts.mint.key();
    wrapped_token_account.last_tx = Clock::get()?.unix_timestamp;
    Ok(())
}

pub fn _wrap_tokens(ctx: Context<WrapTokens>, amount: u64, decimals: u8) -> Result<()> {
    let mint = &ctx.accounts.mint;

    if mint.decimals != decimals {
        return Err(WrapperError::InvalidDecimals.into());
    }

    // CPI to transfer tokens from user to wrapper
    let ix = spl_token::instruction::transfer(
        ctx.accounts.token_program.key,
        &ctx.accounts.user_token_account.key(),
        &ctx.accounts.wrapper_token_account.key(),
        ctx.accounts.owner.key,
        &[ctx.accounts.owner.key],
        amount,
    )?;
    program::invoke(
        &ix,
        &[
            ctx.accounts.token_program.to_account_info(),
            ctx.accounts.user_token_account.to_account_info(),
            ctx.accounts.wrapper_token_account.to_account_info(),
            ctx.accounts.owner.to_account_info(),
        ],
    )?;

    // Adding to the corresponding wrapped token account
    let wrapped_token_account = &mut ctx.accounts.user_wrapped_token_account;
    wrapped_token_account.amount += amount;

    Ok(())
}
