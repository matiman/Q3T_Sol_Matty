import {
  Connection,
  Keypair,
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
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { getOrCreateAssociatedTokenAccount, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";

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
const vaultKeyPair = new PublicKey("GYH7AShPj4vGdefDHKwGMVzZcEoxx85WW2a2xkLM9C7Y");
  // Create the PDA for our enrollment account
  // Seeds are "auth", vaultState
const vault_seeds = [Buffer.from("auth"), vaultKeyPair.toBuffer()];
const vaultAuth = findProgramAddressSync(vault_seeds, program.programId)[0];

// Create the vault key
// Seeds are "vault", vaultAuth
const vault = findProgramAddressSync(
  [Buffer.from("vault"),
  vaultAuth.toBuffer()],
  program.programId)[0];

// Mint address
const mint = new PublicKey("DCKaJDeNRp2Jaoy1ujTLgeSnCkb38TzNW9vsBYwq9UWZ");

  // Execute our enrollment transaction
(async () => {
  const ownerAta = await getOrCreateAssociatedTokenAccount(
    connection,
    WBAkeypair,
    mint,
    WBAkeypair.publicKey
  );
  // Get the token account of the toWallet address, and if it does not exist, create it
  const vaultAta = await getOrCreateAssociatedTokenAccount(
    connection,
    WBAkeypair,
    mint,
    vaultAuth,
    true,
    commitment
  );
    try {
      const signature = await program.methods
      .withdrawSpl(new BN(100))
        .accounts({
          owner: WBAkeypair.publicKey,
          ownerAta: ownerAta.address,
          vaultState:vaultKeyPair,
          vaultAuth,
          vaultAta:vaultAta.address,
          tokenMint: mint,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_PROGRAM_ID
      })
      .signers([
        WBAkeypair
      ]).rpc();
      console.log(`Withdraw success! Check out your TX here:\n\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`);
    } catch (e) {
      console.error(`Oops, something went wrong: ${e}`);
    }
  }
)();
