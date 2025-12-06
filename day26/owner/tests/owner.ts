import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Owner } from "../target/types/owner";
import {confirmTransaction,airdropSol,
    getTransactionDetials,printAccount,
    confirmAndPrintTxDetails} from "./util";
const {Keypair , PublicKey} = anchor.web3;
type Keypair = anchor.web3.Keypair;
type PublicKey = anchor.web3.PublicKey;

describe("owner", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.owner as Program<Owner>;
  const payer = anchor.getProvider().wallet.payer;
  const conn = anchor.getProvider().connection;

  it("initial keypair area!", async () => {
    const keyData = Keypair.generate();
    console.log("keyData:",keyData.publicKey.toBase58())
    const tx = await program.methods.initializeKeypair()
        .accounts({
            data:keyData.publicKey,
            signer: payer.publicKey,
            systemProgram:anchor.web3.SystemProgram.programId,
        }).signers([payer,keyData])
        .rpc();

    //await confirmTransaction(conn,tx);
    await confirmAndPrintTxDetails(conn,tx);
    await printAccount(conn,keyData.publicKey, "after init keypair area");

    console.log("Your transaction signature", tx);
  });

    it("initial pda!", async () => {
        let [pda,_bump] =  anchor.web3.PublicKey.findProgramAddressSync(
            [],
            program.programId
        )
        // Add your test here.
        const tx = await program.methods.initializePda()
            .accounts({
                data:pda,
                signer:payer.publicKey,
                systemProgram: anchor.web3.SystemProgram.programId,
            })
            .signers([payer])
            .rpc();
        await confirmAndPrintTxDetails(conn,tx);
        await printAccount(conn,pda, "after init pda");
        console.log("Your transaction signature", tx);
    });
});
