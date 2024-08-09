import { VersionedTransaction } from "@solana/web3.js";
import { Connection, Transaction } from "@solana/web3.js";

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