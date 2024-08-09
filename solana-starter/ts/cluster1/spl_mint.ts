import { Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import wallet from "../../../wba-wallet.json"
import { logTx } from "../tools/helpers";

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const token_decimals = 1_000_000n;

// Mint address
const mint = new PublicKey("DCKaJDeNRp2Jaoy1ujTLgeSnCkb38TzNW9vsBYwq9UWZ");

(async () => {
    try {
        // Create an ATA from a mint
        const ata = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        );
        
        logTx(ata.address.toBase58(),false,'Your ata is:')
    
        // Mint to ATA
        const mintTx = await mintTo(
            connection,
            keypair,
            mint,
            ata.address,
            keypair,
            100n*token_decimals
            //TODO once minted to ata, sending a bigger supply caused Operation overflow.
        )
        logTx(mintTx,true,'Your mint txid:');
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
