use crate::error::ErrorCode;
use crate::state::Escrow;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ReleaseEscrow<'info> {
    /// CHECK everything all right
    #[account(mut, signer)]
    pub seller: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"escrow", listing.key().as_ref()],
        bump = escrow.bump,
        close = seller
    )]
    pub escrow: Account<'info, Escrow>,
    /// CHECK everything all right
    pub listing: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

/// Releases funds from escrow—this function checks that the escrow’s seller matches the signer,
/// then (by closing the escrow account) passes all lamports stored in escrow to the seller.
impl<'info> ReleaseEscrow<'info> {
    pub fn release_escrow(&mut self) -> Result<()> {
        let escrow = &mut self.escrow;
        let seller = &self.seller;

        // Verify that the escrow was intended for this seller.
        require!(
            escrow.seller == seller.key(),
            ErrorCode::CustomError // Alternatively, add a more specific error variant here.
        );

        // Optionally update escrow state before closing.
        escrow.locked_amount = 0;
        Ok(())
    }
}
