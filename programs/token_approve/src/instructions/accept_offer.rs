use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::state::Offer;

pub fn accept_offer(ctx: Context<AcceptOffer>) -> Result<()> {
    let offer = &ctx.accounts.offer;

    let cpi_accounts = Transfer {
        from: ctx.accounts.initializer_token_account.to_account_info(),
        to: ctx.accounts.receiver_token_account.to_account_info(),
        authority: ctx.accounts.program_authority.to_account_info(),
    };
    let cpi_program = ctx.accounts.token_program.to_account_info();

    let seeds = &[b"authority".as_ref(), &[ctx.bumps.program_authority]];
    let signer = &[&seeds[..]];

    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

    token::transfer(cpi_ctx, offer.amount)?;

    Ok(())
}

#[derive(Accounts)]
pub struct AcceptOffer<'info> {
    /// CHECK: This is the program authority; no additional checks are necessary.
    #[account(mut)]
    pub receiver: Signer<'info>,
    /// CHECK: This is the program authority; no additional checks are necessary.
    #[account(mut)]
    pub initializer: AccountInfo<'info>,
    #[account(mut)]
    pub initializer_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub receiver_token_account: Account<'info, TokenAccount>,
    #[account(mut, has_one = initializer)]
    pub offer: Account<'info, Offer>,
    pub token_program: Program<'info, Token>,
    /// CHECK: This is the program authority; no additional checks are necessary.
    #[account(seeds = [b"authority".as_ref()], bump)]
    pub program_authority: AccountInfo<'info>,
}
