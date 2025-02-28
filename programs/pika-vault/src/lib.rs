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

