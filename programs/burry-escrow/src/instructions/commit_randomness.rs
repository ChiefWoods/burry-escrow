use anchor_lang::prelude::*;
use switchboard_on_demand::RandomnessAccountData;

use crate::{constants::ESCROW_SEED, errors::BurryError, state::Escrow};

#[derive(Accounts)]
pub struct CommitRandomness<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [ESCROW_SEED, user.key().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
    /// CHECK: RandomnessAccountData
    pub randomness: AccountInfo<'info>,
}

impl CommitRandomness<'_> {
    pub fn handler(ctx: Context<CommitRandomness>) -> Result<()> {
        let randomess_data = RandomnessAccountData::parse(ctx.accounts.randomness.data.borrow()).unwrap();

        require_eq!(
            randomess_data.seed_slot,
            Clock::get()?.slot - 1,
            BurryError::RandomessAlreadyRevealed
        );

        ctx.accounts.escrow.randomness = ctx.accounts.randomness.key();
        ctx.accounts.escrow.seed_slot = randomess_data.seed_slot;

        Ok(())
    }
}