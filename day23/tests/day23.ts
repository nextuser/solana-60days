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

function getPublicKeyes(count : number): anchor.web3.PublicKey[]{
    let results = [];
    for(let  i = 0 ;i < count; ++ i) {
        const r1 = anchor.web3.Keypair.generate().publicKey;
        results.push(r1);
    }
    return results;
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

  function accountMetas(... keys :anchor.web3.PublicKey[]){
      let result = []
      for(let key of keys){
          let meta = {pubkey:key, isWritable:true,isSigner:false};
          result.push(meta);
      }
      return result;
  }

  async function printAccountBalance( prompt:string, ... accounts :anchor.web3.PublicKey[] )
  {
      console.log(prompt);
      for(let acc of accounts) {
          const balance = await conn.getBalance(acc) / anchor.web3.LAMPORTS_PER_SOL;
          const addr = acc.toBase58();
          console.log(`balance of ${addr} : ${balance} SOL`)
      }
  }

  it("split sol 3 users", async()=>{
      const r1 = anchor.web3.Keypair.generate().publicKey;
      const r2 = anchor.web3.Keypair.generate().publicKey;
      const r3 = anchor.web3.Keypair.generate().publicKey;
      await printAccountBalance("before Split Sol",r1,r2,r3);
      let miniRent = (await conn.getMinimumBalanceForRentExemption(0)) * 3;
      await program.methods.splitSol(new anchor.BN(miniRent)).remainingAccounts(accountMetas(r1,r2,r3)).rpc();

      await printAccountBalance("after splitSol",r1,r2,r3)

  })



    it("split sol n users", async()=>{
        // const r1 = anchor.web3.Keypair.generate().publicKey;
        // const r2 = anchor.web3.Keypair.generate().publicKey;
        // const r3 = anchor.web3.Keypair.generate().publicKey;
        // const r4 = anchor.web3.Keypair.generate().publicKey;
        let keys = getPublicKeyes(5);
        await printAccountBalance("before Split Sol",... keys);
        const metas = accountMetas(... keys);
        let miniRent = (await conn.getMinimumBalanceForRentExemption(0)) * metas.length;
        await program.methods.splitSol(new anchor.BN(miniRent)).remainingAccounts(metas).rpc();

        await printAccountBalance("after splitSol",...keys)

    })


    it("split sol 2 users", async()=>{
        const r1 = anchor.web3.Keypair.generate().publicKey;
        const r2 = anchor.web3.Keypair.generate().publicKey;
        await printAccountBalance("before Split Sol",r1,r2);
        let miniRent = (await conn.getMinimumBalanceForRentExemption(0)) * 3;
        await program.methods.splitSol(new anchor.BN(miniRent)).remainingAccounts(accountMetas(r1,r2)).rpc();

        await printAccountBalance("after splitSol",r1,r2)

    })
});
