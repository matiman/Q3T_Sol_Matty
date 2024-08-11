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
//import { WbaVault, IDL } from "../programs/wba";
import wallet from "../../../wba-wallet.json";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { WbaVault, IDL } from "./programs/wba_vault";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { logTx } from "../tools/helpers";

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

const token_decimals = 1_000_000n;

// Mint address
const mint = new PublicKey("DCKaJDeNRp2Jaoy1ujTLgeSnCkb38TzNW9vsBYwq9UWZ");

// Execute our enrollment transaction
(async () => {
  try {
    // Get the token account of the fromWallet address, and if it does not exist, create it
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
    const signature = await program.methods
    .depositSpl(new BN(1000000))
    .accounts({
      owner: WBAkeypair.publicKey,
      ownerAta: ownerAta.address,
      vaultState,
      vaultAuth,
      vaultAta: vaultAta.address,
      tokenMint: mint,
      systemProgram: SystemProgram.programId,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID
    })
    .signers([
        WBAkeypair
    ]).rpc();
    logTx(signature,true,"SPL Vault Deposit Sucess: ")
    
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();
