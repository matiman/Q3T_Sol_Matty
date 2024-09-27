import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DevLoot } from "../target/types/dev_loot";
import { Keypair, PublicKey } from "@solana/web3.js";
import { utf8 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { confirm, log } from "./helper";
import { getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, generateSigner, keypairIdentity, KeypairSigner, percentAmount } from "@metaplex-foundation/umi";
import { createNft, findMasterEditionPda, findMetadataPda, mplTokenMetadata, verifySizedCollectionItem } from "@metaplex-foundation/mpl-token-metadata";

describe("dev-loot", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;

  const program = anchor.workspace.DevLoot as Program<DevLoot>;

  const [admin, studentA, studentB] = [new Keypair(), new Keypair(), new Keypair()];

  //for UMI since it only exists in Mainnet ?
  const RPC_ENDPOINT = "https://api.devnet.solana.com";

  const umi = createUmi(RPC_ENDPOINT);

  const payer = provider.wallet as NodeWallet;

  let nftMint: KeypairSigner = generateSigner(umi);
  let collectionMint: KeypairSigner = generateSigner(umi);

  const creatorWallet = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(payer.payer.secretKey));
  const creator = createSignerFromKeypair(umi, creatorWallet);
  umi.use(keypairIdentity(creator));
  umi.use(mplTokenMetadata());

  const collection = new anchor.web3.PublicKey(collectionMint.publicKey.toString());
  const diamondNft = new anchor.web3.PublicKey(nftMint.publicKey.toString());
  //atas
  let sellerAta: PublicKey, buyerAta: PublicKey;

  //vault to hold the NFT listed from seller
  let vaultAtaA: PublicKey;

  const TOKEN_DECIMALS = 6;
  
  //marketplace PDA with seed
  const courseId: number = 100
  const course = "course_config";
  
  const [solanaCourseConfig,courseConfigBump] =  anchor.web3.PublicKey.findProgramAddressSync([
        utf8.encode(course), Buffer.from([courseId])],
        program.programId);
  console.log(`\nCourse Config PDA : ${solanaCourseConfig.toBase58()}`);

  const [goldRewardsMint,goldRewardsMintBump] =  anchor.web3.PublicKey.findProgramAddressSync([
    utf8.encode("gold_rewards_mint"), solanaCourseConfig.toBuffer()],
    program.programId);
  console.log(`\nGoldMint PDA : ${goldRewardsMint.toBase58()}`);

  const [diamondRewardsMint,diamondRewardsMintBump] =  anchor.web3.PublicKey.findProgramAddressSync([
    utf8.encode("diamond_rewards_mint"), solanaCourseConfig.toBuffer()],
    program.programId);
  console.log(`\nDiamondMint PDA : ${diamondRewardsMint.toBase58()}`);

  const [studentAAccount,studentAAccountBump] =  anchor.web3.PublicKey.findProgramAddressSync([
    utf8.encode("student"),
    studentA.publicKey.toBuffer(),
    ],
    program.programId);
  console.log(`\nStudent PDA for StudentA: ${studentAAccount.toBase58()}`);

  const [studentBAccount,studentBAccountBump] =  anchor.web3.PublicKey.findProgramAddressSync([
    utf8.encode("student"),
    studentB.publicKey.toBuffer(),
    ],
    program.programId);
  console.log(`\nStudent PDA for StudentB: ${studentBAccount.toBase58()}`);

  const [studentASolanaProgress,studentAProgressBump] =  anchor.web3.PublicKey.findProgramAddressSync([
    utf8.encode("student_progress"),
    studentA.publicKey.toBuffer(),
    solanaCourseConfig.toBuffer()
    ],
    program.programId);
  console.log(`\nSolana Course ProgressPDA for StudentA: ${studentASolanaProgress.toBase58()}`);

  const [studentBSolanaProgress,studentBProgressBump] =  anchor.web3.PublicKey.findProgramAddressSync([
    utf8.encode("student_progress"),
    studentB.publicKey.toBuffer(),
    solanaCourseConfig.toBuffer()
    ],
    program.programId);
  console.log(`\nSolana Course ProgressPDA for StudentB: ${studentASolanaProgress.toBase58()}`);

  const [stakeConfig,stakeConfigBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
  );

  const [stakeRewardsMint,stakeRewardsBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("stake_rewards"),
      stakeConfig.toBuffer()
    ],
    program.programId
  );

  const [studentAstakeAccount, studentAstakeAccountBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("stake_account"),
      diamondNft.toBuffer(),
      stakeConfig.toBuffer()
    ],
    program.programId
  );
  
    //Airdrop SOL to intialize accounts
    it("Airdrop", async () => {
      await Promise.all([admin, studentA, studentB].map(async (k) => {
        await connection.requestAirdrop(k.publicKey, 100 * anchor.web3.LAMPORTS_PER_SOL).then(confirm).then(log);
      }))
    });
  
    it("Mint Diamond Rewards Collection NFT", async () => {
      await createNft(umi, {
          mint: collectionMint,
          name: "Diamond Rewards Collection",
          symbol: "DMNDC",
          uri: "https://arweave.net/DMNDC",
          sellerFeeBasisPoints: percentAmount(5),
          creators: null,
          collectionDetails: { 
            __kind: 'V1', size: 10,
          }
      }).sendAndConfirm(umi)
      console.log(`Created Collection NFT: ${collectionMint.publicKey.toString()}`)
    });
  
    it("Mint Diamond Rewards NFT", async () => {
      await createNft(umi, {
          mint: nftMint,
          name: "Diamond Rewards",
          symbol: "DMND",
          uri: "https://arweave.net/DMND",
          sellerFeeBasisPoints: percentAmount(5),
          collection: {verified: false, key: collectionMint.publicKey},
          creators: null,
      }).sendAndConfirm(umi)
      console.log(`\nCreated NFT: ${nftMint.publicKey.toString()}`)
    });
  
    it("Verify Diamond Collection NFT", async () => {
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
  
  //TODO add asserts
  it("Is course configuration initialized ? ", async () => {
    const totalQuestions = 5;
    const lastContentIndex = 10;
    const minimumPointsForReward = 80;
    // Add your test here.
    const tx = await program.methods
      .initConfig(courseId, lastContentIndex, totalQuestions, minimumPointsForReward)
      .accountsPartial({
        admin: admin.publicKey,
        courseConfig: solanaCourseConfig,
        diamondRewardsMint,
        goldRewardsMint,
        tokenProgram: TOKEN_PROGRAM_ID
    }).signers([admin])
      .rpc().then(confirm).then(log);;
    //console.log("Your transaction signature", tx);
  });

  //TODO add asserts
  it("Is student enrolled for free ? ", async () => {
    // Add your test here.
    //wallet, full_name, course_id
    const studentAFullName = "Mathias Abraham"
    const tx = await program.methods
      .enrollFreeStudent(studentA.publicKey,studentAFullName)
      .accountsPartial({
        student: studentA.publicKey,
        studentAccount: studentAAccount,
        studentProgress: studentASolanaProgress,
        courseConfig: solanaCourseConfig,
    }).signers([studentA])
      .rpc().then(confirm).then(log);;
    //console.log("Your transaction signature", tx);
  });

    //TODO add asserts
    it("Is student enrolled paid ? ", async () => {
      // Add your test here.
      //wallet, full_name, course_id
      const studentBFullName = "Jennifer Lopez"
      const tx = await program.methods
        .enrollPaidStudent(studentB.publicKey,studentBFullName)
        .accountsPartial({
          student: studentB.publicKey,
          studentAccount: studentBAccount,
          studentProgress: studentBSolanaProgress,
          courseConfig: solanaCourseConfig,
      }).signers([studentB])
        .rpc().then(confirm).then(log);;
      //console.log("Your transaction signature", tx);
    });

  it("Bulk update student progress ", async () => {
    // Add your test here.
    //wallet, full_name, course_id
    const studentAFullName = "Mathias Abraham"
    const tx = await program.methods
      .bulkUpdateStudentProgress(2,2)
      .accountsPartial({
        student: studentA.publicKey,
        studentAccount: studentAAccount,
        studentProgress: studentASolanaProgress,//contains course id
        courseConfig: solanaCourseConfig,
    }).signers([studentA])
      .rpc().then(confirm).then(log);;
    //console.log("Your transaction signature", tx);
  });

  it(" update student score ", async () => {
    // Add your test here.
    //wallet, full_name, course_id
    const studentAFullName = "Mathias Abraham"
    const tx = await program.methods
      .updateScore(2,4)
      .accountsPartial({
        student: studentA.publicKey,
        studentAccount: studentAAccount,
        studentProgress: studentASolanaProgress,
        courseConfig: solanaCourseConfig,
    }).signers([studentA])
      .rpc().then(confirm).then(log);;
    //console.log("Your transaction signature", tx);
  });

  it("update content index ", async () => {
    // Add your test here.
    //wallet, full_name, course_id
    const studentAFullName = "Mathias Abraham"
    const tx = await program.methods
      .updateContentPointer(8)
      .accountsPartial({
        student: studentA.publicKey,
        studentAccount: studentAAccount,
        studentProgress: studentASolanaProgress,//this contains the course id
        courseConfig: solanaCourseConfig,
    }).signers([studentA])
      .rpc().then(confirm).then(log);
    //console.log("Your transaction signature", tx);
  });

  it("complete course ", async () => {
    // Add your test here.
    //wallet, full_name, course_id
    const studentAFullName = "Mathias Abraham"
    const tx = await program.methods
      .completeCourse()
      .accountsPartial({
        student: studentA.publicKey,
        studentAccount: studentAAccount,
        studentProgress: studentASolanaProgress,//this contains the course id
        courseConfig: solanaCourseConfig,
    }).signers([studentA])
      .rpc().then(confirm).then(log);
    //console.log("Your transaction signature", tx);
  });

  const mintAtaStudent = getAssociatedTokenAddressSync(diamondNft, studentA.publicKey);

  it("Stake NFT", async () => {
    const nftMetadata = findMetadataPda(umi, {mint: nftMint.publicKey});
    const nftEdition = findMasterEditionPda(umi, { mint: nftMint.publicKey });
    
    const tx = program.methods.stakeDiamondNft()
      .accountsPartial(
        {
          student: studentA.publicKey,
          diamondRewardsMint: diamondNft,
          collectionMint: collection,
          courseConfig:solanaCourseConfig,
          mintAtaStudent,
          stakeAccount: studentAstakeAccount,
          metadata: new anchor.web3.PublicKey(nftMetadata[0]),
          edition: new anchor.web3.PublicKey(nftEdition[0]),
          tokenProgram: TOKEN_PROGRAM_ID

        }
    ).rpc().then(confirm).then(log)
  });

  it("Unstake Diamond NFT", async () => {
    const userTokenAta = getAssociatedTokenAddressSync(diamondNft, provider.wallet.publicKey);
    const nftEdition = findMasterEditionPda(umi, { mint: nftMint.publicKey });

    console.log("\n NFT unstaked by User!");
    const tx = program.methods.unstakeDiamondNft()
      .accountsPartial(
        {
          student: provider.wallet.publicKey,
          studentTokenAta: studentAAccount,
          diamondRewardsMint: diamondNft,
          courseConfig: solanaCourseConfig,
          studentProgress: studentASolanaProgress,
          stakeConfig,
          stakeAccount: studentAstakeAccount,
          edition: new anchor.web3.PublicKey(nftEdition[0]),
          tokenProgram: TOKEN_PROGRAM_ID
        }
    ).rpc().then(confirm).then(log)
    
    
    });
  });
  
