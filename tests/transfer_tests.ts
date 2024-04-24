import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HandmadeNaive } from "../target/types/handmade_naive";

export async function transfer_wtokens(
  amount: number,
  wrapper_account: anchor.web3.PublicKey,
  source_owner: anchor.web3.Signer,
  source_wrapped_account: anchor.web3.PublicKey,
  destination_owner: anchor.web3.PublicKey,
  destination_wrapped_account: anchor.web3.PublicKey,
  two_auth: anchor.web3.PublicKey,
  two_auth_signer: anchor.web3.Signer | null,
  program: Program<HandmadeNaive>
) {
  const instruction = await program.methods
    .transfer(new anchor.BN(amount))
    .accountsPartial({
      sourceOwner: source_owner.publicKey,
      destinationOwner: destination_owner,
      sourceWrappedAccount: source_wrapped_account,
      destinationWrappedAccount: destination_wrapped_account,
      twoAuthSigner: two_auth_signer ? two_auth_signer.publicKey : null,
      twoAuth: two_auth,
      wrapperAccount: wrapper_account,
    })
    .instruction();

  const transaction = new anchor.web3.Transaction().add(instruction);

  const txSig = await anchor.web3.sendAndConfirmTransaction(
    anchor.getProvider().connection,
    transaction,
    two_auth_signer ? [source_owner, two_auth_signer] : [source_owner]
  );

  console.log(`Transfer (wrapped) of ${amount} tx : ${txSig}`);
}

export async function self_transfer_wtokens(
  amount: number,
  wrapper_account: anchor.web3.PublicKey,
  source_owner: anchor.web3.Signer,
  source_wrapped_account: anchor.web3.PublicKey,
  program: Program<HandmadeNaive>
) {
  const instruction = await program.methods
    .transfer(new anchor.BN(amount))
    .accountsPartial({
      sourceOwner: source_owner.publicKey,
      destinationOwner: source_owner.publicKey,
      sourceWrappedAccount: source_wrapped_account,
      destinationWrappedAccount: source_wrapped_account,
      twoAuthSigner: null,
      wrapperAccount: wrapper_account,
    })
    .signers([source_owner])
    .instruction();

  const transaction = new anchor.web3.Transaction().add(instruction);

  const txSig = await anchor.web3.sendAndConfirmTransaction(
    anchor.getProvider().connection,
    transaction,
    [source_owner]
  );

  console.log(`Self-Transfer (wrapped) of ${amount} tx : ${txSig}`);
}
