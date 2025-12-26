import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CheckTransfer } from "../target/types/check_transfer";
import { Transaction, SystemProgram } from "@solana/web3.js"
import { confirmAndPrintTxDetails } from "./util";
describe("check_transfer", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.checkTransfer as Program<CheckTransfer>;
  const payer = program.provider.wallet.payer;
  const repient = anchor.web3.Keypair.generate();
  const conn = program.provider.connection;

  it("check transfer!", async () => {
    // Add your test here.
    const amount = new anchor.BN(1_000_000_000);
    const tx = new Transaction();
    tx.add(SystemProgram.transfer({
      fromPubkey: payer.publicKey,
      toPubkey: repient.publicKey,
      lamports: amount.toNumber(),
    }))
    tx.add(await program.methods.verifyTransfer(amount).accounts({
      instructionSysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,
    }).instruction());

    const sig = await anchor.getProvider().sendAndConfirm(tx, [payer]);
    await confirmAndPrintTxDetails(conn, sig);
    


    console.log("Your transaction signature", tx);
  });
});
