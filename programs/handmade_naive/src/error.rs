use anchor_lang::prelude::*;

#[error_code]
pub enum WrapperError {
    #[msg("Decimal provided does not match the mint's decimal value")]
    InvalidDecimals,
}

#[error_code]
pub enum TransferError {
    #[msg("The source account does not have enough funds to transfer")]
    InsufficientFunds,
    #[msg("Decimal provided does not match the mint's decimal value")]
    InvalidDecimals,
}

#[error_code]
pub enum IdendityError {
    #[msg("Idendity already exists")]
    IdendityAlreadyExists,
    #[msg("Idendity is not active")]
    IdendityNotActive,
    #[msg("Idendity expired")]
    IdendityExpired,
    #[msg("Idendity recovered")]
    IdendityRecovered,
    #[msg("Idendity already recovered")]
    IdendityAlreadyRecovered,
    #[msg("Issuer is not approved")]
    IssuerNotApproved,
}
