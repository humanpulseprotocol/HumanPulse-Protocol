import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { HumanpulseBurn } from "../target/types/humanpulse_burn";
import { createMint, createAccount, mintTo } from "@solana/spl-token";
import { PublicKey } from "@solana/web3.js";
import { expect } from "chai";

describe("humanpulse_burn", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.HumanpulseBurn as Program<HumanpulseBurn>;

  let mint: PublicKey;
  let payerTokenAccount: PublicKey;
  let contractTokenAccount: PublicKey;
  let validatorTokenAccount: PublicKey;
  const payer = (provider.wallet as anchor.Wallet).payer;
  const validator = anchor.web3.Keypair.generate();

  before(async () => {
    // Crea mint con 9 decimali
    mint = await createMint(
      provider.connection,
      payer,
      payer.publicKey,
      null,
      9
    );

    // Crea token account per pagatore, contratto e validatore
    payerTokenAccount = await createAccount(provider.connection, payer, mint, payer.publicKey);
    contractTokenAccount = await createAccount(provider.connection, payer, mint, (await PublicKey.findProgramAddress([Buffer.from("burner")], program.programId))[0], true);
    validatorTokenAccount = await createAccount(provider.connection, payer, mint, validator.publicKey);

    // Conia 1.000.000 HPP al pagatore (espresso in unità base: 1M * 10^9)
    await mintTo(provider.connection, payer, mint, payerTokenAccount, payer.publicKey, 1_000_000_000_000_000);
  });

  it("Processa una verifica: brucia il 50% e premia il validatore", async () => {
    const amount = new anchor.BN(100_000_000_000); // 100 HPP

    const tx = await program.methods
      .processVerificationFee(amount)
      .accounts({
        payer: payer.publicKey,
        payerTokenAccount,
        contractTokenAccount,
        validatorTokenAccount,
        validator: validator.publicKey,
        mint,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("Transazione:", tx);

    // Verifica che il contratto abbia bruciato 50 HPP
    const contractBalance = (await provider.connection.getTokenAccountBalance(contractTokenAccount)).value.uiAmount;
    expect(contractBalance).to.equal(0); // Il contratto non trattiene nulla

    // Verifica che il validatore abbia ricevuto 50 HPP
    const validatorBalance = (await provider.connection.getTokenAccountBalance(validatorTokenAccount)).value.uiAmount;
    expect(validatorBalance).to.equal(50);

    // Verifica evento
    // (in un test più completo si possono intercettare gli eventi)
  });
});