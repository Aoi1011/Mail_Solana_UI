import {
  Connection,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import Wallet from "@project-serum/sol-wallet-adapter";
import { accountConstants } from "../constants";

export const connectWallet = (seed: any) => {
  return async (dispatch: any, getState: any) => {
    dispatch(request());

    try {
      const programId = getState().account.programId;
      const { derivedAddress, wallet } = await createOrGetAccount(
        seed,
        programId
      );
    } catch (err) {}
  };
};

const request = () => {
  return { type: accountConstants.CREATE_REQUEST };
};

const success = (payload: any) => {
  return { type: accountConstants.CREATE_ACCOUNT_SUCCESSFUL, payload };
};

const failed = (payload: any) => {
  return { type: accountConstants.CREATE_ACCOUNT_FAILED, payload };
};

const cluster = "https://api/devnet.solana.com";
const connection = new Connection("https://api.devnet.solana.com");

async function createOrGetAccount(seed: any, programId: any) {
  let provideUrl = "https://www.sollet.io";
  let wallet = new Wallet(provideUrl, cluster);

  await wallet.connect();

  const derivedAddress = await PublicKey.createWithSeed(
    wallet.publicKey!,
    seed,
    programId
  );

  const mailAccount = await connection.getAccountInfo(derivedAddress);

  if (mailAccount === null) {
    const lamports = await connection.getMinimumBalanceForRentExemption(
      1000000
    );

    const createAccountInstruction = SystemProgram.createAccountWithSeed({
      fromPubkey: wallet.publicKey!,
      basePubkey: wallet.publicKey!,
      seed,
      newAccountPubkey: derivedAddress,
      lamports,
      space: 1000000,
      programId: programId,
    });

    const initAccountInstruction = new TransactionInstruction({
      keys: [{ pubkey: derivedAddress, isSigner: false, isWritable: true }],
      programId,
      data: Buffer.from([0]),
    });

    const transaction = new Transaction();
    transaction.add(createAccountInstruction).add(initAccountInstruction);

    let { blockhash } = await connection.getRecentBlockhash();
    transaction.recentBlockhash = blockhash;
    transaction.feePayer = wallet.publicKey!;

    let signed = await wallet.signTransaction(transaction);
    let txid = await connection.sendRawTransaction(signed.serialize());

    await connection.confirmTransaction(txid);
  }

  return {
    derivedAddress,
    wallet,
  };
}
