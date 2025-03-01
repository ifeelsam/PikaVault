pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("BWKeDzrKgFNb626Cu8UpjbMYKHpPH6hzi5UC9Gs3yZ6m");

#[program]
pub mod pika_vault {
    use super::*;

    pub fn register_user(ctx: Context<RegisterUser>) -> Result<()> {
        ctx.accounts.init(&ctx.bumps)
    }

    pub fn initialize_marketplace(ctx: Context<Initialize>, fee: u16) -> Result<()> {
        ctx.accounts.init(fee, &ctx.bumps)?;
        Ok(())
    }

    pub fn mint_and_list(
        ctx: Context<List>,
        name: String,
        symbol: String,
        listing_price: u64,
        card_metadata: String,
        image_url: String
    ) -> Result<()> {
        ctx.accounts.mint_and_list(
            name,
            symbol,
            listing_price,
            card_metadata,
            image_url,
            &ctx.bumps
        )?;
        Ok(())
    }

    pub fn delist(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.delist()
    }

    pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.purchase()?;
        Ok(())
    }

    pub fn release_escrow(ctx: Context<ReleaseEscrow>) -> Result<()> {
        ctx.accounts.release_escrow()
    }
}

// - Upload Card & Mint NFT
// - List Card for Sale
// - Unlist Card
// - Purchase Card (Create Escrow)
// - Ship Card (Seller Confirmation)
// - Confirm Receipt (Release Escrow)
// - Cancel Transaction (Refund)
// - View Listings
// - View Collection
