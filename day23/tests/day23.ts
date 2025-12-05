import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day23 } from "../target/types/day23";
import {expect} from 'chai';
async function requestAirdrop(conn: anchor.web3.Connection, pubKey:anchor.web3.PublicKey)
{
     const lastBockHash = await conn.getLatestBlockhash();
     let sig = await conn.requestAirdrop(pubKey , 20000);
      await conn.confirmTransaction({signature:sig,blockhash:lastBockHash },{commitment:'confirmed'});
}
function getFromKeypair(){
    return anchor.getProvider().wallet.payer
}
describe("day23", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const conn = anchor.getProvider().connection;
  const provider = anchor.getProvider();
  const program = anchor.workspace.day23 as Program<Day23>;
  const bob = anchor.web3.Keypair.generate().publicKey//new anchor.web3.PublicKey("6PXVBoDbSFeQnazHu4NdqwGBxgsX97jYpp1fcVGQe7mA")
  const charlie = getFromKeypair();
    // const charlie = anchor.web3.Keypair.generate();
  it("transfer sol!", async () => {

    let balance  = await conn.getBalance(bob)
    console.log("balance of bob:", balance);
    //注意如果dataLength没有赋值 ，这个会返回0，dataLength 0 bytes => 890_880   , dataLength undefined =>0
    const mini_amount = await conn.getMinimumBalanceForRentExemption(0);
    console.log("minit amount ",mini_amount);
    const amount = mini_amount ;
    // Add your test here.
    const tx = await program.methods.transferSol(new anchor.BN(amount)).accounts({
        from:charlie.publicKey,
        to:bob,
        program:anchor.web3.SystemProgram.programId
    }).signers([provider.wallet.payer]).rpc();

    await conn.confirmTransaction(tx);
    let left = await conn.getBalance(bob) - balance;
    console.log("balance increase", left);
    expect(left).to.eq(amount);
    //version 0 可能会查不到，
    const txDetail = await conn.getTransaction(tx,{commitment:'confirmed',maxSupportedTransactionVersion:1});
    console.log("Your transaction signature", tx);
    if(txDetail && txDetail.meta) {
        console.log("tx log message", txDetail.meta.logMessages)
    }

  });
});
