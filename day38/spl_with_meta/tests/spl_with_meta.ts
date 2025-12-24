import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SplWithMeta } from "../target/types/spl_with_meta";
import { MPL_TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata";
import { TOKEN_PROGRAM_ID ,createMint} from "@solana/spl-token";
import {confirmAndPrintTxDetails,printAccount} from "./util"
import { assert } from "chai";
import fs from 'fs';
import path from 'path'
import {
  Metaplex,
  irysStorage,
  keypairIdentity,
  toMetaplexFile,
} from '@metaplex-foundation/js';

describe("spl_with_meta", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const conn = anchor.getProvider().connection;
  const program = anchor.workspace.splWithMeta as Program<SplWithMeta>;
  const payer = anchor.getProvider().wallet.payer;
  const mintAuthority = payer.publicKey;
  const freezeAuthority = payer.publicKey;
  const decimals = 4;
  const METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );  

  // metaplex 上傳需要付費，因此需要鏈接devenet，payer 可以通過空投獲得sol  [faucet](https://faucet.solana.com/)
  const metaplex = Metaplex.make(conn)
  .use(keypairIdentity(payer))
  .use(irysStorage({
      address: "https://devnet.irys.xyz",
      providerUrl: anchor.getProvider().connection.rpcEndpoint,// 鏈接devenet
      timeout: 60_000,
  }));

  it("spl token with meta !", async () => {
    console.log("1. create mint"  );
    const mint = await createMint(
    conn,
    payer,
    mintAuthority,
    freezeAuthority,
    decimals);    
    const [metadataPDA] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("metadata"), METADATA_PROGRAM_ID.toBuffer(), mint.toBuffer()],
      METADATA_PROGRAM_ID
    );
    console.log("2. find pda",metadataPDA, "mint:",mint.toBase58()  );
    const url = path.resolve(__dirname, "../assets/image/kitten.png");
    console.log("3.local image url:",url);
    const imageBuffer = fs.readFileSync(url);
    const metaplexFile = toMetaplexFile(imageBuffer, "kitten.png");
    console.log("4.upload image to irys...",metaplexFile);
    const arweaveImageUri = await metaplex.storage().upload(metaplexFile);
    const imageTxId = arweaveImageUri.split("/").pop();
    const imageUri = `https://devnet.irys.xyz/${imageTxId}`;

 ;

    console.log("5.devenet irys image url:",imageUri);
    const metadata = {
      name: "yuandatou",
      symbol: "YDT",
      image: imageUri,
      description: 'Yuandatou First president of China',
      isMutable: true,
    };
    console.log("6.upload meta json to irys...",metadata);
    const arweaveMetadataUri : string = await metaplex
      .storage()
      .uploadJson(metadata);
      
    const metaTxId = arweaveMetadataUri.split("/").pop();
    const metadataUri = `https://devnet.irys.xyz/${metaTxId}`
    console.log("7.devnet iry metadata url :",metadataUri);

    const metaAuthority = mintAuthority
    console.log("8. before createTokenMetadata");
    const fee_rate = 100;
    const mutable  = true;
    // Add your test here.
    const tx = await program.methods.createTokenMetadata(
      metadata.name,
      metadata.symbol,
      metadataUri,
      fee_rate, //fee rate 1%
      mutable,
    ).accounts({
      metadata: metadataPDA,
      mint: mint,
      authority: metaAuthority,
      payer:payer.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
      tokenMetadataProgram: METADATA_PROGRAM_ID,
    }).signers([payer]).rpc();
    await confirmAndPrintTxDetails(conn, tx);
    console.log("8. Your transaction signature", tx);
    const info = await conn.getAccountInfo(metadataPDA);
    console.log("9. metadata account info:",info);
    assert(info.owner.equals(METADATA_PROGRAM_ID),"wrong program ownder , for metadata account");
  });
});
