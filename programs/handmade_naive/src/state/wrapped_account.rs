use anchor_lang::prelude::*;

#[account]
pub struct WrappedTokenAccount {
    pub wrapper_account: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub last_tx: i64, // Last transaction timestamp
}

impl WrappedTokenAccount {

    pub const LEN : usize = 8 + 32 + 32 + 32 + 8 + 8 ;
}
