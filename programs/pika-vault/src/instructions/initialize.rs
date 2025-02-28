use crate::state::MarketPlace;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [b"marketplace", admin.key().as_ref()],
        bump,
        space = 8 + MarketPlace::INIT_SPACE
    )]
    pub marketplace: Account<'info, MarketPlace>,
    #[account(seeds = [b"treasury", marketplace.key().as_ref()], bump)]
    pub treasury: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, fee: u16, bumps: &InitializeBumps) -> Result<()> {
        self.marketplace.set_inner(MarketPlace {
            authority: self.admin.key(),
            fee,
            bump: bumps.marketplace,
            treasury_bump: bumps.treasury,
        });
        Ok(())
    }
}
