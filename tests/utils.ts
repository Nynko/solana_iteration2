import * as anchor from "@coral-xyz/anchor";
import fs from "fs";
import { HandmadeNaive } from "../target/types/handmade_naive";
import { Program } from "@coral-xyz/anchor";

export function load_keypair(filename: string): anchor.web3.Keypair {
  const secret = JSON.parse(fs.readFileSync(filename).toString()) as number[];
  const secretKey = Uint8Array.from(secret);
  return anchor.web3.Keypair.fromSecretKey(secretKey);
}

export function sleep(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

export async function sendTransaction(
  from: anchor.web3.Keypair,
  to: anchor.web3.PublicKey,
  amount: number
) {
  const transaction = new anchor.web3.Transaction().add(
    anchor.web3.SystemProgram.transfer({
      fromPubkey: from.publicKey,
      toPubkey: to,
      lamports: amount,
    })
  );

  // Sign transaction, broadcast, and confirm
  const signature = await anchor.web3.sendAndConfirmTransaction(
    anchor.getProvider().connection,
    transaction,
    [from]
  );

  console.log(`Transfer to ${to.toBase58()} of ${amount} tx : ${signature}`);
}

export async function create_user_with_best_bump(
  program: Program<HandmadeNaive>,
  mint: anchor.web3.PublicKey
) {
  let user;
  let errorOccurred = true;
  const bump = 255;
  while (errorOccurred) {
    try {
      user = anchor.web3.Keypair.generate();
      anchor.web3.PublicKey.createProgramAddressSync(
        [
          Buffer.from("identity"),
          user.publicKey.toBuffer(),
          Buffer.from([bump]),
        ],
        program.programId
      );
      // anchor.web3.PublicKey.createProgramAddressSync(
      //   [
      //     Buffer.from('two_auth'),
      //     wrapper_account.toBuffer(),
      //     keypair.publicKey.toBuffer(),
      //     Buffer.from([bump]),
      //   ],
      //   programId,
      // );
      errorOccurred = false;
    } catch (error) {
      continue;
    }
  }

  return user;
}
