import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day14OnlyOwner } from "../target/types/day14_only_owner";

describe("day14-only-owner", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day14OnlyOwner as Program<Day14OnlyOwner>;
  let myKeypair = anchor.web3.Keypair.generate();
  it("signed with multiple signers!", async () => {
        // Add your test here.
        const tx = await program.methods.initialize().accounts({
            payer:anchor.getProvider().publicKey,
            signer:myKeypair.publicKey
        }).signers([myKeypair]).rpc();

        console.log("tx", tx);
        console.log("payer:", anchor.getProvider().publicKey.toBase58())
        console.log("signer", myKeypair.publicKey.toBase58());
  });


    it("signed only owner", async () => {
        // Add your test here.
        const tx = await program.methods.call().accounts({
            payer:anchor.getProvider().publicKey,
        }).rpc();

        console.log("tx", tx);
        console.log("payer:", anchor.getProvider().publicKey.toBase58())
    });


    it("signed only owner should fail", async () => {
        // Add your test here.
        const tx = await program.methods.call().accounts({
            payer:myKeypair.publicKey
        }).signers([myKeypair]).rpc();

        console.log("tx", tx);
        console.log("payer:", anchor.getProvider().publicKey.toBase58())
    });
});
