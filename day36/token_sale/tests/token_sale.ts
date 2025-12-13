import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenSale } from "../target/types/token_sale";
import { Keypair, PublicKey,SystemProgram } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID,ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import {confirmAndPrintTxDetails} from './util'

describe("token_sale", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.tokenSale as Program<TokenSale>;
  const conn = anchor.getProvider().connection;
  const provider = anchor.getProvider();
  const adminKp = provider.wallet.payer;
  const buyer  = Keypair.generate();
  const adminConfig = Keypair.generate();
  const [mint,_bump ] = PublicKey.findProgramAddressSync(
    [Buffer.from("token_mint")],
    program.programId
  );
  const [treasury,_bump2 ] = PublicKey.findProgramAddressSync(
    [Buffer.from("treasury")],
    program.programId
  );

  const TOKENS_PER_BUY = 100;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      adminConfig: adminConfig.publicKey,
      admin: adminKp.publicKey,
      mint: mint,
      treasury: treasury,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
      


    }).signers([adminKp,adminConfig]).rpc();

    await confirmAndPrintTxDetails(conn,tx);

    console.log("Your transaction signature", tx);
  });
});
