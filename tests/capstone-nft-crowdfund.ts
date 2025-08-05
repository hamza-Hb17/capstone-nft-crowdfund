import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CapstoneNftCrowdfund } from "../target/types/capstone_nft_crowdfund";

describe("capstone-nft-crowdfund", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.capstoneNftCrowdfund as Program<CapstoneNftCrowdfund>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
