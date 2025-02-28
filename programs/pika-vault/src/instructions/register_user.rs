use anchor_lang::prelude::*;

use crate::state::UserAccount;

#[derive(Accounts)]
pub struct RegisterUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = UserAccount::INIT_SPACE + 8,
        seeds = [b"user_account", user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,

    pub system_program: Program<'info, System>,
}

impl<'info> RegisterUser<'info> {
    pub fn init(&mut self, bumps: &RegisterUserBumps) -> Result<()> {
        self.user_account.set_inner(UserAccount {
            authority: self.user.key(),
            nft_sold: 0,
            nft_bought: 0,
            nft_listed: 0,
            created_at: Clock::get()?.unix_timestamp,
            bump: bumps.user_account,
        });
        Ok(())
    }
}
