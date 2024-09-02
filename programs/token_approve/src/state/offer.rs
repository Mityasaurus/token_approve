use anchor_lang::prelude::*;

#[account]
pub struct Offer {
    pub amount: u64,
    pub initializer: Pubkey,
}
