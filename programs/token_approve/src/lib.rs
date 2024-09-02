use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::*;

declare_id!("3acuxn7voUxVAEMyANx9DNFDY3Y5SXaWxKx7dKdFouuE");

#[program]
pub mod token_approve {
    use super::*;

    pub fn create_offer(ctx: Context<CreateOffer>, amount: u64) -> Result<()> {
        instructions::create_offer(ctx, amount)
    }

    pub fn accept_offer(ctx: Context<AcceptOffer>) -> Result<()> {
        instructions::accept_offer(ctx)
    }
}
