use anchor_lang::prelude::*;
use solana_program::{program::invoke, system_instruction};

use crate::{error::ListingError, Escrow, ListingAccount, ListingStatus, MarketPlace, UserAccount};

#[derive(Accounts)]
pub struct Refund<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user_account", buyer.key().as_ref()],
        bump = buyer_account.bump
    )]
    pub buyer_account: Account<'info, UserAccount>,

    #[account(
        mut,
        seeds = [b"user_account", listing.owner.as_ref()],
        bump = seller_account.bump
    )]
    pub seller_account: Account<'info, UserAccount>,
    #[account(
        seeds = [b"marketplace", marketplace.authority.as_ref()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, MarketPlace>,
    #[account(
        mut,
        seeds =  [marketplace.key().as_ref(), listing.nft_address.as_ref()],
        bump = listing.bump,
        constraint = matches!(listing.status , ListingStatus::Sold) @ ListingError::ListingNotSold
    )]
    pub listing: Account<'info, ListingAccount>,

    #[account(
        mut,
        seeds = [b"escrow", listing.key().as_ref()],
        bump = escrow.bump,
        constraint = buyer.key() == escrow.buyer @ ListingError::UnauthorizedRefund,
        constraint = escrow.locked_amount > 0 @ ListingError::EscrowAlreadyReleased
    )]
    pub escrow: Account<'info, Escrow>,

    pub system_program: Program<'info, System>,
}

impl<'info> Refund<'info> {
    pub fn refund(&mut self) -> Result<()> {
        require!(
            self.escrow.locked_amount > 0,
            ListingError::EscrowAlreadyReleased
        );

        let refund_amount = self.escrow.locked_amount;

        let transfer_instruction = system_instruction::transfer(
            &self.escrow.to_account_info().key(),
            &self.escrow.buyer,
            refund_amount,
        );

        invoke(
            &transfer_instruction,
            &[
                self.escrow.to_account_info(),
                self.system_program.to_account_info(),
            ],
        )?;

        //state update
        self.escrow.locked_amount = 0;
        self.listing.status = ListingStatus::Active;
        self.buyer_account.nft_bought -= 1;
        self.seller_account.nft_sold -= 1;
        self.seller_account.nft_listed += 1;

        Ok(())
    }
}
