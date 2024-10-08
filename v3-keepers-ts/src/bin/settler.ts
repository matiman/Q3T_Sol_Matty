/* eslint-disable @typescript-eslint/no-unused-vars */
import {
  ParclV3Sdk,
  getExchangePda,
  Address,
  ProcessSettlementRequestsAccounts,
  ProgramAccount,
  SettlementRequest,
  translateAddress,
} from "@parcl-oss/v3-sdk";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";
import { Commitment, Connection, Keypair, Signer } from "@solana/web3.js";
import bs58 from "bs58";
import { Helius } from "helius-sdk";
import * as dotenv from "dotenv";
dotenv.config({ path: ".env.example.settler" });

(async function main() {
  console.log("Starting settler");
  if (process.env.HELIUS_API_KEY === undefined) {
    throw new Error("Missing RPC API Key");
  }
  if (process.env.RPC_URL === undefined) {
    throw new Error("Missing rpc url");
  }
  console.log(`rpc: ${process.env.RPC_URL}`);
  if (process.env.PRIVATE_KEY === undefined) {
    throw new Error("Missing settler signer");
  }
  console.log(`pk: ${process.env.PRIVATE_KEY}`);
  // Note: only handling single exchange
  const [exchangeAddress] = getExchangePda(0);
  const payer = Keypair.fromSecretKey(bs58.decode(process.env.PRIVATE_KEY));
  console.log(`payer: ${payer.publicKey}`);
  const interval = parseInt(process.env.INTERVAL ?? "300");
  const commitment = process.env.COMMITMENT as Commitment | undefined;
  const sdk = new ParclV3Sdk({ rpcUrl: process.env.RPC_URL, commitment });
  const connection = new Connection(process.env.RPC_URL, commitment);
  const exchange = await sdk.accountFetcher.getExchange(exchangeAddress);
  const helius = new Helius(process.env.HELIUS_API_KEY);
  if (exchange === undefined) {
    throw new Error("Invalid exchange address");
  }
  const keeperTokenAccount = getAssociatedTokenAddressSync(
    translateAddress(exchange.collateralMint),
    payer.publicKey
  );
  await runSettler({
    sdk,
    connection,
    helius,
    interval,
    exchangeAddress,
    payer,
    keeperTokenAccount,
  });
})();

type RunSettlerParams = {
  sdk: ParclV3Sdk;
  connection: Connection;
  helius: Helius;
  interval: number;
  exchangeAddress: Address;
  keeperTokenAccount: Address;
  payer: Keypair;
};

async function runSettler({
  sdk,
  connection,
  helius,
  interval,
  exchangeAddress,
  keeperTokenAccount,
  payer,
}: RunSettlerParams): Promise<void> {
  let firstRun = true;
  // eslint-disable-next-line no-constant-condition
  while (true) {
    if (firstRun) {
      firstRun = false;
    } else {
      await new Promise((resolve) => setTimeout(resolve, interval * 1000));
    }
    const exchange = await sdk.accountFetcher.getExchange(exchangeAddress);
    if (exchange === undefined) {
      throw new Error("Invalid exchange address");
    }
    const allSettlementRequests = await sdk.accountFetcher.getAllSettlementRequests();
    console.log(`Fetched ${allSettlementRequests.length} settlement requests`);
    const now = new Date().getTime() / 1000;
    const matureSettlementRequests: ProgramAccount<SettlementRequest>[] = [];
    for (const settlementRequest of allSettlementRequests) {
      if (now >= settlementRequest.account.maturity) {
        matureSettlementRequests.push(settlementRequest);
      }
    }
    const chunkSize = 4;
    for (let i = 0; i < matureSettlementRequests.length; i += chunkSize) {
      const batch = matureSettlementRequests.slice(i, i + chunkSize);
      const settlementRequests: Address[] = [];
      const ownerTokenAccounts: Address[] = [];
      const owners: Address[] = [];
      for (const settlementRequest of batch) {
        settlementRequests.push(settlementRequest.address);
        ownerTokenAccounts.push(settlementRequest.account.ownerTokenAccount);
        owners.push(settlementRequest.account.owner);
      }
      const signature = await processSettlementRequest(
        sdk,
        connection,
        helius,
        {
          exchange: exchangeAddress,
          collateralVault: exchange.collateralVault,
          keeperTokenAccount,
          payer: payer.publicKey,
        },
        settlementRequests,
        ownerTokenAccounts,
        owners,
        [payer],
        payer.publicKey
      );
      console.log("Signature: ", signature);
    }
  }
}
async function processSettlementRequest(
  sdk: ParclV3Sdk,
  connection: Connection,
  helius: Helius,
  accounts: ProcessSettlementRequestsAccounts,
  settlementRequests: Address[],
  ownerTokenAccounts: Address[],
  owners: Address[],
  signers: Signer[],
  feePayer: Address
): Promise<string> {
  const { blockhash: recentBlockhash } = await connection.getLatestBlockhash();
  const tx = sdk
    .transactionBuilder()
    .processSettlementRequests(accounts, settlementRequests, ownerTokenAccounts, owners)
    .feePayer(feePayer)
    .buildSigned(signers, recentBlockhash);
  return await helius.rpc.sendSmartTransactionWithTip(tx.instructions, signers, [], 100000, "NY");
}
