import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { StructuraProgram } from "../target/types/structura_program";
import { expect } from "chai";

describe("Structura On-Chain Tests", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  const program = anchor.workspace.StructuraProgram as Program<StructuraProgram>;

  let projectAccountPda: anchor.web3.PublicKey;
  let bump: number;

  it("Initialize a project", async () => {
    [projectAccountPda, bump] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("project"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    await program.methods
      .initializeProject("AI-Driven Construction", new anchor.BN(1000000))
      .accounts({
        projectAccount: projectAccountPda,
      })
      .rpc();

    const projectData = await program.account.projectAccount.fetch(projectAccountPda);
    expect(projectData.name).to.equal("AI-Driven Construction");
    expect(projectData.budget.toNumber()).to.equal(1000000);
  });

  it("Update a milestone", async () => {
    await program.methods
      .updateMilestone(1, true)
      .accounts({
        projectAccount: projectAccountPda,
      })
      .rpc();

    const projectData = await program.account.projectAccount.fetch(projectAccountPda);
    expect(projectData.milestonesCompleted).to.equal(1);
  });

  it("Distribute rewards (placeholder)", async () => {
    await program.methods
      .distributeRewards(new anchor.BN(1000))
      .accounts({
        projectAccount: projectAccountPda,
      })
      .rpc();

    // No real checks since it's just a placeholder
  });
});
