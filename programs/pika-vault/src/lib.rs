pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("6z9GJwjWb666YqcP8oocc54K4u3UbbJbh7efdKFH9x7Z");


#[program]
pub mod pika_vault {
    use super::*;

    pub fn register_user(ctx: Context<RegisterUser>) -> Result<()> {
        ctx.accounts.init(&ctx.bumps)
    }

    pub fn initialize_marketplace(ctx: Context<Initialize>, fee: u16) -> Result<()>{
        ctx.accounts.init(fee, &ctx.bumps)?;
        Ok(())
    }

    pub fn mint_and_list(ctx: Context<List>, name: String, symbol: String, listing_price: u64, card_metadata: String, image_url: String,) -> Result<()>{
        ctx.accounts.mint_and_list(name, symbol, listing_price, card_metadata, image_url, &ctx.bumps)?;
        Ok(())
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

