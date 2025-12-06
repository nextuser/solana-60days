import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";

import { KeypairVsPda } from "../target/types/keypair_vs_pda";
import {airdropSol,getTransactionDetials,confirmTransaction} from "./util";

describe("keypair_vs_pda", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const conn = anchor.getProvider().connection;

  const program = anchor.workspace.keypairVsPda as Program<KeypairVsPda>;
  const dataKey = anchor.web3.Keypair.generate();
  console.log("alice:", dataKey.publicKey.toBase58());

  it("Is initialized!", async () => {
    let accountInfo :anchor.web3.AccountInfo = await conn.getAccountInfo(dataKey.publicKey)
      if(accountInfo != null) {
          console.log("before init :data key info :", accountInfo.lamports, accountInfo.data.length)
      } else{
          console.log("accountInfo null");
      }
    // Add your test here.
    const tx = await program.methods.initialize(new anchor.BN(88)).accounts({
        myKeypairAccount : dataKey.publicKey,
        signer : anchor.getProvider().publicKey,
        systemProgram : anchor.web3.SystemProgram.programId,
    }).signers([dataKey,anchor.getProvider().wallet.payer]).rpc();
    await confirmTransaction(conn,tx);
    const detail = await getTransactionDetials(conn,tx);
    accountInfo = await conn.getAccountInfo(dataKey.publicKey)
    console.log("after init data key info :",
        "lamports",accountInfo.lamports,
        "space",accountInfo.space,
        "owner:",accountInfo.owner)
    console.log('tx detail :',detail);
    
    console.log("Your transaction signature", tx);
  });
});
