use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    pub authority: Pubkey,
    pub bump: u8,
    pub nft_sold: u64,
    pub nft_bought: u64,
    pub nft_listed: u64,
    // pub total_volume: u64,
    pub created_at: i64,
    // pub shipping_address_hash: [u8; 32],
}
