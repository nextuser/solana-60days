import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorEscrow } from "../target/types/anchor_escrow";
import { ConfirmOptions, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js";
import { Mint,getOrCreateAssociatedTokenAccount, getAssociatedTokenAddress ,
  createMint, ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, 
  createAssociatedTokenAccount, mintTo,
  getAccount,
} from '@solana/spl-token';
import {dot,airdropSol,confirmAndPrintTxDetails } from 'anchor-utils';
import { expect } from "chai";

// async function confirmTransaction(connection: anchor.web3.Connection, signature: string){
//     const Block = await connection.getLatestBlockhash();
//     await  connection.confirmTransaction(
//       {signature,
//         blockhash: Block.blockhash,
//         lastValidBlockHeight: Block.lastValidBlockHeight
//       },  
//       "confirmed"
      
//     );
// }
// async function airdrop(connection: anchor.web3.Connection, pubkey: anchor.web3.PublicKey){
//     let signature = await connection.requestAirdrop(pubkey, 10_000_000_000);
//     await confirmTransaction(connection, signature);
    
// }



async function getTokenAmount(connection: anchor.web3.Connection, ata: anchor.web3.PublicKey){

  return (await getAccount(connection, ata, "confirmed")).amount;
}

type EscrowInfo = {
  seed : bigint,
  escrow : anchor.web3.PublicKey,
  vault : anchor.web3.PublicKey,
  bump : number,
}

const program = anchor.workspace.anchorEscrow as Program<AnchorEscrow>;
console.log("anchor program id :",program.programId.toBase58());
async function getEscrowInfo(
    connection: anchor.web3.Connection, 
    mint_key: anchor.web3.PublicKey,
    maker_key : anchor.web3.PublicKey, 
    seed: bigint): Promise<EscrowInfo>{
    const [escrow, bump] =  anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        maker_key.toBuffer(),
        new anchor.BN(seed).toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );
    const vault = await getAssociatedTokenAddress(mint_key, escrow, true, TOKEN_PROGRAM_ID);
  
  return {
    seed,
    escrow,
    vault,
    bump

  }
}

describe("anchor_escrow", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const payer = anchor.getProvider().wallet.payer;
  const maker = anchor.web3.Keypair.generate();
  const taker = anchor.web3.Keypair.generate();
  const connection = anchor.getProvider().connection;
  let mintA = Keypair.generate();
  let mintB = Keypair.generate();
  let makerAtaA : anchor.web3.PublicKey;
  let takerAtaB : anchor.web3.PublicKey;
  let takerAtaA : anchor.web3.PublicKey;
  let makerAtaB : anchor.web3.PublicKey;
  
  const amount = 1000_000n;
  const receive = 2000_000n ;
  
  // const [escrow, bump] =  anchor.web3.PublicKey.findProgramAddressSync(
  //     [
  //       Buffer.from("escrow"),
  //       maker.publicKey.toBuffer(),
  //       new anchor.BN(seed).toArrayLike(Buffer, "le", 8),
  //     ],
  //     program.programId
  //   );
  const confirm_option : ConfirmOptions = {commitment:"confirmed"}
  before(async () => {
    
    console.log("wait to create mint ")
     await createMint(connection, payer, payer.publicKey, payer.publicKey, 6,mintA,confirm_option,TOKEN_PROGRAM_ID);
     await createMint(connection, payer, payer.publicKey, payer.publicKey, 6,mintB,confirm_option,TOKEN_PROGRAM_ID);
    console.log("mint created");
    await airdropSol( maker.publicKey,1);
    await airdropSol( taker.publicKey,1);
    await airdropSol( payer.publicKey,1);

    console.log("air drop ok");


    let init_balance = await connection.getBalance(maker.publicKey);
    makerAtaA = await createAssociatedTokenAccount(connection,payer,mintA.publicKey, maker.publicKey,confirm_option);
    makerAtaB = await createAssociatedTokenAccount(connection,payer,mintB.publicKey, maker.publicKey,confirm_option);
    
    takerAtaA = await createAssociatedTokenAccount(connection,payer,mintA.publicKey, taker.publicKey,confirm_option);
    takerAtaB = await createAssociatedTokenAccount(connection,payer,mintB.publicKey, taker.publicKey,confirm_option);
    console.log("ata created for taker")
    console.log("ata created for maker")



    // let vault = await createAssociatedTokenAccount(connection,payer,mintA.publicKey,escrow,confirm_option);
    // console.log("ata created for vault");
  });
  it("test make an take", async () => {
    const seed = 1n;
    const escrowInfo : EscrowInfo= await getEscrowInfo(connection, mintA.publicKey, maker.publicKey, seed);
    await mintTo(connection,payer,mintA.publicKey,makerAtaA,payer.publicKey,amount,[payer],confirm_option);
    console.log("mint tokenA to maker");
    await mintTo(connection,payer,mintB.publicKey,takerAtaB,payer.publicKey,receive,[payer],confirm_option)
    console.log("mint tokenB to taker");
    const signature = await program.methods.make(
      new anchor.BN(seed), 
      new anchor.BN(amount),
      new anchor.BN(receive) 
    ).accounts({
      maker: maker.publicKey,
      escrow: escrowInfo.escrow,
      mintA: mintA.publicKey,
      mintB: mintB.publicKey,
      makerAtaA: makerAtaA,
      vault: escrowInfo.vault,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
    }) .signers([maker]).rpc();
    dot();
    expect( (await getAccount(connection,makerAtaA)).amount).to.equal(0n);
    dot();
    expect( (await getAccount(connection,escrowInfo.vault)).amount).to.equal(amount);
    dot();
    expect( (await getAccount(connection,takerAtaB)).amount).to.equal(receive);

    await confirmAndPrintTxDetails( signature,"make1");

    const signature2 = await program.methods.take().accounts({
      taker: taker.publicKey,
      maker: maker.publicKey,
      escrow: escrowInfo.escrow,
      mintA: mintA.publicKey,
      mintB: mintB.publicKey,
      takerAtaB: takerAtaB,
      takerAtaA: takerAtaA,
      makerAtaB: makerAtaB,
      vault: escrowInfo.vault,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID, 
    }) .signers([taker]).rpc();
    await confirmAndPrintTxDetails( signature2,"take1");

    dot();
    expect( (await getAccount(connection,makerAtaB)).amount).to.equal(receive);
    dot();
    //vault distroyed
    //expect( await getTokenAmount(connection,vault)).to.equal(0n);
    //console.log(2);
    expect(await getTokenAmount(connection,takerAtaB)).to.equal(0n);
    dot();
    expect( await getTokenAmount(connection,takerAtaA)).to.equal(amount);
    dot();
    expect( await getTokenAmount(connection,makerAtaA)).to.equal(0n);    
  });//end it


  it("test make an refund!", async () => {
    await mintTo(connection,payer,mintA.publicKey,makerAtaA,payer.publicKey,amount,[payer],confirm_option);
    console.log("mint tokenA to maker");

    const seed = 2n;
    const escrowInfo : EscrowInfo= await getEscrowInfo(connection, mintA.publicKey, maker.publicKey, seed);
    expect( (await getAccount(connection,makerAtaB)).amount).to.equal(receive);
    const signature = await program.methods.make(new anchor.BN(seed),  new anchor.BN(amount),new anchor.BN(receive))
      .accounts({
      maker: maker.publicKey,
      escrow: escrowInfo.escrow,
      mintA: mintA.publicKey,
      mintB: mintB.publicKey,
      makerAtaA: makerAtaA,
      vault: escrowInfo.vault,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
    }) .signers([maker]).rpc();

    await confirmAndPrintTxDetails( signature, "make2");

    dot();
    expect( (await getAccount(connection,makerAtaA)).amount).to.equal(0n);
    dot();
    expect( (await getAccount(connection,escrowInfo.vault)).amount).to.equal(amount);
    dot();


    const signature2 = await program.methods.refund().accounts({
      maker: maker.publicKey,
      escrow: escrowInfo.escrow,
      mintA: mintA.publicKey,
      vault: escrowInfo.vault,
      makerAtaA: makerAtaA,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID, 
    }) .signers([maker]).rpc();
    await confirmAndPrintTxDetails( signature2,"refund2");

    dot();
    //vault distroyed
    //expect( await getTokenAmount(connection,vault)).to.equal(0n);
    //console.log(2);


    expect( await getTokenAmount(connection,makerAtaA)).to.equal(amount);    
  });//end it
});
