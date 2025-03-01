use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        create_master_edition_v3,
        create_metadata_accounts_v3,
        mpl_token_metadata::types::{ Collection, Creator, DataV2 },
        CreateMasterEditionV3,
        CreateMetadataAccountsV3,
        Metadata,
    },
    token_interface::{
        mint_to,
        transfer_checked,
        Mint,
        MintTo,
        TokenAccount,
        TokenInterface,
        TransferChecked,
    },
};

use crate::state::{ ListingAccount, ListingStatus, MarketPlace, UserAccount };

#[derive(Accounts)]
pub struct List<'info> {
    // Makes the listing for the NFT
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user_account", maker.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    // Marketplace metadata verification
    #[account(seeds = [b"marketplace", marketplace.authority.as_ref()], bump = marketplace.bump)]
    pub marketplace: Account<'info, MarketPlace>,

    // NFT mint account
    #[account(
        init,
        payer = maker,
        mint::authority = maker,
        mint::decimals = 0,
        mint::freeze_authority = maker
    )]
    pub nft_mint: InterfaceAccount<'info, Mint>,

    // maker ata for transferring nft
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = nft_mint,
        associated_token::authority = maker
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,

    // Vault for storing the NFT
    #[account(
        init,
        payer = maker,
        associated_token::mint = nft_mint,
        associated_token::authority = listing
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    // listing account strores metadata for the listing
    #[account(
        init,
        payer = maker,
        seeds = [marketplace.key().as_ref(), nft_mint.key().as_ref()],
        bump,
        space = 8 + ListingAccount::INIT_SPACE
    )]
    pub listing: Account<'info, ListingAccount>,

    // Collection mint account NFT is part of
    pub collection_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [b"metadata", metadata_program.key().as_ref(), nft_mint.key().as_ref()],
        seeds::program = metadata_program.key(),
        bump,
    )]
    /// CHECK: This account is created via CPI
    pub metadata: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            nft_mint.key().as_ref(),
            b"edition",
        ],
        seeds::program = metadata_program.key(),
        bump
    )]
    /// CHECK: This account is created via CPI
    pub master_edition_account: UncheckedAccount<'info>,

    pub rent: Sysvar<'info, Rent>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> List<'info> {
    pub fn mint_and_list(
        &mut self,
        name: String,
        symbol: String,
        listing_price: u64,
        card_metadata: String,
        image_url: String,
        bumps: &ListBumps
    ) -> Result<()> {
        let cpi_programs = self.token_program.to_account_info();
        let cpi_account = MintTo {
            mint: self.nft_mint.to_account_info(),
            to: self.maker_ata.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_programs, cpi_account);
        mint_to(cpi_ctx, 1)?;

        let creators = vec![Creator {
            address: self.maker.key(),
            verified: true,
            share: 100,
        }];

        let collection = Some(Collection {
            verified: false,
            key: self.collection_mint.key(),
        });

        let data = DataV2 {
            name,
            symbol,
            uri: image_url.clone(),
            seller_fee_basis_points: 0,
            creators: Some(creators),
            collection,
            uses: None,
        };

        // create metadata account
        let cpi_accounts = CreateMetadataAccountsV3 {
            metadata: self.metadata.to_account_info(),
            mint: self.nft_mint.to_account_info(),
            mint_authority: self.maker.to_account_info(),
            payer: self.maker.to_account_info(),
            update_authority: self.maker.to_account_info(),
            system_program: self.system_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.metadata_program.to_account_info(), cpi_accounts);

        create_metadata_accounts_v3(cpi_ctx, data, true, true, None)?;

        // create master edition
        let cpi_accounts = CreateMasterEditionV3 {
            edition: self.master_edition_account.to_account_info(),
            mint: self.nft_mint.to_account_info(),
            update_authority: self.maker.to_account_info(),
            mint_authority: self.maker.to_account_info(),
            payer: self.maker.to_account_info(),
            metadata: self.metadata.to_account_info(),
            token_program: self.token_program.to_account_info(),
            system_program: self.system_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.metadata_program.to_account_info(), cpi_accounts);

        create_master_edition_v3(cpi_ctx, Some(0))?;

        // create the listing account
        self.listing.set_inner(ListingAccount {
            owner: self.maker.key(),
            nft_address: self.nft_mint.key(),
            card_metadata,

            listing_price,
            status: ListingStatus::Active,
            created_at: Clock::get()?.unix_timestamp,
            image_url,
            bump: bumps.listing,
        });

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from: self.maker_ata.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
            mint: self.nft_mint.to_account_info(),
        };

        let cpi_ctx: CpiContext<'_, '_, '_, '_, TransferChecked<'_>> = CpiContext::new(
            cpi_program,
            cpi_accounts
        );
        transfer_checked(cpi_ctx, 1, self.nft_mint.decimals)?;

        self.user_account.nft_listed += 1;

        Ok(())
    }
}
