import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { AnchorMarketplace } from "../target/types/anchor_marketplace";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { randomBytes } from "crypto";
import { assert, expect } from "chai";
import { createMint, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { confirm, log } from "./helper";
import * as utf8 from "@coral-xyz/anchor/dist/cjs/utils/bytes/utf8";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { createNft, findMasterEditionPda, findMetadataPda, mplTokenMetadata, verifySizedCollectionItem } from '@metaplex-foundation/mpl-token-metadata'
import { createSignerFromKeypair, generateSigner, keypairIdentity, KeypairSigner, percentAmount } from "@metaplex-foundation/umi";
import crypto from 'crypto';
describe("anchor-marketplace", () => {

  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;

  const program = anchor.workspace.AnchorMarketplace as Program<AnchorMarketplace>;

  //Initialization of the necessary accounts, mints, atas, listing, marketplace and vault.
  //maker and taker
  const [adminKeyPair, seller, buyer] = [new Keypair(), new Keypair(), new Keypair()];
  
  const umi = createUmi(provider.connection);

  //mint to be listed by seller
  let nftMint: KeypairSigner = generateSigner(umi);
  //collection the mint should be checked against.
  let collectionMint: KeypairSigner = generateSigner(umi);

  const adminWallet = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(adminKeyPair.secretKey));
  const adminSigner = createSignerFromKeypair(umi, adminWallet);
  umi.use(keypairIdentity(adminSigner));
  umi.use(mplTokenMetadata());

  //atas
  let sellerAta: PublicKey, buyerAta: PublicKey;

  //vault to hold the NFT listed from seller
  let vaultAtaA: PublicKey;

  const TOKEN_DECIMALS = 6;
  
  //marketplace PDA with seed
  const marketplaceName = "TensorFlow";
  const [marketplace,marketplaceBump] =  anchor.web3.PublicKey.findProgramAddressSync([
    utf8.encode("marketplace"), utf8.encode(marketplaceName)],
    program.programId);
  console.log(`\nMarkeplace PDA : ${marketplace.toBase58()}`);
  console.log(`\nMarkeplace PDA : ${marketplace.toString()}`);

  //treasury PDA with seed
  const treasury =  anchor.web3.PublicKey.findProgramAddressSync([
     utf8.encode("treasury"), marketplace.toBuffer()],
     program.programId)[0];
  console.log(`\nTreasury PDA : ${treasury.toBase58()}`);

  //rewards
  const [rewards, rewardsBump] =  anchor.web3.PublicKey.findProgramAddressSync(
      [utf8.encode("rewards"), marketplace.toBuffer()],
      program.programId
  );
  console.log(`\nReward PDA : ${rewards.toBase58()}`);
  

  //Airdrop SOL to intialize accounts
  it("Airdrop", async () => {
    await Promise.all([adminKeyPair, seller, buyer].map(async (k) => {
      await connection.requestAirdrop(k.publicKey, 100 * anchor.web3.LAMPORTS_PER_SOL).then(confirm).then(log);
    }))
  });

  it("Mint Madlads Collection NFT", async () => {
    await createNft(umi, {
        mint: collectionMint,
        name: "MadLads Collection",
        symbol: "MDCL",
        uri: "https://arweave.net/madlads",
        sellerFeeBasisPoints: percentAmount(5),
        creators: null,
        collectionDetails: { 
          __kind: 'V1', size: 10,
        }
    }).sendAndConfirm(umi)
    console.log(`Created Collection NFT: ${collectionMint.publicKey.toString()}`)
  });
  
  it("Mint Madlad NFT", async () => {
    await createNft(umi, {
        mint: nftMint,
        name: "MadLad",
        symbol: "MD",
        uri: "https://arweave.net/madlads/1",
      sellerFeeBasisPoints: percentAmount(5),
        //make sure verified is true 
        collection: {verified: true, key: collectionMint.publicKey},
        creators: null,
    }).sendAndConfirm(umi)
    console.log(`\nCreated Mad Lads NFT: ${nftMint.publicKey.toString()}`)
  });

  let hexString = crypto.createHash('sha256').update(marketplace.toString(),'utf-8').digest('hex');
  let seed = Uint8Array.from(Buffer.from(hexString,'hex'));
  //listing
  const [listing, listingBump] = anchor.web3.PublicKey.findProgramAddressSync([
    marketplace.toBuffer(),
    utf8.encode(collectionMint.publicKey)
  ],
    program.programId
  );
  

  //Airdrop SOL to intialize accounts
  it.skip("Prepare and Intialize accounts ", async () => {
    //create mints
    //mintA = await createMint(connection, maker, maker.publicKey, null,TOKEN_DECIMALS);
    //mintB = await createMint(connection, taker, taker.publicKey, null, TOKEN_DECIMALS);
    
    //derive vault. since owner is escrow, a PDA, allowOffCurve is true
    // vaultAtaA = getAssociatedTokenAddressSync(mintA, escrow, true);
    // console.log(`VaultAtaA: ${vaultAtaA.toBase58()}`);

    //maker and taker atas
    // makerAtaA = (await getOrCreateAssociatedTokenAccount(connection, maker, mintA, maker.publicKey)).address;
    // makerAtaB = (await getOrCreateAssociatedTokenAccount(connection, maker, mintB, maker.publicKey)).address;
    // takerAtaA = (await getOrCreateAssociatedTokenAccount(connection, taker, mintA, taker.publicKey)).address;
    // takerAtaB = (await getOrCreateAssociatedTokenAccount(connection, taker, mintB, taker.publicKey)).address;

    // //mint to makerAtaA and takerAtaB to hold their initial balance before swap
    // await mintTo(connection, maker, mintA, makerAtaA, maker, 10000000000).then(log);
    // await mintTo(connection, taker, mintB, takerAtaB, taker, 25000000000);

    // const makerBalanceA = (await connection.getTokenAccountBalance(makerAtaA)).value.amount;
    // const takerBalanceB = (await connection.getTokenAccountBalance(takerAtaB)).value.amount;

    // console.log(`\nmakerAtaA ${makerAtaA.toBase58()} token balance: ${makerBalanceA}`);
    // console.log(`takerAtaB ${makerAtaB.toBase58()} token balance: ${takerBalanceB}`);

    // const makerLamport = await connection.getBalance(makerAtaA);
    // const takerLamport = await connection.getBalance(makerAtaA);

    // console.log(`\nmaker SOL balance: ${makerLamport / 1e9} SOL`);
    // console.log(`taker SOL balance: ${takerLamport/1e9} SOL`);
    
  });

  it("Intialize Marketplace", async () => {

    const fee: number = 200; //2% 

    //Anchor .30 and above doesnt need to specify accounts and signers
    const tx = await program.methods.initialize(marketplaceName,fee)
      .accountsPartial({
        admin: adminKeyPair.publicKey,
        marketplace,
        treasury,
        rewardsMint:rewards,
        tokenProgram: TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
    })
      .signers([adminKeyPair])
      .rpc().then(log);
    console.log("Your transaction signature", tx);
    //const vaultBalanceA = new BN((await connection.getTokenAccountBalance(vaultAtaA)).value.amount);

    //console.log(`\ndeposit amount is ${deposit}`);
    //console.log(`vaultAtaA ${vaultAtaA.toBase58()} token balance: ${vaultBalanceA}`);

    //assert if deposit has been made from makerAtaA to vaultAtaA
    //assert.deepStrictEqual(deposit, vaultBalanceA, `Vault should have deposit amount of ${deposit}`);
    //TODO more assertion including negative assertions

    const marketplaceAccount = await program.account.marketplace.fetch(marketplace);
  
    expect(marketplaceAccount.name).to.be.equal(marketplaceName);
    expect(marketplaceAccount.salesFee).to.be.equal(fee);
    expect(marketplaceAccount.bump).to.be.equal(marketplaceBump);

    expect(marketplaceAccount.rewardsBump).to.be.equal(rewardsBump);

  });

});