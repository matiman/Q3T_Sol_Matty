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
import { Key } from "@metaplex-foundation/mpl-token-metadata";

// Import our keypair from the wallet file
const WBAKeypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Commitment
const commitment: Commitment = "confirmed";

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(WBAKeypair), {
  commitment,
});

// Create our program
const program = new Program<WbaVault>(IDL, "D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o" as Address, provider);

// Create a random keypair
//TODO Why we have vault and closeVaultState?
const vaultState = new PublicKey("2BeCXYnPDcogo3HYJjoTnh1WQxFxs2GQQRspD6vEh94K");

// Create a random keypair
const closeVaultState = new PublicKey("2BeCXYnPDcogo3HYJjoTnh1WQxFxs2GQQRspD6vEh94K");

(async () => {
  try {
    const signature = await program.methods
    .closeAccount()
    .accounts({
      owner: WBAKeypair.publicKey,
      closeVaultState: closeVaultState,
      vaultState,
      systemProgram: SystemProgram.programId
      
    })
    .signers([
      WBAKeypair
    ]).rpc();
    console.log(`Close success! Check out your TX here:\n\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();