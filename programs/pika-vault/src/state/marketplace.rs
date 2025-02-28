use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MarketPlace {
    pub authority: Pubkey,
    pub fee: u64,
    pub bump: u8,
}
