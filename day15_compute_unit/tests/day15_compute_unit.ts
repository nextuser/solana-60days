import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { ComputeUnit } from "../target/types/compute_unit";


describe("day15_compute_unit", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.compute_unit as Program<ComputeUnit>;

  it("Is initialized!", async () => {
    // Add your test here.
    const web3 = anchor.web3;
    const payer = program.provider.wallet.publicKey;
    let balance = await anchor.getProvider().connection.getBalance(payer)
    console.log("balance of payer:", balance);
    const tx = await program.methods.initialize().rpc();
     let balance2 = await anchor.getProvider().connection.getBalance(payer)
     console.log("cost fee:",balance2-balance);
    console.log("balance of payer:", balance);
    console.log("Your transaction signature", tx);
  });

  

  it("call_signed!", async () => {
    // Add your test here.
    const web3 = anchor.web3;
    const payer = program.provider.wallet.publicKey;
    let balance = await anchor.getProvider().connection.getBalance(payer)
    console.log("balance of payer:", balance);
    const other = web3.Keypair.generate();
    const tx = await program.methods.callSigned()
                        .accounts({signer:payer,secondSigner:other.publicKey})
                        .signers([other])
                        .rpc();
    // 5000*2 compute unit, 2 signers
     let balance2 = await anchor.getProvider().connection.getBalance(payer)
     console.log("cost fee:",balance2-balance);
     console.log("balance of payer:", balance);
     console.log("Your transaction signature", tx);
     const connection = anchor.getProvider().connection;
     await connection.confirmTransaction(tx);
     const transaction = await connection.getParsedTransaction(tx,{commitment:'confirmed'});
     if(!transaction.meta){
      console.log("meta is null",transaction);
     } else{
      console.log("transaction computeunit:", transaction.meta.computeUnitsConsumed,
                "cost unit", transaction.meta.costUnits,
                "fee",transaction.meta.fee);
     }
    });
});
