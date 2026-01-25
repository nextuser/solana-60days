import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorEscrow } from "../target/types/anchor_escrow";
import { ConfirmOptions, Keypair, PublicKey } from "@solana/web3.js";
import { Mint,getOrCreateAssociatedTokenAccount, getAssociatedTokenAddress ,createMint, ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, createAssociatedTokenAccount, mintTo} from '@solana/spl-token';
async function confirmTransaction(connection: anchor.web3.Connection, signature: string){
    const Block = await connection.getLatestBlockhash();
    await  connection.confirmTransaction(
      {signature,
        blockhash: Block.blockhash,
        lastValidBlockHeight: Block.lastValidBlockHeight
      },  
      "confirmed"
      
    );
}
async function airdrop(connection: anchor.web3.Connection, pubkey: anchor.web3.PublicKey){
    let signature = await connection.requestAirdrop(pubkey, 10_000_000_000);
    await confirmTransaction(connection, signature);
    
}

describe("anchor_escrow", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.anchorEscrow as Program<AnchorEscrow>;
  const payer = anchor.getProvider().wallet.payer;
  const maker = anchor.web3.Keypair.generate();
  const taker = anchor.web3.Keypair.generate();
  const connection = anchor.getProvider().connection;
  let mintA = Keypair.generate();
  let mintB = Keypair.generate();
  let vault : anchor.web3.PublicKey;
  let makerAtaA : anchor.web3.PublicKey;
  let takerAtaB : anchor.web3.PublicKey;
  const seed = 1;
  const amount = (1000_000);
  const receive =(2000_000) ;

  const [escrow, bump] =  anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("escrow"),
        maker.publicKey.toBuffer(),
        new anchor.BN(seed).toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );
  const confirm_option : ConfirmOptions = {commitment:"confirmed"}
  before(async () => {
    console.log("wait to create mint ")
     await createMint(connection, payer, payer.publicKey, payer.publicKey, 6,mintA,confirm_option,TOKEN_PROGRAM_ID);
     await createMint(connection, payer, payer.publicKey, payer.publicKey, 6,mintB,confirm_option,TOKEN_PROGRAM_ID);
    console.log("mint created");
    await airdrop(connection, maker.publicKey);
    await airdrop(connection, taker.publicKey);
    await airdrop(connection, payer.publicKey);

    console.log("air drop ok");


    let init_balance = await connection.getBalance(maker.publicKey);
    makerAtaA = await createAssociatedTokenAccount(connection,payer,mintA.publicKey, maker.publicKey,confirm_option);
    takerAtaB = await createAssociatedTokenAccount(connection,payer,mintB.publicKey, taker.publicKey,confirm_option);
    console.log("ata created for maker")
    vault = await getAssociatedTokenAddress(mintA.publicKey, escrow, true, TOKEN_PROGRAM_ID);

    await mintTo(connection,payer,mintA.publicKey,makerAtaA,payer.publicKey,amount,[payer],confirm_option);
    console.log("mint tokenA to maker");
    await mintTo(connection,payer,mintB.publicKey,takerAtaB,payer.publicKey,receive,[payer],confirm_option)
    console.log("mint tokenB to taker");
    // let vault = await createAssociatedTokenAccount(connection,payer,mintA.publicKey,escrow,confirm_option);
    // console.log("ata created for vault");
  });
  it("test make an refund!", async () => {


    const signature = await program.methods.make(new anchor.BN(seed), new anchor.BN(receive), new anchor.BN(amount)).accounts({
      maker: maker.publicKey,
      escrow: escrow,
      mintA: mintA.publicKey,
      mintB: mintB.publicKey,
      makerAtaA: makerAtaA,
      vault: vault,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
    .signers([maker])
    .rpc();
    await confirmTransaction(connection, signature);
    const meta = await connection.getParsedTransaction(signature, confirm_option);
    console.log(meta.meta);

    
  });
});
