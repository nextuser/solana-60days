import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import * as splToken from "@solana/spl-token";
import {SplToken } from "../target/types/spl_token";

describe("spl_token", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.splToken as Program<SplToken>;

  const provider = anchor.getProvider();
  const signerKp = provider.wallet.payer;
  const toKp = anchor.web3.Keypair.generate();
  const tokenName = "Rmb";
  const [mint ,_bump] = PublicKey.findProgramAddressSync(
    [signerKp.publicKey.toBuffer(),Buffer.from(tokenName)],
    program.programId
  );
  const ata = splToken.getAssociatedTokenAddressSync(
    mint,
    signerKp.publicKey,
    false);
  

  it("Is initialized!", async () => {
    
    // Add your test here.
    const tx = await program.methods.createAndMintToken("Rmb")
      .accounts({
        signer: signerKp.publicKey,
        tokenProgram: splToken.TOKEN_PROGRAM_ID,
        associatedTokenProgram: splToken.ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: anchor.web3.SystemProgram.programId,
        newMint: mint,
        newAta: ata,
      })
      .rpc();
    console.log("Your transaction signature", tx);
    console.log("Mint: ", mint.toBase58());
    console.log("ATA: ", ata.toBase58());
    console.log("Mint Info: ", await getMintInfo(provider.connection, mint));
  });
});

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
