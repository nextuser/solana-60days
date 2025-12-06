import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import { KeypairVsPda } from "../target/types/keypair_vs_pda";
import {
    airdropSol,
    getTransactionDetials,
    confirmTransaction,
    printAccount,
    transfer_and_print
} from "./util";

describe("keypair_vs_pda", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const conn = anchor.getProvider().connection;

  const program = anchor.workspace.keypairVsPda as Program<KeypairVsPda>;


  it("keypair address init ", async () => {
      const dataKey = anchor.web3.Keypair.generate();
    await printAccount(conn,dataKey.publicKey,"before init data key info :");

    // Add your test here.
    const tx = await program.methods.initialize(new anchor.BN(88)).accounts({
        myKeypairAccount : dataKey.publicKey,
        signer : anchor.getProvider().publicKey,
        systemProgram : anchor.web3.SystemProgram.programId,
    }).signers([dataKey,anchor.getProvider().wallet.payer]).rpc();
    await confirmTransaction(conn,tx);
    const detail = await getTransactionDetials(conn,tx);
    await printAccount(conn,dataKey.publicKey,"after init data key info :");

    console.log('tx detail :',detail);
  });


    it("keypair address init with sol", async () => {
        const dataKey = anchor.web3.Keypair.generate();
        await airdropSol(conn,dataKey.publicKey,1e9);
        await printAccount(conn,dataKey.publicKey,"before init data key info :");
        // Add your test here.
        const tx = await program.methods.initialize(new anchor.BN(88)).accounts({
            myKeypairAccount : dataKey.publicKey,
            signer : anchor.getProvider().publicKey,
            systemProgram : anchor.web3.SystemProgram.programId,
        }).signers([dataKey,anchor.getProvider().wallet.payer]).rpc();
        await confirmTransaction(conn,tx);
        const detail = await getTransactionDetials(conn,tx);
        await printAccount(conn,dataKey.publicKey,"after init data key info :");
        console.log('tx detail :',detail);
    });

    it("keypair address : airdrop-transfer-init-transfer", async () => {
        const dataKey = anchor.web3.Keypair.generate();
        const receiver = anchor.web3.Keypair.generate();
        const other = anchor.web3.Keypair.generate();
        await airdropSol(conn,dataKey.publicKey,1e9);
        await airdropSol(conn,other.publicKey,1e9)
        await printAccount(conn,dataKey.publicKey,"after airdrop :");

        await transfer_and_print(conn,dataKey,receiver.publicKey,1e8);

        const tx = await program.methods.initialize(new anchor.BN(88)).accounts({
            myKeypairAccount : dataKey.publicKey,
            signer : anchor.getProvider().publicKey,
            systemProgram : anchor.web3.SystemProgram.programId,
        }).signers([dataKey,anchor.getProvider().wallet.payer]).rpc();
        await confirmTransaction(conn,tx);
        const detail = await getTransactionDetials(conn,tx);
        await printAccount(conn,dataKey.publicKey,"after init data key info :");
        console.log('tx detail :',detail);

        await transfer_and_print(conn,other,dataKey.publicKey,1e8);
        //后续转账会失败，因为dataKey的owner 已经不是systemProgram，
        /**

        var trans2 = new anchor.web3.Transaction().add(
            anchor.web3.SystemProgram.transfer({
                fromPubkey:dataKey.publicKey,
                toPubkey: receiver.publicKey,
                lamports: 1e8,
            })
        )
        await anchor.web3.sendAndConfirmTransaction(conn,trans2,[dataKey]);


        await printAccount(conn,dataKey.publicKey,"after transfer2 :");
        await printAccount(conn,receiver.publicKey,"after transfer2 :");*/
    });


    it("after transfer ,owner changed", async () => {
        const dataKey = anchor.web3.Keypair.generate();
        const receiver = anchor.web3.Keypair.generate();
        await airdropSol(conn, dataKey.publicKey, 1e9);
        await transfer_and_print(conn, dataKey, receiver.publicKey, 1e8);
    });



    // it("keypair address init by other will crash", async () => {
    //     const dataKey = anchor.web3.Keypair.generate();
    //     const other = anchor.web3.Keypair.generate();
    //     //await airdropSol(conn,dataKey.publicKey,1e9);
    //     await airdropSol(conn,other.publicKey,1e9);
    //     await printAccount(conn,dataKey.publicKey,"before init data key info :");
    //     // Add your test here.
    //     const tx = await program.methods.initialize(new anchor.BN(88)).accounts({
    //         myKeypairAccount : dataKey.publicKey,
    //         signer : anchor.getProvider().publicKey,
    //         systemProgram : anchor.web3.SystemProgram.programId,
    //     }).signers([other,anchor.getProvider().wallet.payer]).rpc();
    //     await confirmTransaction(conn,tx);
    //     const detail = await getTransactionDetials(conn,tx);
    //     await printAccount(conn,dataKey.publicKey,"after init data key info :");
    //     console.log('tx detail :',detail);
    // });
});
