import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PikaVault } from "../target/types/pika_vault";
import {
    Keypair,
    SystemProgram,
    LAMPORTS_PER_SOL,
    PublicKey,
    Transaction,
    ComputeBudgetProgram,
    sendAndConfirmTransaction,
} from "@solana/web3.js";
import { assert } from "chai";
import {
    TOKEN_PROGRAM_ID,
    createMint,
    getAssociatedTokenAddress,
    getAccount,
} from "@solana/spl-token";
import { BN } from "bn.js";

const metadataProgramId = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
);

describe("buying nft testing", () => {
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.PikaVault as Program<PikaVault>;
    let user = new Keypair();
    let admin = new Keypair();
    const fee = 1000;

    let makerAta: PublicKey;
    let vault: PublicKey;
    let listing: PublicKey;
    let listingBump: number;
    let nftMint: PublicKey;
    let marketplace: PublicKey;
    let collectionMint: PublicKey;
    let metadata: PublicKey;
    let masterEdition: PublicKey;

    const [marketplacePDA, marketplaceBump] = PublicKey.findProgramAddressSync(
        [Buffer.from("marketplace"), admin.publicKey.toBuffer()],
        program.programId
    );

    it("Airdrop for nft", async () => {
        await Promise.all(
            [user].map(async (k) => {
                return await anchor
                    .getProvider()
                    .connection.requestAirdrop(
                        k.publicKey,
                        100 * LAMPORTS_PER_SOL
                    )
                    .then(confirmTx);
            })
        );
    });

    it("Airdrop for Marketplace Authority", async () => {
        await Promise.all(
            [admin].map(async (k) => {
                return await anchor
                    .getProvider()
                    .connection.requestAirdrop(
                        k.publicKey,
                        100 * LAMPORTS_PER_SOL
                    )
                    .then(confirmTx);
            })
        );
    });
    it("Initializes Marketplace", async () => {
        const [treasuryPDA, treasuryBump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [Buffer.from("treasury"), marketplacePDA.toBuffer()],
                program.programId
            );
        await program.methods
            .initializeMarketplace(fee)
            .accountsStrict({
                admin: admin.publicKey,
                marketplace: marketplacePDA,
                treasury: treasuryPDA,
                systemProgram: SystemProgram.programId,
            })
            .signers([admin])
            .rpc()
            .then(confirmTx);

        const marketplaceAccount = await program.account.marketPlace.fetch(
            marketplacePDA
        );
        assert.equal(
            marketplaceAccount.authority.toString(),
            admin.publicKey.toString()
        );
        assert.equal(marketplaceAccount.bump, marketplaceBump);
        assert.equal(marketplaceAccount.fee, fee);
    });

    it("Registers a user", async () => {
        const [userAccountPDA, userAccountBump] =
            anchor.web3.PublicKey.findProgramAddressSync(
                [Buffer.from("user_account"), user.publicKey.toBuffer()],
                program.programId
            );
        await program.methods
            .registerUser()
            .accounts({
                user: user.publicKey,
            })
            .signers([user])
            .rpc();

        const userAccount = await program.account.userAccount.fetch(
            userAccountPDA
        );
        assert.equal(
            userAccount.authority.toString(),
            user.publicKey.toString(),
            `Authority check failed`
        );
        assert.equal(
            userAccount.nftSold.toNumber(),
            0,
            `NFT Sold check failed!`
        );
        assert.equal(
            userAccount.nftBought.toNumber(),
            0,
            `NFT Bought check failed!`
        );
        assert.equal(
            userAccount.nftListed.toNumber(),
            0,
            `NFT Listed check failed!`
        );
        assert.equal(userAccount.bump, userAccountBump, `Bump check failed!`);
    });

    const [userAccountPDA, userAccountBump] =
        anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("user_account"), user.publicKey.toBuffer()],
            program.programId
        );
    it("Mints and Lists an NFT", async () => {
        const nftMintKeypair = Keypair.generate();
        nftMint = nftMintKeypair.publicKey;

        collectionMint = await createMint(
            anchor.getProvider().connection,
            admin,
            admin.publicKey,
            admin.publicKey,
            0
        );

        [marketplace] = anchor.web3.PublicKey.findProgramAddressSync(
            [Buffer.from("marketplace"), admin.publicKey.toBuffer()],
            program.programId
        );

        await Promise.all(
            [nftMintKeypair].map(async (k) => {
                return await anchor
                    .getProvider()
                    .connection.requestAirdrop(
                        k.publicKey,
                        100 * LAMPORTS_PER_SOL
                    )
                    .then(confirmTx);
            })
        );
        [metadata] = await PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"),
                metadataProgramId.toBuffer(),
                nftMint.toBuffer(),
            ],
            metadataProgramId
        );

        [masterEdition] = await PublicKey.findProgramAddressSync(
            [
                Buffer.from("metadata"),
                metadataProgramId.toBuffer(),
                nftMint.toBuffer(),
                Buffer.from("edition"),
            ],
            metadataProgramId
        );

        makerAta = await getAssociatedTokenAddress(nftMint, user.publicKey);

        [listing, listingBump] = await PublicKey.findProgramAddressSync(
            [marketplace.toBuffer(), nftMint.toBuffer()],
            program.programId
        );

        vault = await getAssociatedTokenAddress(nftMint, listing, true);

        const ix = await program.methods
            .mintAndList(
                "Test NFT",
                "TNT",
                new anchor.BN(anchor.web3.LAMPORTS_PER_SOL),
                "Test Card Metadata",
                "https://example.com/image.png"
            )
            .accountsStrict({
                maker: user.publicKey,
                userAccount: userAccountPDA,
                marketplace,
                nftMint,
                makerAta,
                vault,
                listing,
                collectionMint,
                metadata,
                masterEditionAccount: masterEdition,
                rent: anchor.web3.SYSVAR_RENT_PUBKEY,
                tokenProgram: TOKEN_PROGRAM_ID,
                associatedTokenProgram:
                    anchor.utils.token.ASSOCIATED_PROGRAM_ID,
                systemProgram: anchor.web3.SystemProgram.programId,
                metadataProgram: metadataProgramId,
            })
            .signers([user, nftMintKeypair])
            .instruction();

        const tx = new Transaction();
        tx.add(ComputeBudgetProgram.setComputeUnitLimit({ units: 500_000 }));
        tx.add(ix);
        const sig = await sendAndConfirmTransaction(
            anchor.getProvider().connection,
            tx,
            [user, nftMintKeypair]
        );

        const listingAccount = await program.account.listingAccount.fetch(
            listing
        );

        // checks
        assert.equal(
            listingAccount.owner.toString(),
            user.publicKey.toString(),
            "failed owner check"
        );
        assert.equal(
            listingAccount.nftAddress.toString(),
            nftMint.toString(),
            "failed nft address"
        );
        assert.equal(
            listingAccount.cardMetadata,
            "Test Card Metadata",
            "error: failed to get listing's metadata"
        );
        assert.equal(
            listingAccount.listingPrice.toString(),
            anchor.web3.LAMPORTS_PER_SOL.toString()
        );
        assert.deepEqual(listingAccount.status, { active: {} });
        assert.equal(listingAccount.imageUrl, "https://example.com/image.png");
        assert.equal(listingAccount.bump, listingBump);

        const vaultAccount = await getAccount(
            anchor.getProvider().connection,
            vault
        );
        assert.equal(vaultAccount.amount.toString(), "1", "bn error");

        const updatedUserAccount = await program.account.userAccount.fetch(
            userAccountPDA
        );
        assert.equal(updatedUserAccount.nftListed.toNumber(), 1);
    });

    const buyer = Keypair.generate();

    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const connection = provider.connection;

    let buyerUserAccount: PublicKey;
    let buyerUserAccountBump: number;

    [buyerUserAccount, buyerUserAccountBump] = PublicKey.findProgramAddressSync(
        [Buffer.from("user_account"), buyer.publicKey.toBuffer()],
        program.programId
    );

    it("Airdrop for buyer", async () => {
        await Promise.all(
            [buyer].map(async (k) => {
                return await anchor
                    .getProvider()
                    .connection.requestAirdrop(
                        k.publicKey,
                        100 * LAMPORTS_PER_SOL
                    )
                    .then(confirmTx);
            })
        );
    });

    it("Registers a buyer", async () => {
        await program.methods
            .registerUser()
            .accounts({
                user: buyer.publicKey,
            })
            .signers([buyer])
            .rpc();

        const userAccount = await program.account.userAccount.fetch(
            buyerUserAccount
        );
        assert.equal(
            userAccount.authority.toString(),
            buyer.publicKey.toString(),
            `Authority check failed`
        );
        assert.equal(
            userAccount.nftSold.toNumber(),
            0,
            `NFT Sold check failed!`
        );
        assert.equal(
            userAccount.nftBought.toNumber(),
            0,
            `NFT Bought check failed!`
        );
        assert.equal(
            userAccount.nftListed.toNumber(),
            0,
            `NFT Listed check failed!`
        );
        assert.equal(
            userAccount.bump,
            buyerUserAccountBump,
            `Bump check failed!`
        );
    });

    it("Allows a buyer to purchase the NFT", async () => {
        let escrow: PublicKey;
        [escrow] = PublicKey.findProgramAddressSync(
            [Buffer.from("escrow"), listing.toBuffer()],
            program.programId
        );

        const buyerBalanceBefore = await connection.getBalance(buyer.publicKey);
        await program.methods
            .purchase()
            .accountsStrict({
                buyer: buyer.publicKey,
                buyerAccount: buyerUserAccount,
                marketplace: marketplacePDA,
                listing: listing,
                escrow: escrow,
                nftMint: nftMint,
                sellerAccount: userAccountPDA,
                systemProgram: SystemProgram.programId,
            })
            .signers([buyer])
            .rpc();

        const updatedListing = await program.account.listingAccount.fetch(
            listing
        );
        assert.notDeepEqual(updatedListing.status, { active: {} });

        const escrowAccount = await program.account.escrow.fetch(escrow);
        assert.equal(
            escrowAccount.saleAmount.toString(),
            LAMPORTS_PER_SOL.toString()
        );
        assert.equal(
            escrowAccount.buyer.toString(),
            buyer.publicKey.toString()
        );
        const buyerUser = await program.account.userAccount.fetch(
            buyerUserAccount
        );
        assert.equal(buyerUser.nftBought.toNumber(), 1);

        // Check that buyer's lamport balance decreased by at least the sale amount.
        const buyerBalanceAfter = await connection.getBalance(buyer.publicKey);
        assert(
            buyerBalanceBefore - buyerBalanceAfter >= LAMPORTS_PER_SOL,
            "Buyer lamports did not decrease appropriately"
        );
    });
});

const confirmTx = async (signature: string) => {
    const blockHash = await anchor
        .getProvider()
        .connection.getLatestBlockhash();
    await anchor.getProvider().connection.confirmTransaction(
        {
            signature,
            ...blockHash,
        },
        "confirmed"
    );
    return signature;
};
