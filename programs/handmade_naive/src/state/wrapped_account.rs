use anchor_lang::prelude::*;

use crate::TwoAuthParameters;

#[account]
pub struct WrappedTokenAccount {
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub last_tx: i64, // Last transaction timestamp
    pub two_auth: Option<TwoAuthParameters>,
}

impl WrappedTokenAccount {
    pub fn get_init_len(two_auth: Option<TwoAuthParameters>) -> usize {
        let two_auth_len = match two_auth {
            Some(two_auth) => {
                TwoAuthParameters::get_init_len(two_auth.functions, two_auth.allowed_issuers)
            }
            None => 0,
        };
        return 8 + 32 + 32 + 8 + 8 + 1 + two_auth_len;
    }
}
