use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use switchboard_on_demand::{prelude::rust_decimal::{prelude::FromPrimitive, Decimal}, PullFeedAccountData};

use crate::{
    constants::{ESCROW_SEED, SOL_USDC_FEED},
    errors::BurryError,
    state::Escrow,
};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        close = user,
        seeds = [ESCROW_SEED, user.key().as_ref()],
        bump = escrow.bump,
    )]
    pub escrow: Account<'info, Escrow>,
    /// CHECK: PullFeedAccountData
    #[account(address = SOL_USDC_FEED @ BurryError::InvalidSwitchboardAccount)]
    pub pull_feed: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

impl Withdraw<'_> {
    pub fn handler(ctx: Context<Withdraw>) -> Result<()> {
        let pull_feed_data = ctx.accounts.pull_feed.data.borrow();
        let pull_feed = PullFeedAccountData::parse(pull_feed_data).unwrap();

        let escrow = &ctx.accounts.escrow;
        let current_sol_price= pull_feed.value(&Clock::get()?).unwrap();

        msg!("Current SOL price: {}", current_sol_price);
        msg!("Escrow unlock price: {}", escrow.unlock_price);

        require_gte!(
            current_sol_price,
            // escrow.unlock_price,
            Decimal::from_f64(escrow.unlock_price).unwrap(),
            BurryError::SolPriceBelowUnlockPrice
        );

        transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.escrow.to_account_info(),
                    to: ctx.accounts.user.to_account_info(),
                },
            ),
            escrow.escrow_amount,
        )
    }
}
