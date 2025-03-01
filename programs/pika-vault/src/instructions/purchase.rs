use crate::error::MarketplaceError;
use crate::state::{ Escrow, ListingAccount, ListingStatus, MarketPlace, UserAccount };
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{ program::invoke, system_instruction };
use anchor_spl::token::{ Mint, TokenAccount };

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user_account", buyer.key().as_ref()],
        bump = buyer_user_account.bump
    )]
    pub buyer_user_account: Account<'info, UserAccount>,

    #[account(seeds = [b"marketplace", marketplace.authority.as_ref()], bump = marketplace.bump)]
    pub marketplace: Account<'info, MarketPlace>,

    #[account(
        mut,
        seeds = [marketplace.key().as_ref(), nft_mint.key().as_ref()],
        bump = listing.bump,
        constraint = matches!(listing.status, ListingStatus::Active),
        constraint = listing.owner != buyer.key()
    )]
    pub listing: Account<'info, ListingAccount>,

    #[account(
        init,
        payer = buyer,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", listing.key().as_ref()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    pub nft_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = nft_mint,
        associated_token::authority = listing
    )]
    pub vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
}

impl<'info> Purchase<'info> {
    pub fn purchase(&mut self) -> Result<()> {
        let sale_amount = self.listing.listing_price;

        let transfer_instruction = system_instruction::transfer(
            self.buyer.key,
            self.escrow.to_account_info().key,
            sale_amount
        );
        invoke(
            &transfer_instruction,
            &[
                self.buyer.to_account_info(),
                self.escrow.to_account_info(),
                self.system_program.to_account_info(),
            ]
        )?;

        self.escrow.seller = self.listing.owner;
        self.escrow.buyer = self.buyer.key();
        self.escrow.nft_mint = self.nft_mint.key();
        self.escrow.sale_amount = sale_amount;
        self.escrow.locked_amount = sale_amount;
        self.escrow.timestamp = Clock::get()?.unix_timestamp;

        // mark the listing as Sold so it can no longer be purchased.
        self.listing.status = ListingStatus::Sold;

        self.buyer_user_account.nft_bought += 1;
        // self.buyer_user_account.nft_bought = self
        //     .buyer_user_account
        //     .nft_bought
        //     .checked_add(1)
        //     .ok_or(error!(MarketplaceError::NameTooLong))?;

        Ok(())
    }
}
