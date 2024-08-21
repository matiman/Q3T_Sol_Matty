import * as anchor from "@coral-xyz/anchor";
import { Program, BN } from "@coral-xyz/anchor";
import { AnchorEscrow } from "../target/types/anchor_escrow";
import { Keypair } from "@solana/web3.js";
import { createMint, getAccount, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { confirm, log } from "./helper";
import { PublicKey } from "@solana/web3.js";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { randomBytes } from "crypto";
import { assert } from "chai";

//TODO abstract out intitalization of accounts to an outer function
describe("anchor-escrow", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;

  const program = anchor.workspace.AnchorEscrow as Program<AnchorEscrow>;

  //Initialization of the necessary accounts, mints, atas,escrow, and vault.
  //maker and taker
  const [maker, taker] = [new Keypair(), new Keypair()];

  //mints
  let mintA: PublicKey ,mintB: PublicKey;
  //let mintB: PublicKey;

  //atas
  let makerAtaA: PublicKey, makerAtaB: PublicKey;
  let takerAtaA: PublicKey, takerAtaB: PublicKey;
  let vaultAtaA: PublicKey;

  const TOKEN_DECIMALS = 6;
  
  //escrow PDA with seed
  const seed = new BN(randomBytes(8));
  const escrow = findProgramAddressSync(
    [Buffer.from("escrow"), maker.publicKey.toBuffer(),seed.toBuffer('le',8)],
    program.programId)[0];
  console.log(`\nEscrow PDA : ${escrow.toBase58()}`);
  
  
  //vault to hold mintA from maker
  //since the owner is escrow, a PDA, we set allowOwerOffCurve to true

  //Airdrop SOL to intialize accounts
  it("Airdrop", async () => {
    await Promise.all([maker, taker].map(async (k) => {
      await connection.requestAirdrop(k.publicKey, 100 * anchor.web3.LAMPORTS_PER_SOL).then(confirm).then(log);
    }))
  });

  //Airdrop SOL to intialize accounts
  it("Prepare and Intialize accounts ", async () => {
    //create mints
    mintA = await createMint(connection, maker, maker.publicKey, null,TOKEN_DECIMALS);
    mintB = await createMint(connection, taker, taker.publicKey, null, TOKEN_DECIMALS);
    
    //derive vault. since owner is escrow, a PDA, allowOffCurve is true
    vaultAtaA = getAssociatedTokenAddressSync(mintA, escrow, true);
    console.log(`VaultAtaA: ${vaultAtaA.toBase58()}`);

    //maker and taker atas
    makerAtaA = (await getOrCreateAssociatedTokenAccount(connection, maker, mintA, maker.publicKey)).address;
    makerAtaB = (await getOrCreateAssociatedTokenAccount(connection, maker, mintB, maker.publicKey)).address;
    takerAtaA = (await getOrCreateAssociatedTokenAccount(connection, taker, mintA, taker.publicKey)).address;
    takerAtaB = (await getOrCreateAssociatedTokenAccount(connection, taker, mintB, taker.publicKey)).address;

    //mint to makerAtaA and takerAtaB to hold their initial balance before swap
    await mintTo(connection, maker, mintA, makerAtaA, maker, 10000000000).then(log);
    await mintTo(connection, taker, mintB, takerAtaB, taker, 25000000000);

    const makerBalanceA = (await connection.getTokenAccountBalance(makerAtaA)).value.amount;
    const takerBalanceB = (await connection.getTokenAccountBalance(takerAtaB)).value.amount;

    console.log(`\nmakerAtaA ${makerAtaA.toBase58()} token balance: ${makerBalanceA}`);
    console.log(`takerAtaB ${makerAtaB.toBase58()} token balance: ${takerBalanceB}`);

    const makerLamport = await connection.getBalance(makerAtaA);
    const takerLamport = await connection.getBalance(makerAtaA);

    console.log(`\nmaker SOL balance: ${makerLamport / 1e9} SOL`);
    console.log(`taker SOL balance: ${takerLamport/1e9} SOL`);
    
  });

  it("Intialize Escorw and Make!", async () => {

    const deposit = new BN(100000000);
    const recieve = new BN(500000000);

    //Anchor .30 and above doesnt need to specify accounts and signers
    const tx = await program.methods.initialize(seed, recieve,deposit)
    .accountsPartial({
      maker: maker.publicKey,
      mintA,
      mintB,
      makerAtaA,
      vaultAtaA,
      escrow,
      tokenProgram: TOKEN_PROGRAM_ID,
      //TOD systems program and associated token program is generated in the IDL
      //so we don't need them ? 
      //associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      //SystemProgram: SystemProgram.programId,
    })
      .signers([maker])
      .rpc().then(log);
    //console.log("Your transaction signature", tx);
    const vaultBalanceA = new BN((await connection.getTokenAccountBalance(vaultAtaA)).value.amount);

    console.log(`\ndeposit amount is ${deposit}`);
    console.log(`vaultAtaA ${vaultAtaA.toBase58()} token balance: ${vaultBalanceA}`);

    //assert if deposit has been made from makerAtaA to vaultAtaA
    assert.deepStrictEqual(deposit, vaultBalanceA, `Vault should have deposit amount of ${deposit}`);
    //TODO more assertion including negative assertions

  });

  it("Taker depositing to maker, reciving takers amount and closing account!", async () => {

    const deposit = new BN(500000000);

    console.log(`vaultAtaA exists before closing vault account:`);
    const vaultAccountInfoBeforeClose = await connection.getAccountInfo(vaultAtaA);
    assert.isNotNull(vaultAccountInfoBeforeClose);

    //Anchor .30 and above doesnt need to specify accounts and signers
    const tx = await program.methods.takeAndClose()
    .accountsPartial({
      maker: maker.publicKey,
      taker: taker.publicKey,
      mintA,
      mintB,
      makerAtaB,
      takerAtaA,
      takerAtaB,
      vaultAtaA,
      escrow,
      tokenProgram: TOKEN_PROGRAM_ID,
      //TOD systems program and associated token program is generated in the IDL
      //so we don't need them ? 
      //associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      //SystemProgram: SystemProgram.programId,
    })
      .signers([taker])
      .rpc().then(log);
    
    const makerAtaBBalance = new BN((await connection.getTokenAccountBalance(makerAtaB)).value.amount);
  
    console.log(`makerAtaB: ${makerAtaB.toBase58()} token balance: ${makerAtaBBalance}`);

    console.log(`vaultAtaA is emptied after closing vault account:`);
    const vaultAccountInfoAfterClose = await connection.getAccountInfo(vaultAtaA);
    assert.isNull(vaultAccountInfoAfterClose);

    //assert if deposit has been made from taker to vaultAtaA
    assert.deepStrictEqual(deposit, makerAtaBBalance, ` MakerAtaB should recieve amount of ${deposit}`);
    //TODO more assertion 

  });

});


