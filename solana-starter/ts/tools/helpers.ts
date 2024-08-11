import { findProgramAddressSync } from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { PublicKey, Connection, Keypair, Commitment, LAMPORTS_PER_SOL } from "@solana/web3.js";

  // Commitment
  const commitment: Commitment = "finalized";

  // Create a devnet connection
  const connection = new Connection("https://api.devnet.solana.com");

//Log tx to console. isTxHash is used to know if we are printing account info vs tx info.
export const logTx = (input: String, isTxHash: boolean, display: String) => {
    if (isTxHash) {
        const addressLink = `https://solana.fm/tx/${input}?cluster=devnet-solana`;
        console.log(`${display}: ${addressLink}`);
    }
    else {
        console.log(`${display}: ${input}`);
        const transactionLink = `https://solana.fm/address/${input}/transactions?cluster=devnet-solana`;
        console.log(`${display}: ${transactionLink}`);
    }  
}

// export async function processTransaction(connection: Connection, transaction: VersionedTransaction): Promise<string> {
//     const signature = await connection.sendTransaction(transaction);
//     console.log(`Transaction sent: ${signature}`);
//     return signature;
// }

    export const isAccountPDA = (account: PublicKey) => {
        return !PublicKey.isOnCurve(account.toBytes())
    }

    export const getBalance = async (pubKey: PublicKey) => {
        return await connection.getBalance(pubKey)/LAMPORTS_PER_SOL;
        
    }

    