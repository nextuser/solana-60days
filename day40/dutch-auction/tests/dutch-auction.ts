import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { DutchAuction } from "../target/types/dutch_auction";

import { fromWorkspace,LiteSVMProvider  } from "anchor-litesvm";

import { airdropSol, confirmAndPrintTxDetails,printAccount,printTokenAccount } from "./util";

import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  MINT_SIZE,
  createAssociatedTokenAccountInstruction,
  createInitializeMintInstruction,
  createMintToInstruction,
  getAssociatedTokenAddress,
  getAccount,
  getMint,

} from "@solana/spl-token";




import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";

import { use } from "chai";
import { LiteSVM } from "litesvm";

const useSvm = false;


async function getVersionedTransaction(
  conn : anchor.web3.Connection,
  payer: Keypair,
  instructions : anchor.web3.TransactionInstruction[],
  signers : Keypair[] = [],
) : Promise<anchor.web3.VersionedTransaction>{
    const blockhash = (await conn.getLatestBlockhash("confirmed")).blockhash;
    
    // 4. 创建消息对象
    const message = new anchor.web3.TransactionMessage({
      payerKey: payer.publicKey,
      recentBlockhash: blockhash,
      instructions: instructions,
    }).compileToV0Message();
    
    // 5. 创建VersionedTransaction
    const versionedTx = new anchor.web3.VersionedTransaction(message);
    signers.push(payer);
    versionedTx.sign(signers);
    return versionedTx;
}

describe("dutch-auction", () => {

  let svm = fromWorkspace("./").withDefaultPrograms().withBuiltins().withSysvars().withBlockhashCheck(true);
  const program = anchor.workspace.dutchAuction as Program<DutchAuction>;

  if(useSvm){    
    const provider = new LiteSVMProvider(svm);
    anchor.setProvider(provider);
  } else {
    
    anchor.setProvider(anchor.AnchorProvider.env());
  }

  let conn:any = anchor.getProvider().connection; 
  conn = conn ? conn : svm;
  console.log("connection:",conn);

    // Configure the client to use the local cluster.
  /// anchor.setProvider(anchor.AnchorProvider.env());

  const seller = Keypair.generate();
  const buyer = Keypair.generate();
  const mintKp = Keypair.generate();
  const auctionKeypair = Keypair.generate();
  let sellerAta: PublicKey;
  let buyerAta: PublicKey;
  let vaultAuth: PublicKey;


  before(async() => {
    // Airdrop some SOL to the provider wallet    
    if(useSvm){

      svm.airdrop(seller.publicKey, 10_000_000_000n);
      svm.airdrop(buyer.publicKey, 10_000_000_000n);
    }else{
      await airdropSol(conn,seller.publicKey, 10_000_000_000);
      await airdropSol(conn,buyer.publicKey, 10_000_000_000);
    }
    const lamportsForMint = LAMPORTS_PER_SOL;
    const creatMintIdx = SystemProgram.createAccount({
      fromPubkey: seller.publicKey,
      newAccountPubkey: mintKp.publicKey,
      space: MINT_SIZE,
      lamports: lamportsForMint,
      programId: TOKEN_PROGRAM_ID,
    });
    const mint_authority = seller;
    const initMintIx = createInitializeMintInstruction(
      mintKp.publicKey,
      0,
      mint_authority.publicKey,
      null,
    );
    // const mintTx = new Transaction().add(creatMintIdx, initMintIx);
    // mintTx.recentBlockhash = svm.latestBlockhash();
    // mintTx.feePayer = seller.publicKey;
    // mintTx.sign(seller, mintKp);
    // svm.sendTransaction(mintTx);

    const mintTx = await getVersionedTransaction(
      conn,
      seller,
      [creatMintIdx, initMintIx]  ,[mintKp]);
    let txMint = await conn.sendTransaction(mintTx);
    console.log("1.mint tx",txMint);
    await confirmAndPrintTxDetails(conn, txMint,"1.1 mint signature");

   await printAccount(conn, mintKp.publicKey, "1.2 mint account:");
   const mintInfo = await  getMint(conn, mintKp.publicKey);
   console.log("2. mint info",mintInfo);

    sellerAta = await getAssociatedTokenAddress(
      mintKp.publicKey,
      seller.publicKey
    );

    const createSellerAtaIx = createAssociatedTokenAccountInstruction(
      seller.publicKey,
      sellerAta,
      seller.publicKey,//owner
      mintKp.publicKey
    );
    // const sellerAtaTx = new Transaction().add(createSellerAtaIx);
    // sellerAtaTx.recentBlockhash = svm.latestBlockhash();
    // sellerAtaTx.feePayer = seller.publicKey;
    // sellerAtaTx.sign( seller);
    // let tx1 = svm.sendTransaction(sellerAtaTx);
    console.log("3.mint",mintKp.publicKey.toBase58());
    let sellerAtaTx = await getVersionedTransaction(
      conn,
      seller,
      [createSellerAtaIx]
    );
    let tx1 = await conn.sendTransaction(sellerAtaTx);
    await confirmAndPrintTxDetails(conn, tx1,"\n4.seller ata signature");

    // const tokenAccount = await getAccount(conn, sellerAta);
    // console.log("seller ata",tokenAccount.address.toBase58(), "amount",tokenAccount.amount.toString(),"mint" ,tokenAccount.mint.toBase58());
    // //console.log("seller ata tx",tx1.toString());



    buyerAta = await getAssociatedTokenAddress(
      mintKp.publicKey,
      buyer.publicKey
    );
    const createBuyerAtaIx = createAssociatedTokenAccountInstruction(
      buyer.publicKey,
      buyerAta,
      buyer.publicKey,
      mintKp.publicKey
    );

    // const buyerAtaTx = new Transaction().add(createBuyerAtaIx);
    // buyerAtaTx.recentBlockhash = svm.latestBlockhash();
    // buyerAtaTx.feePayer = buyer.publicKey;
    // buyerAtaTx.sign( buyer);

    const buyerAtaTx = await getVersionedTransaction(
      conn,
      buyer,
      [createBuyerAtaIx]
    );
    let tx2 = await conn.sendTransaction(buyerAtaTx);
    await confirmAndPrintTxDetails(conn, tx2,"\n5.buyer ata signature");

    await printAccount(conn, sellerAta, "seller ata");
    //conn.sendTransaction
    // let tx2 = svm.sendTransaction(buyerAtaTx);
    //console.log("buyer ata tx",tx2.toString());
    // confirmAndPrintTxDetails(conn, tx2);
    let destAta = sellerAta;
    const mintToIx = createMintToInstruction(
      mintKp.publicKey,
      destAta,
      mint_authority.publicKey,
      BigInt(1)
    );

    // const mintToTx = new Transaction().add(mintToIx);
    // mintToTx.recentBlockhash = (await conn.getLatestBlockhash()).blockhash;
    // // svm.latestBlockhash();
    // mintToTx.feePayer = seller.publicKey;
    // mintToTx.sign( seller);
    // let tx3 = await conn.sendTransaction(mintToTx, [ seller]);

    const mintToTx = await getVersionedTransaction(
      conn,
      seller,
      [mintToIx],
      [mint_authority]
    );
    //let tx3 = svm.sendTransaction(mintToTx);
    const sig = await conn.sendTransaction(mintToTx);
    await confirmAndPrintTxDetails(conn, sig,"mint to seller");

    //const tokenAccount = await getAccount(conn, sellerAta);
    //console.log("seller ata",tokenAccount.address.toBase58(), "amount",tokenAccount.amount.toString(),"mint" ,tokenAccount.mint.toBase58());
    await printTokenAccount(conn, sellerAta, "sellerAta after mint:");
    [vaultAuth] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), auctionKeypair.publicKey.toBuffer()],
      program.programId
    );

    const vaultAta = await getAssociatedTokenAddress(
      mintKp.publicKey,
      vaultAuth,
      true  //allowOwnerOffCurve
    );

    printAccount(conn, sellerAta, "sellerAta");


    const startPrice = new anchor.BN(2_000_000_000);
    const floorPrice = new anchor.BN(500_000_000);
    const duration = new anchor.BN(60 * 60 ); // 1 day in seconds

    const sellerAtaAccount = await conn.getAccountInfo(sellerAta);
    console.log("Seller ATA exists:", !!sellerAtaAccount);
    if (sellerAtaAccount) {
      console.log("Seller ATA data length:", sellerAtaAccount.data.length);
      console.log("Seller ATA owner:", sellerAtaAccount.owner.toBase58());
    }

    const tx = await program.methods.initializeAuction(startPrice, floorPrice, duration).accounts({
      auction: auctionKeypair.publicKey,
      seller: seller.publicKey,
      sellerAta: sellerAta,
      mint: mintKp.publicKey,
      vaultAuth: vaultAuth,
      vaultAta: vaultAta,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId,
    }).signers([seller, auctionKeypair]).rpc();

    confirmAndPrintTxDetails(conn, tx);


  });

  it("Is initialized!", async () => {


    //const mint = createMintToInstruction



  //   // Add your test here.
  //   const tx = await program.methods.initializeAuction(startPrice, floorPrice, duration).accounts({
  //     seller: seller.publicKey,
  //     buyer: buyer.publicKey,
  //         pub auction: Account<'info, Auction>,

  //   #[account(mut)]
  //   pub seller : Signer<'info>,

  //   #[account(mut,
  //       associated_token::mint = mint,
  //       associated_token::authority = seller,
  //   )]
  //   seller_ata: Account<'info, TokenAccount>,

  //   #[account(mut)]
  //   pub mint : Account<'info, Mint>,

  //   /// CHECK: vault auth
  //   #[account(
  //       seeds = [b"vault", auction.key().as_ref()],
  //       bump,
  //   )]
  //   pub vault_auth: UncheckedAccount<'info>,
  //   }).rpc();
  //   console.log("Your transaction signature", tx);
   });
});
