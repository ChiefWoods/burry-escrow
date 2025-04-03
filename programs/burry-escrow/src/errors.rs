use anchor_lang::prelude::*;

#[error_code]
#[derive(Eq, PartialEq)]
pub enum BurryError {
    #[msg("Not a valid Switchboard account")]
    InvalidSwitchboardAccount,
    #[msg("Current SOL price is not above Escrow unlock price.")]
    SolPriceBelowUnlockPrice,
}
