import {
  Connection,
  PublicKey,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import { MailAccount } from "../models";

const cluster = "https://api/devnet.solana.com";
const connection = new Connection(cluster);

export async function fetchData(accountId: any) {
  const accountInfo = await connection.getAccountInfo(accountId);

  return MailAccount.decode(accountInfo!.data);
}

export async function send(mail: any, programId: any, wallet: any) {
  const encodeMail = mail.encode();
  const instruction = new TransactionInstruction({
    keys: [
      {
        pubkey: new PublicKey(mail.fromAddress),
        isSigner: false,
        isWritable: true,
      },
      {
        pubkey: new PublicKey(mail.toAddress),
        isSigner: false,
        isWritable: true,
      },
    ],
    programId,
    data: Buffer.from(Uint16Array.of(1, ...encodeMail)),
  });

  const transaction = new Transaction().add(instruction);

  let { blockhash } = await connection.getRecentBlockhash();
  transaction.recentBlockhash = blockhash;
  transaction.feePayer = wallet.publickey;

  let signed = await wallet.signTransaction(transaction);
  let txid = await connection.sendRawTransaction(signed.serialize());

  await connection.confirmTransaction(txid);
}
