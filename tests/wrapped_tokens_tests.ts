import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HandmadeNaive } from "../target/types/handmade_naive";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_2022_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  createAssociatedTokenAccountInstruction,
  createMint,
  getAssociatedTokenAddressSync,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import { program } from "@coral-xyz/anchor/dist/cjs/native/system";
import { expect } from "chai";

export async function wrap_tokens(
  amount: number,
  decimals: number,
  wrapper: anchor.web3.PublicKey,
  approver: anchor.web3.PublicKey,
  owner: anchor.web3.Signer,
  user_token_account: anchor.web3.PublicKey,
  mint: anchor.web3.PublicKey,
  wrapper_token_holder: anchor.web3.PublicKey,
  program: Program<HandmadeNaive>,
  tokenProgram: anchor.web3.PublicKey = TOKEN_PROGRAM_ID
) {
  const tx = await program.methods
    .wrapTokens(new anchor.BN(amount), decimals)
    .accountsPartial({
      userTokenAccount: user_token_account,
      owner: owner.publicKey,
      wrapperTokenAccount: wrapper_token_holder,
      wrapperAccount: wrapper,
      approver: approver,
      mint: mint,
      tokenProgram: tokenProgram,
    })
    .signers([owner])
    .rpc();

  console.log(`Transfer of ${amount} tx : ${tx}`);
}
