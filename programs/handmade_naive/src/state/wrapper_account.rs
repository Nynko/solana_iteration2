use anchor_lang::prelude::*;

#[account]
pub struct WrapperAccount {
    pub approver: Pubkey,
    pub list_issuer: Vec<Pubkey>,
}

impl WrapperAccount {
    pub fn get_init_len(list_issuer: Vec<Pubkey>) -> usize {
        return 8 + 32 + 4 + 32 * list_issuer.len();
    }

    pub fn get_add_issuer_len(&self) -> usize {
        return 8 + 32 + 4 + 32 * self.list_issuer.len() + 32;
    }

    pub fn get_remove_issuer_len(&self) -> usize {
        return 8 + 32 + 4 + 32 * self.list_issuer.len() - 32;
    }
}
