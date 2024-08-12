import { Commitment, Connection, Keypair, PublicKey } from "@solana/web3.js"
import wallet from "../../../wba-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";
import { logTx } from "../tools/helpers";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("DCKaJDeNRp2Jaoy1ujTLgeSnCkb38TzNW9vsBYwq9UWZ");

// Recipient address
const to = new PublicKey("4J7kYZUjgWz4r1TA82tYup8xxrvqRBXuvWyhEPdN1pfa");

const token_decimals = 1_000_000n;

(async () => {
    try {
        // Get the token account of the fromWallet address, and if it does not exist, create it
        const fromAta = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        );

        // Get the token account of the toWallet address, and if it does not exist, create it
        const toAta = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to
        );

        // Transfer the new token to the "toTokenAccount" we just created

        const tx = await transfer(
            connection,
            keypair,
            fromAta.address,
            toAta.address,
            keypair,
            10n*token_decimals
        )

        logTx(tx,true,"Your transfer tx: ")
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();