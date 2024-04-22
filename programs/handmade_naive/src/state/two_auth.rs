use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TwoAuthParameters {
    pub functions: Vec<TwoAuthFunction>, // 4 + 11* len
    pub two_auth_entity: Pubkey,         // 32 - Also called Insurance
    pub allowed_issuers: Vec<Pubkey>,    // 4 + 32 * len
}

impl TwoAuthParameters {
    pub fn get_init_len(functions: Vec<TwoAuthFunction>, allowed_issuers: Vec<Pubkey>) -> usize {
        return 8 + 4 + 11 * functions.len() + 32 + 4 + 32 * allowed_issuers.len();
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum TwoAuthFunction {
    // 1 + MAX(all fields)  = 1 + 8 + space(Duration) = 9 + 2 = 11
    Always,
    Never,
    OnMax {
        max: u64,
    },
    Random,
    CounterResetOnMax {
        max: u64,
    },
    CounterResetOnTime {
        // Usually the time is a day
        max: u64,
        time: Duration,
    },
    CounterWithTimeWindow {
        // Usually the time is a month (4 weeks)
        max: u64,
        time: Duration,
    },
    DeactivateForGeneralWhiteList, // This white list is derived from the receiver address: the insurance has to add their addresss to the white list (to white list the receiver token account)
    DeactivateForUserSpecificWhiteList, // This is user specific and derived from user and receiver address
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum Duration {
    // Space = 1 + 1 = 2
    Seconds(u8),
    Minutes(u8),
    Hours(u8),
    Days(u8),
    Weeks(u8),
}
