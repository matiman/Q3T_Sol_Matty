import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DevLoot } from "../target/types/dev_loot";
import { Keypair, PublicKey } from "@solana/web3.js";
import { utf8 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { confirm, log } from "./helper";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

describe("dev-loot", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const connection = provider.connection;

  const program = anchor.workspace.DevLoot as Program<DevLoot>;

  const [admin, studentA, studentB] = [new Keypair(), new Keypair(), new Keypair()];

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

  const [studentASolanaProgress,studentAProgressBump] =  anchor.web3.PublicKey.findProgramAddressSync([
    utf8.encode("student_progress"),
    studentA.publicKey.toBuffer(),
    solanaCourseConfig.toBuffer()
    ],
    program.programId);
  console.log(`\nSolana Course ProgressPDA for StudentA: ${studentASolanaProgress.toBase58()}`);
  
    //Airdrop SOL to intialize accounts
    it("Airdrop", async () => {
      await Promise.all([admin, studentA, studentB].map(async (k) => {
        await connection.requestAirdrop(k.publicKey, 100 * anchor.web3.LAMPORTS_PER_SOL).then(confirm).then(log);
      }))
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
      .rpc();
    console.log("Your transaction signature", tx);
  });

  //TODO add asserts
  it("Is student enrolled ? ", async () => {
    // Add your test here.
    //wallet, full_name, course_id
    const studentAFullName = "Mathias Abraham"
    const tx = await program.methods
      .enrollStudent(studentA.publicKey,studentAFullName)
      .accountsPartial({
        student: studentA.publicKey,
        studentAccount: studentAAccount,
        studentProgress: studentASolanaProgress,
        courseConfig: solanaCourseConfig,
    }).signers([studentA])
      .rpc();
    console.log("Your transaction signature", tx);
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
      .rpc();
    console.log("Your transaction signature", tx);
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
      .rpc();
    console.log("Your transaction signature", tx);
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
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
