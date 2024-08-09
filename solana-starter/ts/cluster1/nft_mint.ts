import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../../../wba-wallet.json"
import base58 from "bs58";
import { logTx } from "../tools/helpers";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

//another keypair to add as a creator
let creatorUmiKeyPair = umi.eddsa.generateKeypair();

(async () => {
    let tx = createNft(
        umi, 
        {
          mint,
          name:"OHM",
          symbol: "OM",
          //metadata URI not image URI
          uri: "https://arweave.net/7tx7vATzeIFTh1c8P6pQ45zp8VcG1b2J5Tl-IL0GMb4",
          creators: [
            {
                    address: keypair.publicKey,
                    verified: true,
                    share: 80
            },
            {
                    address: creatorUmiKeyPair.publicKey,
                    verified: false,
                    share: 20
            }
            ],
          sellerFeeBasisPoints: percentAmount(10),
        },
    ) 

    //TODO Take out logging and sending tx to a separate method for all files.

    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);

    logTx(signature, true, "Successfuly minted your NFT: ");

    logTx( mint.publicKey,false,"Mint Address: ");
})();