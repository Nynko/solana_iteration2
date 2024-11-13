import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HandmadeNaive } from "../target/types/handmade_naive";

export async function issue_first_idendity(
  validity_duration: number,
  owner: anchor.web3.Signer,
  issuer: anchor.web3.Signer,
  approver: anchor.web3.PublicKey,
  wrapper: anchor.web3.PublicKey,
  program: Program<HandmadeNaive>
) {
  const [idendity, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("identity"), owner.publicKey.toBuffer()],
    program.programId
  );

  console.log(`[Pk] Issue  Idendity : ${idendity}`);

  const tx = await program.methods
    .initializeId(new anchor.BN(validity_duration))
    .accountsPartial({
      approver: approver,
      wrapperAccount: wrapper,
      issuer: issuer.publicKey,
      owner: owner.publicKey,
      payer: anchor.Wallet.local().publicKey,
      idendity: idendity,
    })
    .signers([issuer, anchor.Wallet.local().payer, owner])
    .rpc();

  console.log(`Creating Idendity tx : ${tx}`);
}
