use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, metadata::{create_master_edition_v3, create_metadata_accounts_v3, mpl_token_metadata::types::{Collection, Creator, DataV2}, CreateMasterEditionV3, CreateMetadataAccountsV3, MasterEditionAccount, Metadata, MetadataAccount}, token_interface::{mint_to, transfer_checked, Mint, MintTo, TokenAccount, TokenInterface, TransferChecked}
};

use crate::state::{ListingAccount, MarketPlace, UserAccount, ListingStatus};

#[derive(Accounts)]
pub struct List<'info> {
    //makes the listing for the NFT
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user_account", maker.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    //marketplace metadata verification
    #[account(
        seeds = [b"marketplace", marketplace.authority.as_ref()],
        bump = marketplace.bump
    )]
    pub marketplace: Account<'info, MarketPlace>,

    // nft mint account
    #[account(
        init,
        payer = maker, 
        mint::authority = maker,
        mint::decimals = 0,
        mint::freeze_authority = maker,

    )]
    pub nft_mint: InterfaceAccount<'info, Mint>,

    // maker ata for transferring nft
    #[account(
        mut,
        associated_token::mint= nft_mint,
        associated_token::authority= maker,
    )]
    pub maker_ata: InterfaceAccount<'info, TokenAccount>,

    // vault for storing the nft
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

    // Collection mint account nft is part of
    pub collection_mint: InterfaceAccount<'info, Mint>,

    // metadata account for the nft
    #[account(
        seeds = [b"metadata", metadata_program.key().as_ref(), nft_mint.key().as_ref()],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() ==
        collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true
    )]
    pub metadata: Account<'info, MetadataAccount>,

    // master edition account for the nft
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            nft_mint.key().as_ref(),
            b"edition",
        ],
        seeds::program = metadata_program.key(),
        bump
    )]
    pub master_edition_account: Account<'info, MasterEditionAccount>,
    pub rent: Sysvar<'info, Rent>,

    // programs
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> List<'info> {
// pub fn mint_nft(
//         &mut self,
//         name: String,
//         symbol: String,
//         uri: String,
//         card_metadata: String,
//         image_url: String,
//         bumps: &ListBumps
//     ) -> Result<()> {
//         Ok(())
//     }

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

        let creators = vec![
            Creator {
                address: self.maker.key(),
                verified: true,
                share: 100,
            }
        ];
        
        let collection = Some(Collection {
            verified: false, // Will need verification in a separate tx
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
        
        let cpi_accounts = CreateMetadataAccountsV3 {
            metadata: self.metadata.to_account_info(),
            mint: self.nft_mint.to_account_info(),
            mint_authority: self.maker.to_account_info(),
            payer: self.maker.to_account_info(),
            update_authority: self.maker.to_account_info(),
            system_program: self.system_program.to_account_info(),
            rent: self.rent.to_account_info(),
        };
        
        let cpi_ctx = CpiContext::new(
            self.metadata_program.to_account_info(),
            cpi_accounts,
        );
        
        create_metadata_accounts_v3(
            cpi_ctx,
            data,
            true,
            true,
            None,
        )?;
        
        // 3. Create master edition
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
        
        let cpi_ctx = CpiContext::new(
            self.metadata_program.to_account_info(),
            cpi_accounts,
        );
        
        create_master_edition_v3(cpi_ctx, Some(0))?;



    self.listing.set_inner(ListingAccount {
        owner: self.maker.key(), 
        nft_address: self.nft_mint.key(),
        card_metadata, 
        listing_price,
        status: ListingStatus::Active,
        created_at: Clock::get()?.unix_timestamp,
        image_url,
        bump: bumps.listing
    });

    let cpi_program = self.token_program.to_account_info();

    let cpi_accounts = TransferChecked {
        from: self.maker_ata.to_account_info(),
        to: self.vault.to_account_info(),
        authority: self.maker.to_account_info(),
        mint: self.nft_mint.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    transfer_checked(cpi_ctx, 1, self.nft_mint.decimals)?;
    self.user_account.nft_listed += 1;
    Ok(())
    }
}
