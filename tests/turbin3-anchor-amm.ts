import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Turbin3AnchorAmm } from "../target/types/turbin3_anchor_amm";

describe("turbin3-anchor-amm", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.turbin3AnchorAmm as Program<Turbin3AnchorAmm>;

  it("Initialize config", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
