import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Keypair, LAMPORTS_PER_SOL, PublicKey} from "@solana/web3.js";
import * as splToken from "@solana/spl-token";
import {SplToken } from "../target/types/spl_token";
import { airdropSol, confirmAndPrintTxDetails, generateKeypairWithSol } from "./util";
type Connection = anchor.web3.Connection;
function generateTokenName(){
  const name = Keypair.generate().publicKey.toBase58().substring(0,8);
  return "TOKEN_" + name;
}

describe("spl_token", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const conn = anchor.getProvider().connection;

  const program = anchor.workspace.splToken as Program<SplToken>;

  const provider = anchor.getProvider();
  const signerKp = provider.wallet.payer;
  const toKp = anchor.web3.Keypair.generate();
  //考虑到每次调用例子的时候，如果token已经存在，会报错，所以这里每次调用都生成一个新的tokenName
  const tokenName = generateTokenName();
  const mintToKeypair = Keypair.generate();

  const [mint ,_bump] = PublicKey.findProgramAddressSync(
    [signerKp.publicKey.toBuffer(),Buffer.from(tokenName)],
    program.programId
  );
  const fromAta = splToken.getAssociatedTokenAddressSync(
    mint,
    mintToKeypair.publicKey,
    false);

  const toAta = splToken.getAssociatedTokenAddressSync(
    mint,
    toKp.publicKey,
    false);
  

  it("Is initialized!", async () => {
    
    
    // Add your test here.
    const tx = await program.methods.createAndMintToken(tokenName)
      .accounts({
        signer: signerKp.publicKey,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
        associatedTokenProgram: splToken.ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        newMint: mint,
        newAta: fromAta,
        ataAuthority: mintToKeypair.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
    console.log("Mint: ", mint.toBase58());
    console.log("ATA: ", fromAta.toBase58());
    console.log("Mint Info: ", await getMintInfo(provider.connection, mint));
  });


  it("transfer token",async()=>{
    try{
      await airdropSol(conn, mintToKeypair.publicKey, 10 * LAMPORTS_PER_SOL)
      await airdropSol(conn, toKp.publicKey, 10 * LAMPORTS_PER_SOL)

      //toATA 需要付租金 ， fromAta在调用时初始化
      const ata_rent_amount = await splToken.getMinimumBalanceForRentExemptAccount(conn);
      await airdropSol(conn, toAta, ata_rent_amount)
      const tx = await program.methods
        .transferSpl(new anchor.BN(2e9))
        .accounts({
          fromAta: fromAta,
          toAta: toAta,
          from: mintToKeypair.publicKey,
          tokenProgram: splToken.TOKEN_PROGRAM_ID,
        }).signers([mintToKeypair])
        .rpc();

        await confirmAndPrintTxDetails(conn,tx);

        const mintInfo = await getMintInfo(conn,mint);
        printTokenBalance(conn,mintToKeypair.publicKey,mintInfo.decimals)
        printTokenBalance(conn,toKp.publicKey,mintInfo.decimals)
    }catch(e){
      console.log(e)
    }

  })


});///end describle
async function getMintInfo(connection: anchor.web3.Connection, mint: anchor.web3.PublicKey){
  let mintInfo = await splToken.getMint(connection, mint);
  return {
    token_mint : mintInfo.address.toBase58(),
    mintAuthority : mintInfo.mintAuthority?.toBase58(),
    freezeAuthority : mintInfo.freezeAuthority?.toBase58() ,
    supply : mintInfo.supply.toString(),
    decimals : mintInfo.decimals,
    isInitialized : mintInfo.isInitialized,
  }
}

async function printTokenBalance(conn:Connection,ata : PublicKey,decimals:number){
  let account = await splToken.getAccount(conn,ata);
  console.log("address:",ata.toBase58(),"balance:",Number(account.amount) / Math.pow(10,decimals));
}
