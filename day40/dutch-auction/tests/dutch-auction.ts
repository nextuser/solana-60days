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

type BN = anchor.BN;
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import { use } from "chai";
import { Clock } from "litesvm";
import { expect } from "chai";

type LiteSVM = ReturnType<typeof fromWorkspace> 

function sendTransaction(svm : LiteSVM, instructions : anchor.web3.TransactionInstruction[], signers:Keypair[]){
  let tx = new Transaction();
  for(let ix of instructions){
    tx = tx.add(ix);
  };
  tx.recentBlockhash = svm.latestBlockhash();
  tx.feePayer = signers[0].publicKey;
  tx.sign(...signers);
  return svm.sendTransaction(tx);

}




describe("dutch-auction", () => {

  let svm = fromWorkspace("./").withDefaultPrograms().withBuiltins().withSysvars().withBlockhashCheck(true);
  
    const provider = new LiteSVMProvider(svm);
    anchor.setProvider(provider);

  const program = anchor.workspace.dutchAuction as Program<DutchAuction>;

  const seller = Keypair.generate();
  const buyer = Keypair.generate();
  const mintKp = Keypair.generate();
  const auctionKeypair = Keypair.generate();
  let sellerAta: PublicKey;
  let buyerAta: PublicKey;
  let vaultAuth: PublicKey;
  let vaultAta : PublicKey;


  const startPrice = new anchor.BN(2_000_000_000);
  const floorPrice = new anchor.BN(500_000_000);
  const duration = new anchor.BN(60 * 60 ); // 1 day in seconds


  before(async() => {


    svm.airdrop(seller.publicKey, 10_000_000_000n);
    svm.airdrop(buyer.publicKey, 10_000_000_000n);

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

    sendTransaction(svm, [creatMintIdx, initMintIx], [seller, mintKp]);

   const mintInfo = await  svm.getAccount(mintKp.publicKey);
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

    console.log("3.mint",mintKp.publicKey.toBase58());

    sendTransaction(svm, [createSellerAtaIx], [seller]);


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


    sendTransaction(svm, [createBuyerAtaIx], [buyer]);

    let destAta = sellerAta;
    const mintToIx = createMintToInstruction(
      mintKp.publicKey,
      destAta,
      mint_authority.publicKey,
      BigInt(1)
    );

    sendTransaction(svm, [mintToIx], [ seller, mint_authority]);

    [vaultAuth] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault"), auctionKeypair.publicKey.toBuffer()],
      program.programId
    );

    vaultAta = await getAssociatedTokenAddress(
      mintKp.publicKey,
      vaultAuth,
      true  //allowOwnerOffCurve
    );

    //printAccount(conn, sellerAta, "sellerAta");
    console.log("sellerAta account info ",svm.getAccount(sellerAta));


    const sellerAtaAccount = await svm.getAccount(sellerAta);
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

  });

  it("execute buy at 25% time with expected price ", async () => {
      const auction = await program.account.auction.fetch(auctionKeypair.publicKey);  
      const startTime = auction.startingTime.toNumber();
      const duration = auction.duration.toNumber();
      const quarterTime = startTime + duration / 4;
      const c = svm.getClock();
      svm.setClock(new Clock(c.slot,
        c.epochStartTimestamp,
        c.epoch,
        c.leaderScheduleEpoch,
        BigInt(quarterTime)))
      const balanceBefore = svm.getBalance(buyer.publicKey) || 0n;
      console.log("buyer balance before: ", Number(balanceBefore)/LAMPORTS_PER_SOL);
      console.log("auction",auctionKeypair.publicKey.toBase58(), 
                  "\nbuyer",buyer.publicKey.toBase58(), 
                  "\nseller",seller.publicKey.toBase58(),
                  "\nbuyer ata",buyerAta.toBase58(),
                  "\nvault auth",vaultAuth.toBase58());
      const tx = await program.methods.buy().accounts({
          buyer: buyer.publicKey,
          seller: seller.publicKey,
          auction: auctionKeypair.publicKey,
          buyerAta: buyerAta,
          vaultAuth: vaultAuth,
          vaultAta: vaultAta,
          tokenProgram: TOKEN_PROGRAM_ID,
        }).signers([buyer]).rpc();
      
      const balanceAfter = svm.getBalance(buyer.publicKey) || 0n;
      console.log("buyer balance after: ", Number(balanceAfter )/LAMPORTS_PER_SOL);
      const pricePaid = Number(balanceBefore) - Number(balanceAfter );
      console.log("pricePaid:",pricePaid);
      const expectedPaid = Number(startPrice) - (Number(startPrice) - Number(floorPrice)) * 0.25;
      console.log("expectedPaid:",expectedPaid);
      expect(pricePaid).to.equals(expectedPaid);

    });
});
