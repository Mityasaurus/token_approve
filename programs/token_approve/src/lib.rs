use anchor_lang::prelude::*;
use anchor_spl::token::{self, Approve, Token, TokenAccount, Transfer};

declare_id!("3acuxn7voUxVAEMyANx9DNFDY3Y5SXaWxKx7dKdFouuE");

#[program]
pub mod token_approve {
    use super::*;

    pub fn create_offer(ctx: Context<CreateOffer>, amount: u64) -> Result<()> {
        let offer = &mut ctx.accounts.offer;
        offer.amount = amount;
        offer.initializer = *ctx.accounts.initializer.key;

        // Approve the program to spend tokens on behalf of the initializer
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

    pub fn accept_offer(ctx: Context<AcceptOffer>) -> Result<()> {
        let offer = &ctx.accounts.offer;

        // Transfer the approved amount of tokens from Alice to Bob
        let cpi_accounts = Transfer {
            from: ctx.accounts.initializer_token_account.to_account_info(),
            to: ctx.accounts.receiver_token_account.to_account_info(),
            authority: ctx.accounts.program_authority.to_account_info(), // Используем program_authority
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();

        // Seeds должны соответствовать тем, что были использованы при создании program_authority
        let seeds = &[b"authority".as_ref(), &[ctx.bumps.program_authority]];
        let signer = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

        token::transfer(cpi_ctx, offer.amount)?;

        Ok(())
    }
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

#[derive(Accounts)]
pub struct AcceptOffer<'info> {
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

#[account]
pub struct Offer {
    pub amount: u64,
    pub initializer: Pubkey,
}
