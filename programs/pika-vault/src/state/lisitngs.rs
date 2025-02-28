use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ListingAccount {
    pub owner: Pubkey,
    pub nft_address: Pubkey,
    #[max_len(50)]
    pub card_metadata: String,
    pub listing_price: u64,
    pub status: ListingStatus,
    pub created_at: i64,
    #[max_len(46)] // Ipfs hash
    pub image_url: String,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum ListingStatus {
    Active,
    Sold,
    Unlisted,
}
