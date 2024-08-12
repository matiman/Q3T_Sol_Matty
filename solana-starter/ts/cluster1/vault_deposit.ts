import {
  Connection,
  Keypair,
  SystemProgram,
  PublicKey,
  Commitment,
  LAMPORTS_PER_SOL
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
import { getBalance, isAccountPDA } from "../tools/helpers";

(async () => {

  // Import our keypair from the wallet file
  const WBAKeypair = Keypair.fromSecretKey(new Uint8Array(wallet));

  // Commitment
  const commitment: Commitment = "finalized";

  // Create a devnet connection
  const connection = new Connection("https://api.devnet.solana.com");

  // Create our anchor provider
  const provider = new AnchorProvider(connection, new Wallet(WBAKeypair), {
    commitment,
  });

  // Create our program
  const program = new Program<WbaVault>(IDL, "D51uEDHLbWAxNfodfQDv7qkp8WZtxrhi3uganGbNos7o" as Address, provider);
  
  const walletBalance = await getBalance(WBAKeypair.publicKey);
  console.log(`WBA Wallet ${WBAKeypair.publicKey} balance is ${walletBalance}} SOL`)

  // Create a random keypair
  const vaultKeyPair = new PublicKey("AxwDSqYAvmEUc5NKTpesh28ZbkETzVWptXhhh1z82wSz");
  console.log(`vault pubkey: ${vaultKeyPair}`);

  // Create the PDA for our enrollment account
  const vault_seeds = [Buffer.from('auth'),vaultKeyPair.toBuffer()];
  const vaultAuth = findProgramAddressSync(vault_seeds, program.programId)[0];
  
  console.log(`Your vaultAuth: ${vaultAuth}`);

  // Create the vault key
  const vault = findProgramAddressSync(
    [Buffer.from('vault'), vaultAuth.toBuffer()],
    program.programId)[0];

  console.log(`Your vault: ${vault} is a PDA?: ${isAccountPDA(vault)}`);  

  // Execute our enrollment transaction
  (async () => {
    try {
      //TODO For this to work I had to send SOL to vault since it had 0 when initialized
      //But I don't understand why vault
      const signature = await program.methods
      .deposit(new BN(LAMPORTS_PER_SOL))
        .accounts({
          owner: WBAKeypair.publicKey,
          vaultState: vaultKeyPair,
          vaultAuth,
          vault,
          systemProgram: SystemProgram.programId
      })
      .signers([
        WBAKeypair
      ]).rpc();
      console.log(`Deposit success! Check out your TX here:\n\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`);
    } catch (e) {
      console.error(`Oops, something went wrong: ${e}`);
    }
  })();
})();

