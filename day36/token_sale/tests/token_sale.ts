import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TokenSale } from "../target/types/token_sale";
import { Keypair, LAMPORTS_PER_SOL, PublicKey,SystemProgram } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID,ASSOCIATED_TOKEN_PROGRAM_ID ,createAssociatedTokenAccount,getAccount,getMint} from "@solana/spl-token";
import {airdropSol, confirmAndPrintTxDetails, printAccount} from './util'
import { expect } from "chai";

describe("token_sale", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.tokenSale as Program<TokenSale>;
  const conn = anchor.getProvider().connection;
  const provider = anchor.getProvider();
  const adminKp = Keypair.generate();
  const TOKEN_PER_SOL = 100;



  const buyer  = Keypair.generate();
  const adminConfigKp = Keypair.generate();

  let mintPda :anchor.web3.PublicKey;
  let treasuryPda :anchor.web3.PublicKey;
  let buyerAta : anchor.web3.PublicKey;

  [mintPda ] = PublicKey.findProgramAddressSync(
    [Buffer.from("token_mint")],
    program.programId
  );
  [treasuryPda] = PublicKey.findProgramAddressSync(
    [Buffer.from("treasury")],
    program.programId
  );
  console.log("mintPda",mintPda.toBase58());
  console.log("treasuryPda",treasuryPda.toBase58());

  const TOKENS_PER_BUY = 100;
  it("initialize", async () => {
    await airdropSol(conn, adminKp.publicKey, 10 * LAMPORTS_PER_SOL);
    ///await airdropSol(conn, treasury, 1);
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      adminConfig: adminConfigKp.publicKey,
      admin: adminKp.publicKey,
      mint: mintPda,
      treasury: treasuryPda,
      tokenProgram: TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,

    }).signers([adminKp,adminConfigKp]).rpc();

    await confirmAndPrintTxDetails(conn,tx);
    const treasuryAccountInfo = await conn.getAccountInfo(
      treasuryPda
    )

    console.log(`treasuryAccountInfo of ${treasuryPda.toBase58()}:`,treasuryAccountInfo);

    console.log("init transaction signature", tx);
  });

  

  it("buyToken ", async()=>{
        console.log("1.buy token begin:")
        await airdropSol(conn,buyer.publicKey,10*LAMPORTS_PER_SOL );
        await printAccount(conn,buyer.publicKey,"2.after airdrop buyer");
        await printAccount(conn,mintPda,"3.mintPda");

        const mintInfo = await getMint(conn, mintPda);
        console.log("3.5 mingInfo:",mintInfo);
        
        console.log("4.begin create pda count n:")
        //await airdropSol(conn,payer.publicKey,1);
        //购买者创建账号
        buyerAta = await createAssociatedTokenAccount(
            conn,
            buyer,
            mintPda,
            buyer.publicKey,
            undefined,
            TOKEN_PROGRAM_ID,
        );

        await printAccount(conn,buyerAta,"5.after create buyerAta");
        /**
         *     
        pub buyer : Signer<'info>,
        pub mint : Account<'info, Mint>,

        pub buyer_ata : Account<'info, TokenAccount>,
        /// CHECK:
        pub treasury : AccountInfo<'info>,
        pub token_program : Program<'info, Token>,
        pub system_program : Program<'info, System>,
    */
        
        await printAccount(conn,treasuryPda,"6.after airdrop treasury");
        const solToSend = new anchor.BN(LAMPORTS_PER_SOL);
        const expectedAmount = Number(solToSend) * TOKEN_PER_SOL;

        let buyerAtaAccount = await getAccount(conn, buyerAta);
        console.log("7.1 before mint token amount ",buyerAtaAccount.amount);
        
        let treasuryBalance = (await conn.getBalance(treasuryPda));
        console.log("7.2 before transfer treasury balance", treasuryBalance);
        await printAccount(conn,treasuryPda,"7.3 before transfer treasury meta");
        const tx = await program.methods.mint(solToSend).accounts({
          buyer: buyer.publicKey,
          buyerAta: buyerAta,
          mint: mintPda,
          admin: adminKp.publicKey,
          treasury: treasuryPda,
          tokenProgram: TOKEN_PROGRAM_ID,
          systemProgram: SystemProgram.programId,

        }).signers([buyer]).rpc();
        await confirmAndPrintTxDetails(conn,tx);

        buyerAtaAccount = await getAccount(conn, buyerAta);
        console.log("7.2 after mint token amount ",buyerAtaAccount.amount);
        console.log("mint signature", tx);
        
        treasuryBalance = (await conn.getBalance(treasuryPda));
        console.log("8.2 after transfer treasury balance", treasuryBalance);
        printAccount(conn,treasuryPda,"8.3.after transfer treasury meta ");

  });




 it("withdraw" , async function () { 
     printAccount(conn,treasuryPda,"before withdraw, treasury meta:");
     const prevBalance = await conn.getBalance(treasuryPda);
     const prevAdminBalance = await conn.getBalance(adminKp.publicKey);
     const withdrawAmount = LAMPORTS_PER_SOL;
     const tx = await  program.methods.withdraw(new anchor.BN(withdrawAmount)).accounts({
       admin: adminKp.publicKey,
       adminConfig : adminConfigKp.publicKey,
       treasury: treasuryPda,
       systemProgram: SystemProgram.programId,
     }).signers([adminKp]).rpc();

     await confirmAndPrintTxDetails(conn,tx);
     const afterBalance = await conn.getBalance(treasuryPda);
     expect(prevBalance).to.equal(afterBalance+withdrawAmount);
     const afterAdminBalance = await conn.getBalance(adminKp.publicKey);
     expect(prevAdminBalance).to.equal(afterAdminBalance-withdrawAmount);
     console.log("admin balance change ",prevAdminBalance, "change to ", afterAdminBalance);
     printAccount(conn,treasuryPda,"after withdraw, treasury meta:");
   })
});