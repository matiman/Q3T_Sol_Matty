import * as anchor from "@coral-xyz/anchor";

anchor.setProvider(anchor.AnchorProvider.env());
const provider = anchor.getProvider();
const connection = provider.connection;

export const confirm = async (signature: string): Promise<string>  => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature,
      ...block
    });
  
    return signature;
  }

 export const log = async (signature: string): Promise<string>  => {
    console.log(`\nYour tx signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=http%3A%2F%2Flocalhost%3A8899`);
  
    return signature;
  }