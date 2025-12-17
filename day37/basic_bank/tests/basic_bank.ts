import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BasicBank } from "../target/types/basic_bank";
import { Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { airdropSol,confirmAndPrintTxDetails, printAccount } from "./util";
import { expect } from "chai";
describe("basic_bank", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.basicBank as Program<BasicBank>;

  const bankAccount = Keypair.generate();
  const payer = anchor.getProvider().wallet.payer;
  const conn = anchor.getProvider().connection;
  console.log("payer:",payer.publicKey)
  console.log("bankAccount:",bankAccount.publicKey)
  
  const user = Keypair.generate();   

  const [userAccount,_bump]  = PublicKey.findProgramAddressSync(
      [Buffer.from("bank_account"), user.publicKey.toBuffer()],
      program.programId
    );

  it("Is initialized!", async () => {
    await airdropSol(conn,payer.publicKey,2 * LAMPORTS_PER_SOL);
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      bank: bankAccount.publicKey,
      payer : payer.publicKey,
    }).signers([bankAccount,payer]).rpc();
    await confirmAndPrintTxDetails(conn,tx);
    console.log("Your transaction signature", tx);
  });

  it("Create user account", async () => {
    console.log("userAccount:",userAccount)
    console.log("payer:",payer.publicKey);
    console.log("user:",user.publicKey);
    const tx = await program.methods.createUserAccount().accounts({
      user: user.publicKey,
      payer : payer.publicKey,
      userAccount: userAccount,
    }).signers([payer]).rpc();
    await confirmAndPrintTxDetails(conn,tx);
    console.log("Your transaction signature", tx);
  });

  it("Deposit", async () => { 
    await airdropSol(conn,user.publicKey,2 * LAMPORTS_PER_SOL);
    await printAccount(conn,user.publicKey,"before deposit ,user:");
    await printAccount(conn,bankAccount.publicKey,"before  deposit bankAccount");
    const tx = await program.methods.deposit(new anchor.BN(1* LAMPORTS_PER_SOL)).accounts({
      user: user.publicKey,
      userAccount: userAccount,
      bank: bankAccount.publicKey,
    }).signers([user]).rpc();
    await confirmAndPrintTxDetails(conn,tx);
    await printAccount(conn,user.publicKey,"after depoist user :");
    await printAccount(conn,bankAccount.publicKey,"after  deposit bankAccount");

  });

  it("Withdraw",async ()=>{
    await printAccount(conn,user.publicKey,"before withdraw user:");
    await printAccount(conn,bankAccount.publicKey,"before withdraw bankAccount:");
    const tx = await program.methods.withdraw(new anchor.BN(1* LAMPORTS_PER_SOL)).accounts({
      user: user.publicKey,
      userAccount: userAccount,
      bank: bankAccount.publicKey,
    }).signers([user]).rpc();
    await confirmAndPrintTxDetails(conn,tx);
    await printAccount(conn,user.publicKey,"after withdraw user:");
    await printAccount(conn,bankAccount.publicKey,"after withdraw bankAccount:");
  })

  it("Withdraw insufficient",async ()=>{
      try{
      await printAccount(conn,user.publicKey,"before withdraw user:");
      await printAccount(conn,bankAccount.publicKey,"before withdraw bankAccount:");
      const tx = await program.methods.withdraw(new anchor.BN(1)).accounts({
        user: user.publicKey,
        userAccount: userAccount,
        bank: bankAccount.publicKey,
      }).signers([user]).rpc();
      throw new Error("should not reach here")
    }catch(e){
      expect(e.message).to.include("Insufficient Balance");
      console.log(e)
    }
  })

    it("Withdraw invaliduser",async ()=>{
      try{
        const invalidUser = Keypair.generate();
      const tx = await program.methods.withdraw(new anchor.BN(1)).accounts({
        user: invalidUser.publicKey,
        userAccount: userAccount,
        bank: bankAccount.publicKey,
      }).signers([invalidUser]).rpc();
      throw new Error("should not reach here")
    }catch(e){
      expect(e.message).to.include("Invalid Authority");
      console.log(e)
    }
  })
});
