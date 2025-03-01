use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{ transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked },
};

use crate::{ marketplace, ListingAccount, ListingStatus, MarketPlace, UserAccount };

#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user_account", owner.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(seeds = [b"marketplace", marketplace.authority.as_ref()], bump = marketplace.bump)]
    pub marketplace: Account<'info, MarketPlace>,

    pub nft_mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = nft_mint,
        associated_token::authority = owner
    )]
    pub owner_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [marketplace.key().as_ref(), nft_mint.key().as_ref()],
        bump = listing.bump,
        constraint = listing.owner == owner.key(),
        constraint = matches!(listing.status, ListingStatus::Active),
        close = owner
    )]
    pub listing: Account<'info, ListingAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Delist<'info> {
    pub fn delist(&mut self) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.nft_mint.to_account_info(),
            to: self.owner_ata.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let marketplace_ref = self.marketplace.key();
        let nft_ref = self.nft_mint.key();
        let seeds = &[marketplace_ref.as_ref(), nft_ref.as_ref(), &[self.listing.bump]];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        transfer_checked(cpi_ctx, 1, self.nft_mint.decimals)?;

        // decrement listed count in user account
        self.user_account.nft_listed -= 1;

        Ok(())
    }
}
