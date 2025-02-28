import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PikaVault } from "../target/types/pika_vault";
import { Keypair } from "@solana/web3.js";
import { assert } from "chai";


describe("pika-vault testing", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.PikaVault as Program<PikaVault>;
    let user = new Keypair();
    let marketplaceAuthority = new Keypair();
    let collectionMint = new Keypair();


    it('Airdrop for User and Marketplace Authority', async () => {
        await Promise.all(
            [user, marketplaceAuthority].map(async (k) => {
                return await anchor
                    .getProvider()
                    .connection.requestAirdrop(
                        k.publicKey,
                        10 * anchor.web3.LAMPORTS_PER_SOL,
                    )
                    .then(confirmTx)
            }),
        )
    })


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
    // Verify user account data
    const userAccount = await program.account.userAccount.fetch(userAccountPDA);
    assert.equal(
      userAccount.authority.toString(),
      user.publicKey.toString(),
      `Authority check failed`
    );
    assert.equal(userAccount.nftSold.toNumber(), 0, `NFT Sold check failed!`);
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


});
const confirmTx = async (signature: string) => {
  const blockHash = await anchor.getProvider().connection.getLatestBlockhash();
  await anchor.getProvider().connection.confirmTransaction(
    {
      signature,
      ...blockHash,
    },
    "confirmed"
  );
  return signature;
};

