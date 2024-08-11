import {
  Connection,
  Keypair,
  SystemProgram,
  PublicKey,
  Commitment,
} from "@solana/web3.js";
import {
  Program,
  Wallet,
  AnchorProvider,
  Address,
  BN,
} from "@coral-xyz/anchor";
import { WbaVault, IDL } from "./programs/wba_vault";
import wallet from "../../../wba-wallet.json";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";

// Import our keypair from the wallet file
const WBAkeypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Commitment
const commitment: Commitment = "finalized";

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(WBAkeypair), {
  commitment,
});

// Create our program
const program = new Program<WbaVault>(IDL, "D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o" as Address, provider);

// Create a random keypair
const vaultState = new PublicKey("GYH7AShPj4vGdefDHKwGMVzZcEoxx85WW2a2xkLM9C7Y");

// Create the PDA for our enrollment account
const vault_seeds = [Buffer.from("auth"),vaultState.toBuffer()];
const vaultAuth = findProgramAddressSync(vault_seeds, program.programId)[0];

// Create the vault key
const vault = findProgramAddressSync([Buffer.from("vault"), vaultAuth.toBuffer()],
            program.programId)[0];

// Mint address 
const mint = new PublicKey("D6Fofb79h8V59KWS7iAPbdttHawKTP2ubMBnVdPSsyYM");

// Execute our deposit transaction
(async () => {
  try {
    const metadataProgram = new PublicKey(
      "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s",
    );
    const metadataAccount = PublicKey.findProgramAddressSync(
      [Buffer.from("metadata"), metadataProgram.toBuffer(), mint.toBuffer()],
      metadataProgram,
    )[0];
    const masterEdition = PublicKey.findProgramAddressSync(
      [
        Buffer.from("metadata"),
        metadataProgram.toBuffer(),
        mint.toBuffer(),
        Buffer.from("edition"),
      ],
      metadataProgram,
    )[0];

    // b"metadata", MetadataProgramID.key.as_ref(), mint.key.as_ref() "master"
    // Get the token account of the fromWallet address, and if it does not exist, create it
    const ownerAta = await getOrCreateAssociatedTokenAccount(
      connection,
      WBAkeypair,
      mint,
      WBAkeypair.publicKey,    
    );

    // // Get the token account of the toWallet address, and if it does not exist, create it
    const vaultAta = await getOrCreateAssociatedTokenAccount(
      connection,
      WBAkeypair,
      mint,
      vaultAuth,
      true
    );

    const signature = await program.methods
    .depositNft()
      .accounts({
        owner: WBAkeypair.publicKey,
        ownerAta: ownerAta.address,
        vaultAta: vaultAta.address,
        vaultAuth,
        vaultState,
        tokenMint: mint,
        nftMetadata: metadataAccount,
        nftMasterEdition: masterEdition,
        metadataProgram
    })
    .signers([
        WBAkeypair
    ]).rpc();
    console.log(`Deposit success! Check out your TX here:\n\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
