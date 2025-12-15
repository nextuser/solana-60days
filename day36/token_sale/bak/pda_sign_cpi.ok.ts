import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CpiNewSigner } from "../target/types/cpi_new_signer";
import { Keypair, LAMPORTS_PER_SOL, PublicKey,SystemProgram } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID,ASSOCIATED_TOKEN_PROGRAM_ID ,createAssociatedTokenAccount,getAccount} from "@solana/spl-token";
import {airdropSol, confirmAndPrintTxDetails} from './util'

describe("token_sale", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.cpiNewSigner as Program<CpiNewSigner>;
  const conn = anchor.getProvider().connection;
  const recipient = Keypair.generate();
  const [pda_pubkey,pda_nonce] =  PublicKey.findProgramAddressSync(
        [Buffer.from("pda_account"),recipient.publicKey.toBuffer()],
        program.programId
    );
 const payer = anchor.getProvider().wallet.payer;

  it("fund pda with sol",async()=>{

    await airdropSol(conn,recipient.publicKey,1 *LAMPORTS_PER_SOL);
    await airdropSol(conn,pda_pubkey,100*LAMPORTS_PER_SOL );


  });

  it("sol transfer with pda signer",async()=>{ 
    const signature = await program.methods.solTransfer(new anchor.BN(1e9)).accounts({
      pdaAccount: pda_pubkey,
      recipient: recipient.publicKey,
      payer: payer.publicKey,
      systemProgram: SystemProgram.programId,
    }).signers([payer]).rpc();
    await confirmAndPrintTxDetails(conn,signature);
  })

});