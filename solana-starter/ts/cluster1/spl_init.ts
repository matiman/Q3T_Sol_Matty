import { Keypair, Connection, Commitment } from "@solana/web3.js";
import { createMint } from '@solana/spl-token';
import wallet from "../../../wba-wallet.json"

import {logTx} from '../tools/helpers'

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

(async () => {
    try {
        // Start here
        //Create Mint using SPL-token library with 6 decimals.
        const mint = await createMint(
            connection,
            keypair,
            keypair.publicKey,
            null,
            6
        )
        logTx(mint.toBase58(), false,'Account Address is:');
        //DFQsP5a1kuciyBk4RVQ5F5BQwgnZkFmkC7nuUbynmTwx
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()


