import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftStaking } from "../target/types/nft_staking";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { createSignerFromKeypair, generateSigner, keypairIdentity, KeypairSigner, percentAmount } from "@metaplex-foundation/umi";
import { createNft, findMasterEditionPda, findMetadataPda, mplTokenMetadata, verifySizedCollectionItem } from "@metaplex-foundation/mpl-token-metadata";
import { getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("nft-staking", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  //for UMI since it only exists in Mainnet ?
  const RPC_ENDPOINT = "https://api.devnet.solana.com";

  // console.log(`\n Provider: ${provider}`)
  // console.log(`\n Provider connection: ${provider.connection.rpcEndpoint}`)
  // console.log(`\n Provider pub key: ${provider.publicKey.toBuffer()}`)
  // console.log(`\n Provider wallet pub key: ${provider.wallet.publicKey}`)

  const program = anchor.workspace.NftStaking as Program<NftStaking>;

  const umi = createUmi(RPC_ENDPOINT);

  const payer = provider.wallet as NodeWallet;

  let nftMint: KeypairSigner = generateSigner(umi);
  let collectionMint: KeypairSigner = generateSigner(umi);

  const creatorWallet = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(payer.payer.secretKey));
  const creator = createSignerFromKeypair(umi, creatorWallet);
  umi.use(keypairIdentity(creator));
  umi.use(mplTokenMetadata());

  const collection = new anchor.web3.PublicKey(collectionMint.publicKey.toString());
  const nft = new anchor.web3.PublicKey(nftMint.publicKey.toString());

  const [config,configBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  );

  const [rewardsMint,rewardsBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("rewards"),
      config.toBuffer()
    ],
    program.programId
  );

  const [user,userBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("user"),
      provider.publicKey.toBuffer()
    ],
    program.programId
  );

  const [stake, stakeBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("stake"),
      nft.toBuffer(),
      config.toBuffer()
    ],
    program.programId
  );

  const mintAta = getAssociatedTokenAddressSync(nft, provider.wallet.publicKey);

  it("Mint MadLads Collection NFT", async () => {
    await createNft(umi, {
        mint: collectionMint,
        name: "MAD",
        symbol: "MAD",
        uri: "https://arweave.net/MAD",
        sellerFeeBasisPoints: percentAmount(5),
        creators: null,
        collectionDetails: { 
          __kind: 'V1', size: 10,
        }
    }).sendAndConfirm(umi)
    console.log(`Created Collection NFT: ${collectionMint.publicKey.toString()}`)
  });

  it("Mint MadLads NFT", async () => {
    await createNft(umi, {
        mint: nftMint,
        name: "MADS1",
        symbol: "MADS1",
        uri: "https://arweave.net/MADS1",
        sellerFeeBasisPoints: percentAmount(5),
        collection: {verified: false, key: collectionMint.publicKey},
        creators: null,
    }).sendAndConfirm(umi)
    console.log(`\nCreated NFT: ${nftMint.publicKey.toString()}`)
  });

  it("Verify Collection NFT", async () => {
    const collectionMetadata = findMetadataPda(umi, {mint: collectionMint.publicKey});
    const collectionMasterEdition = findMasterEditionPda(umi, {mint: collectionMint.publicKey});

    const nftMetadata = findMetadataPda(umi, {mint: nftMint.publicKey});
    await verifySizedCollectionItem(umi, {
      metadata: nftMetadata,
      collectionAuthority: creator,
      collectionMint: collectionMint.publicKey,
      collection: collectionMetadata,
      collectionMasterEditionAccount: collectionMasterEdition,
     }).sendAndConfirm(umi)
    console.log("\nCollection NFT Verified!");
  });

  it("Initialize configurations", async () => {
    const tx = await program.methods.initializeConfig(10, 1, 0)//change freeze set to "0" to test quickly
      .accountsPartial(
        {
        admin: provider.wallet.publicKey,
        config,
        rewardsMint,
        tokenProgram: TOKEN_PROGRAM_ID
      }
    ).rpc()
    console.log("\nConfig Account Initialized!");
    console.log("Your transaction signature", tx);

    //TODO Assert statements
    
  });

  it("Initialize User configurations", async () => {
    const tx = program.methods.intializeUser()
      .accountsPartial(
        {
        user: provider.wallet.publicKey,
        userAccount: user,
      }
    ).rpc()
    console.log("\nUser Account Initialized!");
    console.log("Your transaction signature", tx);

    //TODO Assert statements
  });

  it("Stake NFT", async () => {
    const nftMetadata = findMetadataPda(umi, {mint: nftMint.publicKey});
    const nftEdition = findMasterEditionPda(umi, { mint: nftMint.publicKey });
    
    const tx = program.methods.stake()
      .accountsPartial(
        {
          user: provider.wallet.publicKey,
          mint: nft,
          collectionMint: collection,
          config,
          mintAtaUser:mintAta,
          stakeAccount: stake,
          metadata: new anchor.web3.PublicKey(nftMetadata[0]),
          edition: new anchor.web3.PublicKey(nftEdition[0]),
          tokenProgram: TOKEN_PROGRAM_ID

        }
    ).rpc()
    console.log("\nNFT Staked!");
    console.log("Your transaction signature", tx);

    //TODO Assert statements
  });

  it("Claim Rewards", async () => {
    const userRewardsAta = getAssociatedTokenAddressSync(rewardsMint, provider.wallet.publicKey);
    const tx = program.methods.claim()
      .accountsPartial(
        {
          userAccount: provider.wallet.publicKey,
          user,
          config,
          rewardsMint,
          userRewardsAta,
          tokenProgram: TOKEN_PROGRAM_ID
        }
    ).rpc()
    console.log("\n Rewards claimed by User!");
    console.log("Your transaction signature", tx);

    //TODO Assert statements
  });

  it("Unstake NFT", async () => {
    const userTokenAta = getAssociatedTokenAddressSync(nft, provider.wallet.publicKey);
    const nftEdition = findMasterEditionPda(umi, { mint: nftMint.publicKey });

    const tx = program.methods.unstake()
      .accountsPartial(
        {
          userAccount: provider.wallet.publicKey,
          userTokenAta,
          user,
          mint: nft,
          config,
          stakeAccount: stake,
          edition: new anchor.web3.PublicKey(nftEdition[0]),
          tokenProgram: TOKEN_PROGRAM_ID
        }
    ).rpc()
    console.log("\n NFT unstaked by User!");
    console.log("Your transaction signature", tx);

    //TODO Assert statements
  });
});
