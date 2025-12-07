import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Donate } from "../target/types/donate";
import {generateKeypairWithSol, printAccount, confirmAndPrintTxDetails, confirmTransaction} from "./util";

describe("donate", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.donate as Program<Donate>;
  const conn = anchor.getProvider().connection;
  it("donate!", async () => {
    const alice = await generateKeypairWithSol(conn,10);
    const withdrawer = await generateKeypairWithSol(conn,10);
    const bob = await generateKeypairWithSol(conn,10);
    const seeds = [withdrawer.publicKey.toBytes()];
    let [pda ,_bump]  = anchor.web3.PublicKey.findProgramAddressSync(
        seeds,program.programId
    )

      await  printAccount(conn,alice.publicKey,"before donate, alice balance:")
    // Add your test here.
    const tx = await program.methods.donate(new anchor.BN(60000000)).accounts({
        pda:pda,
        withdrawer: withdrawer.publicKey,
        signer: alice.publicKey,
    }).signers([alice]).rpc();
    await confirmAndPrintTxDetails(conn,tx);
    await  printAccount(conn,alice.publicKey,"after donate, alice balance:")
    await  printAccount(conn,pda,"after donate, pda info:")

    const tx_d = await program.methods.donate(new anchor.BN(50000000)).accounts({
        signer: bob.publicKey,
        withdrawer: withdrawer.publicKey,
        pda:pda,
    }).signers([bob]).rpc();
    await confirmAndPrintTxDetails(conn,tx_d);
    await  printAccount(conn,bob.publicKey,"after donate2, bob balance:")
    await  printAccount(conn,pda,"after donate2, pda info:")

    await  printAccount(conn,withdrawer.publicKey,"before withdraw, withdrawer balance:")
    let tx_wh = await  program.methods.withdraw(new anchor.BN(5000000)).accounts(
        {
            pda : pda,
            withdrawer: withdrawer.publicKey,
        }
    ).signers([withdrawer]).rpc();
    await  confirmAndPrintTxDetails(conn,tx_wh);
    await  printAccount(conn,withdrawer.publicKey,"after withdraw withdrawer balance:")
  });
});
