use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Token, TokenAccount};

use crate::state::Offer;

pub fn create_offer(ctx: Context<CreateOffer>, amount: u64) -> Result<()> {
    let offer = &mut ctx.accounts.offer;
    offer.amount = amount;
    offer.initializer = *ctx.accounts.initializer.key;

    let cpi_accounts = Approve {
        to: ctx.accounts.initializer_token_account.to_account_info(),
        delegate: ctx.accounts.program_authority.to_account_info(),
        authority: ctx.accounts.initializer.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::approve(cpi_ctx, amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct CreateOffer<'info> {
    /// CHECK: This is the program authority; no additional checks are necessary.
    #[account(mut)]
    pub initializer: Signer<'info>,
    #[account(mut)]
    pub initializer_token_account: Account<'info, TokenAccount>,
    #[account(init, payer = initializer, space = 8 + 40)]
    pub offer: Account<'info, Offer>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
    /// CHECK: This is the program authority; no additional checks are necessary.
    #[account(seeds = [b"authority".as_ref()], bump)]
    pub program_authority: AccountInfo<'info>,
}
