import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day23 } from "../target/types/day23";
import {expect} from 'chai';
describe("day23", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const conn = anchor.getProvider().connection;
  const provider = anchor.getProvider();
  const program = anchor.workspace.day23 as Program<Day23>;
  const bob = new anchor.web3.PublicKey("6PXVBoDbSFeQnazHu4NdqwGBxgsX97jYpp1fcVGQe7mA")
  it("Is initialized!", async () => {
    let balance  = await conn.getBalance(bob)
    // Add your test here.
    const tx = await program.methods.transferSol(new anchor.BN(3344)).accounts({
        from:provider.publicKey,
        to:bob,
        program:anchor.web3.SystemProgram.programId
    }).signers([provider.wallet.payer]).rpc();

    await conn.confirmTransaction(tx);
    let left = await conn.getBalance(bob) - balance;
    console.log("balance increase", left);
    expect(left).to.eq(3344);
    const txDetail = await conn.getTransaction(tx,{commitment:'confirmed',maxSupportedTransactionVersion:1});
    console.log("Your transaction signature", tx);
    console.log("tx log message",txDetail.meta.logMessages)

  });
});
