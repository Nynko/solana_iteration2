import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HandmadeNaive } from "../target/types/handmade_naive";

export async function add_an_issuer(
  issuer: anchor.web3.Signer,
  program: Program<HandmadeNaive>
): Promise<anchor.web3.PublicKey> {
  const [approval, bump] = await anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("issuer_approval"), issuer.publicKey.toBuffer()],
    program.programId
  );

  console.log(`[Pk] Approval issuer : ${approval}`);

  const tx = await program.methods
    .approveIssuer()
    .accountsPartial({
      issuer: issuer.publicKey,
      approver: anchor.Wallet.local().publicKey,
      payer: anchor.Wallet.local().publicKey,
      approval: approval,
    })
    .signers([issuer, anchor.Wallet.local().payer])
    .rpc();

  console.log(`Approve issuer tx : ${tx}`);

  return approval;
}

export async function issue_first_idendity(
  validity_duration: number,
  owner: anchor.web3.Signer,
  issuer: anchor.web3.Signer,
  approval: anchor.web3.PublicKey,
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
      issuer: issuer.publicKey,
      approval: approval,
      owner: owner.publicKey,
      payer: anchor.Wallet.local().publicKey,
    })
    .signers([issuer, anchor.Wallet.local().payer, owner])
    .rpc();

  console.log(`Creating Idendity tx : ${tx}`);
}
