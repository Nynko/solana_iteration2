use anchor_lang::prelude::*;

use crate::{TwoAuthFunction, TwoAuthParameters};

#[account]
pub struct WrappedTokenAccount {
    pub wrapper_account: Pubkey,
    pub mint: Pubkey,
    pub owner: Pubkey,
    pub amount: u64,
    pub last_tx: i64, // Last transaction timestamp
    pub two_auth: Option<TwoAuthParameters>,
}

impl WrappedTokenAccount {

    pub fn get_init_len(two_auth: Option<(&Vec<TwoAuthFunction>,&Vec<Pubkey>)>) -> usize {
        let two_auth_len = match two_auth {
            Some(two_auth) => {
                return TwoAuthParameters::get_init_len(two_auth.0, two_auth.1);
            }
            None => 0,
        };
        return 8 + 32 + 32 + 32 + 8 + 8 + 1 + two_auth_len;
    }
}
