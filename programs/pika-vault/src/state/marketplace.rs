use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MarketPlace {
    pub authority: Pubkey,
    pub fee: u16,
    pub bump: u8,
    pub treasury_bump: u8,
}
