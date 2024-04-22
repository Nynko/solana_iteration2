import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HandmadeNaive } from "../target/types/handmade_naive";
import {
  create_spl_mint,
  create_spl_token_account,
  initialize_program,
  initialize_wrapped_account,
  initialize_wrapped_token_holder,
  mint_tokens,
} from "./Initialize_tests";
import { TOKEN_PROGRAM_ID, transfer } from "@solana/spl-token";
import { wrap_tokens } from "./wrapped_tokens_tests";
import { min } from "bn.js";
import { expect } from "chai";
import { create_user_with_best_bump, sendTransaction, sleep } from "./utils";
import { transfer_wtokens } from "./transfer_tests";
import { add_an_issuer, issue_first_idendity } from "./idendity_tests";

describe("handmade_naive", async () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.HandmadeNaive as Program<HandmadeNaive>;
  let mint_info, user1_info, user2_info, wrapper;
  let issuer_approval, user1_id;
  const issuer = anchor.web3.Keypair.generate();

  it("Init", async () => {
    const init_return = await init(program);
    mint_info = init_return.mint_info;
    user1_info = init_return.user1_info;
    user2_info = init_return.user2_info;
    wrapper = init_return.wrapper;

    expect(mint_info.mint).to.not.be.null;
    expect(mint_info.mintAuthority).to.not.be.null;
    expect(mint_info.mintFreezeAuthority).to.not.be.null;
    expect(mint_info.token_program).to.not.be.null;
  });

  it("Create IDs", async () => {
    try {
      issuer_approval = await add_an_issuer(issuer, program);
      await issue_first_idendity(
        1000,
        user1_info.user1,
        issuer,
        issuer_approval,
        program
      );
      await issue_first_idendity(
        1000,
        user2_info.user2,
        issuer,
        issuer_approval,
        program
      );
    } catch (error) {
      console.log("Error", error);
      expect(error).to.be.null;
    }
  });

  it("Wrap Tokens", async () => {
    try {
      await mint_tokens(
        10,
        anchor.Wallet.local().payer,
        mint_info.mint,
        user1_info.token_account,
        mint_info.mintAuthority,
        mint_info.token_program
      );

      console.log("wrapper", wrapper.wrapper_token_holder.toBase58());

      await wrap_tokens(
        5,
        mint_info.decimals,
        user1_info.user1,
        user1_info.token_account,
        mint_info.mint,
        wrapper.wrapper_token_holder,
        program
      );

      const wrapped_account = await program.account.wrappedTokenAccount.fetch(
        user1_info.wrapped_account
      );
      console.log("Wrapped Account", user1_info.wrapped_account.toBase58());
      expect(wrapped_account.amount.toNumber()).to.equal(5);

      const token_account_balance =
        await program.provider.connection.getTokenAccountBalance(
          user1_info.token_account
        );
      expect(Number(token_account_balance.value.amount)).to.equal(5);
    } catch (error) {
      console.log("Error", error);
      expect(error).to.be.null;
    }
  });

  it("Transfer Tokens", async () => {
    await sendTransaction(
      anchor.Wallet.local().payer,
      user1_info.user1.publicKey,
      1000000
    );
    try {
      await transfer_wtokens(
        2,
        mint_info.decimals,
        user1_info.user1,
        user1_info.wrapped_account,
        user2_info.user2.publicKey,
        user2_info.wrapped_account,
        mint_info.mint,
        program
      );
    } catch (error) {
      console.log("Error", error);
      expect(error).to.be.null;
    }

    const user1_balance = await program.account.wrappedTokenAccount
      .fetch(user1_info.wrapped_account)
      .then((account) => account.amount.toNumber());
    const user2_balance = await program.account.wrappedTokenAccount
      .fetch(user2_info.wrapped_account)
      .then((account) => account.amount.toNumber());
    console.log("User1 Balance", user1_balance);
    console.log("User2 Balance", user2_balance);

    expect(user1_balance).to.equal(3);
    expect(user2_balance).to.equal(2);
  });
});

interface InitReturn {
  mint_info: {
    mint: anchor.web3.PublicKey;
    mintAuthority: anchor.web3.Keypair;
    mintFreezeAuthority: anchor.web3.Keypair;
    token_program: anchor.web3.PublicKey;
    decimals: number;
  };
  user1_info: {
    user1: anchor.web3.Keypair;
    token_account: anchor.web3.PublicKey;
    wrapped_account: anchor.web3.PublicKey;
  };
  user2_info: {
    user2: anchor.web3.Keypair;
    wrapped_account: anchor.web3.PublicKey;
  };
  wrapper: {
    wrapper_pda: anchor.web3.PublicKey;
    wrapper_token_holder: anchor.web3.PublicKey;
  };
}

async function init(program: Program<HandmadeNaive>): Promise<InitReturn> {
  const token_program = TOKEN_PROGRAM_ID;
  const mintAuthority = anchor.web3.Keypair.generate();
  console.log("[Pk] mintAuthority", mintAuthority.publicKey.toBase58());
  const mintFreezeAuthority = anchor.web3.Keypair.generate();
  console.log(
    "[Pk] mintFreezeAuthority",
    mintFreezeAuthority.publicKey.toBase58()
  );

  const decimals = 2;
  const mint = await create_spl_mint(
    anchor.Wallet.local().payer,
    mintAuthority,
    mintFreezeAuthority,
    decimals,
    token_program
  );

  const user1 = await create_user_with_best_bump(program, mint);
  console.log("[Pk] user1", user1.publicKey.toBase58());

  const user2 = await create_user_with_best_bump(program, mint);
  console.log("[Pk] user2", user2.publicKey.toBase58());

  const wrapper_pda = await initialize_program(
    anchor.Wallet.local().payer,
    program
  );

  const wrapper_token_holder = await initialize_wrapped_token_holder(
    anchor.Wallet.local().payer,
    mint,
    wrapper_pda,
    program,
    token_program
  );

  const token_account = await create_spl_token_account(
    anchor.Wallet.local().payer,
    user1.publicKey,
    mint,
    token_program
  );

  const wrapped_account = await initialize_wrapped_account(
    user1,
    mint,
    program,
    token_program
  );

  const wrapped_account2 = await initialize_wrapped_account(
    user2,
    mint,
    program,
    token_program
  );

  return {
    mint_info: {
      mint,
      mintAuthority,
      mintFreezeAuthority,
      token_program,
      decimals,
    },
    user1_info: {
      user1,
      token_account,
      wrapped_account,
    },
    user2_info: {
      user2,
      wrapped_account: wrapped_account2,
    },
    wrapper: {
      wrapper_pda,
      wrapper_token_holder,
    },
  };
}
