use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub bump: u8,
    pub unlock_price: f64,
    pub escrow_amount: u64,
}