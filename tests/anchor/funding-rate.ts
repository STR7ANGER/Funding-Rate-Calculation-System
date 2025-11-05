import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { FundingRate } from "../target/types/funding_rate";
import { expect } from "chai";

describe("funding-rate", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.FundingRate as Program<FundingRate>;

  it("Initializes funding state", async () => {
    const symbol = "BTC-PERP";
    const [fundingStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("funding_rate"), Buffer.from(symbol)],
      program.programId
    );

    await program.methods
      .initializeFundingState(symbol)
      .accounts({
        fundingState: fundingStatePda,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    const state = await program.account.fundingRateState.fetch(fundingStatePda);
    expect(state.symbol).to.equal(symbol);
    expect(state.currentRate.toNumber()).to.equal(0);
  });

  it("Updates funding rate", async () => {
    const symbol = "BTC-PERP";
    const [fundingStatePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("funding_rate"), Buffer.from(symbol)],
      program.programId
    );

    const markPrice = new anchor.BN(50000 * 1e8);
    const indexPrice = new anchor.BN(49975 * 1e8);

    await program.methods
      .updateFundingRate(markPrice, indexPrice)
      .accounts({
        fundingState: fundingStatePda,
        authority: provider.wallet.publicKey,
      })
      .rpc();

    const state = await program.account.fundingRateState.fetch(fundingStatePda);
    expect(state.markPrice.toNumber()).to.equal(markPrice.toNumber());
    expect(state.indexPrice.toNumber()).to.equal(indexPrice.toNumber());
  });
});

