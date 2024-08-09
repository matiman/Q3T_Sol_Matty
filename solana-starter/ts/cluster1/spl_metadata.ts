import wallet from "../../../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey } from "@metaplex-foundation/umi";
import bs58 from 'bs58';
import { logTx } from "../tools/helpers";


// Define our Mint address
const mint = publicKey("2acDvtCTVMekV6oizYRkT6HT9NCCqVJZueZnSsLiaUKW")

// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));

//another keypair to add as a creator
let creatorUmiKeyPair = umi.eddsa.generateKeypair();

(async () => {
    try {
        // mint and mintAuthority are the required
        let accounts: CreateMetadataAccountV3InstructionAccounts = {
            mint,
            mintAuthority: signer
        }

        let data: DataV2Args = {
            name: "ODM",
            symbol: "ODM",
            uri: "www.dhamma.org/ohm",
            sellerFeeBasisPoints: 10,
            creators: [
                {
                    address: keypair.publicKey,
                    verified: false,
                    share: 80
                },
                {
                    address: creatorUmiKeyPair.publicKey,//
                    verified: false,
                    share: 20
                }
            ],
            collection: 
                {
                    verified: false,
                    //TODO Just putting pub key for test but this key is not a collection
                    key: keypair.publicKey
                    
                }
            ,
            uses: null
        }

        let args: CreateMetadataAccountV3InstructionArgs = {
            data,
            isMutable: false,
            collectionDetails: null

        }

        let tx = createMetadataAccountV3(
            umi,
            {
                ...accounts,
                ...args
            }
        )

        let result = await tx.sendAndConfirm(umi);
        logTx(`${bs58.encode(result.signature)}`,true,'Your tx detail: ')
        //console.log(`Your tx detail: ${bs58.encode(result.signature)}`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();
