import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BasicBank } from "../target/types/basic_bank";
import { Keypair, PublicKey } from "@solana/web3.js";
import { airdropSol,confirmAndPrintTxDetails } from "./util";
describe("basic_bank", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.basicBank as Program<BasicBank>;

  const bankAccount = Keypair.generate();
  const payer = anchor.getProvider().wallet.payer;
  const conn = anchor.getProvider().connection;
  console.log("payer:",payer.publicKey)
  console.log("bankAccount:",bankAccount.publicKey)
  


  it("Is initialized!", async () => {
    await airdropSol(conn,payer.publicKey,1);
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      bank: bankAccount.publicKey,
      payer : payer.publicKey,
    }).signers([bankAccount,payer]).rpc();
    await confirmAndPrintTxDetails(conn,tx);
    console.log("Your transaction signature", tx);
  });

  it("Create user account", async () => {
    const user = Keypair.generate();    

    const [userAccount,_bump]  = PublicKey.findProgramAddressSync(
      [Buffer.from("bank_account"), user.publicKey.toBuffer()],
      program.programId
    );

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
});
