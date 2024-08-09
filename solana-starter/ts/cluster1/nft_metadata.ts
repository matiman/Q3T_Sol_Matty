import wallet from "../../../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

//another keypair to add as a creator
let creatorUmiKeyPair = umi.eddsa.generateKeypair();

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const imageUri = "https://arweave.net/b3e-_2DAnrIFZ6EjbXZYkxzM_PkYXixAQa-a7jJYL7g"
        const metadata = {
            name: "OHM RUG",
            symbol: "OMRUG",
            description: "OHM about to get rugged NFT",
            image: imageUri,
            attributes: [
                { trait_type: 'When', value: 'Thanksgiving 2024' },
                {trait_type: 'Type', value: 'OMRUG'}
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: imageUri
                    },
                ]
            },
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
            ]
        };
        const myMetaDataUri = await umi.uploader.uploadJson(metadata);

        //const myUri = ???
        console.log("Your image metadata URI: ", myMetaDataUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
