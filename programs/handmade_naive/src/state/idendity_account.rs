use anchor_lang::prelude::*;

#[account]
pub struct IdAccount {
    // 8 + 32 +  4 + issuers.len() * 49  + 1 + optional(1* 32)
    pub owner: Pubkey,                     // 32
    pub issuers: Vec<Issuer>,              // 4 + 1* 49
    pub recovered_address: Option<Pubkey>, // recovered_address is the account address of the new owner if the account has been recovered
}

impl IdAccount {
    pub const INIT_LEN: usize = 8 + 32 + 32 + 4 + 49 + 1;

    pub fn get_add_issuer_len(&self) -> usize {
        return 8 + 32 + 4 + self.issuers.len() * 49 + 49 + 1;
    }

    pub fn get_recover_len(&self) -> usize {
        return 8 + 32 + 4 + self.issuers.len() * 49 + 1 + 32;
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Issuer {
    // Total 49
    pub key: Pubkey,        // 32
    pub last_modified: i64, // 8
    pub expires_at: i64,    // 8
    pub active: bool,       // 1
}
