import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CpiNewSigner } from "../target/types/pda_by_program";
import { Keypair, LAMPORTS_PER_SOL, PublicKey,SystemProgram } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID,ASSOCIATED_TOKEN_PROGRAM_ID ,createAssociatedTokenAccount,getAccount} from "@solana/spl-token";
import {airdropSol, confirmAndPrintTxDetails} from './util'

describe("pda sign cpi", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.pdaByProgram as Program<PdaByProgram>;
  const conn = anchor.getProvider().connection;
  const recipient = Keypair.generate();
  const [pda_pubkey,pda_nonce] =  PublicKey.findProgramAddressSync(
        [Buffer.from("pda_account")],
        program.programId
    );
 const payer = anchor.getProvider().wallet.payer;
 

  it("fund pda with sol",async()=>{

    await airdropSol(conn,recipient.publicKey,1 *LAMPORTS_PER_SOL);

    const signature = await program.methods.initialize().accounts({
      pdaAccount: pda_pubkey,
      payer: payer.publicKey,
      systemProgram: SystemProgram.programId
    }).signers([payer]).rpc();
    await confirmAndPrintTxDetails(conn,signature,'confirmed',"after initialize");
    console.log("programId"  ,program.programId.toBase58())
    let accountInfo = await conn.getAccountInfo(pda_pubkey);
    console.log("pda account owner",accountInfo.owner.toBase58(), "lamports",accountInfo.lamports);
    await airdropSol(conn,pda_pubkey,100*LAMPORTS_PER_SOL );
    accountInfo = await conn.getAccountInfo(pda_pubkey);
    console.log("pda account afeter airdrop lamports",accountInfo.lamports);
    console.log("account info :",accountInfo);

  });

  it("sol transfer with pda signer",async()=>{ 
    const signature = await program.methods.solTransfer(new anchor.BN(1e9)).accounts({
      pdaAccount: pda_pubkey,
      recipient: recipient.publicKey,
      payer: payer.publicKey,
      systemProgram: SystemProgram.programId,
    }).signers([payer]).rpc();
    await confirmAndPrintTxDetails(conn,signature);

    const pda_info = await conn.getAccountInfo(pda_pubkey);
    console.log("pda account info",pda_info);
  })

});