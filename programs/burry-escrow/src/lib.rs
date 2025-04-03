use anchor_lang::prelude::*;
use instructions::{deposit::*, withdraw::*};

pub mod errors;
pub mod instructions;
pub mod state;
pub mod constants;

declare_id!("DDFpiq1hQUmXPr6RvnYw4srpRjeDzGjkVNBZdr4oNfSw");

#[program]
pub mod burry_escrow {
    use super::*;

    pub fn deposit(ctx: Context<Deposit>, args: DepositArgs) -> Result<()> {
        Deposit::handler(ctx, args)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        Withdraw::handler(ctx)
    }
}
