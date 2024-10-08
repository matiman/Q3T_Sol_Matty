import {
  Connection,
  Keypair,
  Commitment,
} from "@solana/web3.js";
import { Program, Wallet, AnchorProvider, Address } from "@coral-xyz/anchor";
import { WbaVault, IDL } from "./programs/wba_vault";
import wallet from "../../../wba-wallet.json";
import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { isAccountPDA } from "../tools/helpers";

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
    const program = new Program<WbaVault>(
      IDL,
      "D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o" as Address,
      provider,
    );

    // Create a random keypair
    const vaultKeyPair = Keypair.generate();
    console.log(`Vault public key: ${vaultKeyPair.publicKey.toBase58()}`);

    // Create the PDA for our enrollment account
    // Seeds are "auth", vaultState
    const vault_seeds = [Buffer.from('auth'), vaultKeyPair.publicKey.toBuffer()];

    const vaultAuth = findProgramAddressSync(vault_seeds, program.programId)[0];
    console.log(`vaultAuth: ${vaultAuth}`) 
    // Create the vault key
    // Seeds are "vault", vaultAuth
    const vault = findProgramAddressSync([
      Buffer.from("vault"), vaultAuth.toBuffer()],
      program.programId)[0];
 
    console.log(`Vault : ${vault} is a PDA: ${isAccountPDA(vault)} `);
  
    // Execute our enrollment transaction
    (async () => {
      try {
        const signature = await program.methods.initialize()
          .accounts({
          owner: WBAKeypair.publicKey,
          vault,
          vaultAuth,
          vaultState: vaultKeyPair.publicKey
        }).signers([WBAKeypair, vaultKeyPair]).rpc();
        console.log(`Init success! Check out your TX here:\n\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`);

      } catch (e) {
        console.error(`Oops, something went wrong: ${e}`);
      }

  })();

