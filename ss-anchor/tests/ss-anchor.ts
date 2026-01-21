import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SsAnchor } from "../target/types/ss_anchor";
import { PublicKey, Transaction } from "@solana/web3.js";
import { confirmAndPrintTxDetails,airdropSol } from "./util";
import { describe,it } from "node:test";
import { expect } from "chai";

describe("ss-anchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.ssAnchor as Program<SsAnchor>;
  const caller = anchor.web3.Keypair.generate();
  const payer = anchor.getProvider().wallet.payer as anchor.web3.Keypair;
  const conn = anchor.getProvider().connection;
  const OVERHEAD_SIZE = 8 + 1 + 32 + 4;//descrimator + bump + authlength + vec header
  const init_balance = anchor.web3.LAMPORTS_PER_SOL * 1;

  const pda   :  PublicKey = anchor.web3.PublicKey.findProgramAddressSync([
      Buffer.from("storage"),
      caller.publicKey.toBuffer(),
    ], program.programId)[0];
  it("Is initialized!", async () => {
    
    await airdropSol(conn,caller.publicKey, init_balance);
    await airdropSol(conn,payer.publicKey, 10_000_000_000);



    const new_data = Buffer.from("hello world");

    // Add your test here.
    const signature  = await program.methods.initialize(new_data).accounts({
      user: caller.publicKey,
      userPda: pda,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([payer,caller]).rpc();


    await confirmAndPrintTxDetails(conn,signature);

    let balance_caller = await conn.getBalance(caller.publicKey);
    const rent_fee = await conn.getMinimumBalanceForRentExemption(new_data.length + OVERHEAD_SIZE);

    console.log();

    expect(balance_caller).to.equal(init_balance - rent_fee);
  });

  it("Update", async () => { 

    const new_data = Buffer.from("hadfasdfasdfsa");
    const signature  = await program.methods.update(new_data).accounts({
      user: caller.publicKey,
      userPda: pda,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([payer,caller]).rpc();
    let fee = await conn.getMinimumBalanceForRentExemption(new_data.length + OVERHEAD_SIZE);
    expect(await conn.getBalance(pda)).to.equal(fee);
    expect(await conn.getBalance(caller.publicKey)).to.equal(init_balance - fee);
  });

});
