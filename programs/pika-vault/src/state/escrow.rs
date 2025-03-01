use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seller: Pubkey,
    pub buyer: Pubkey,
    pub bump: u8,
    pub nft_mint: Pubkey,
    pub sale_amount: u64,
    pub locked_amount: u64,
    pub timestamp: i64,
}
