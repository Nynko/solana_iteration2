use anchor_lang::prelude::*;

// An account has the right to designate any recovery authority to recover the account
// It can be an insurance company, a friend, a family member, a backup address...
// Minimum signatures indicates the minimum number of signatures required to recover the account
#[account]
pub struct RecoveryAuthorities {
    pub authorities: Vec<RecoveryAuthority>, 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct RecoveryAuthority { // space = 4 + 32 + 1 + 4
    pub authority: Pubkey,
    pub min_signatures: u8, // minimum number of signatures required to recover the account
    pub min_duration: u32, // minimum duration before the account can be recovered
}


impl RecoveryAuthorities {
    pub fn get_init_len(recovery_authorities: &Vec<RecoveryAuthority>) -> usize {
        return 8 + recovery_authorities.len() * (4 + 32 + 1 + 4);
    }
}