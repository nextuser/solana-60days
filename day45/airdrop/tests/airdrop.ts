import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { airdropSol, confirmAndPrintTxDetails } from "./util";
import * as nacl  from "tweetnacl";
import {Ed25519Program,Transaction} from "@solana/web3.js";
import { expect } from "chai";
import { Airdrop } from "../target/types/airdrop";

function createEd25519Instruction(
  distributor: anchor.web3.Keypair,
  recipient: anchor.web3.PublicKey,
  amount : number
){
  const message = Buffer.alloc(40);;
  recipient.toBuffer().copy(message, 0);
  message.writeBigUInt64LE(BigInt(amount),32);

  const signature = nacl.sign.detached(message, distributor.secretKey);
  return Ed25519Program.createInstructionWithPublicKey({
    publicKey: distributor.publicKey.toBytes(),
    message: message,
    signature: signature,
  })
}
describe("airdrop", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.airdrop as Program<Airdrop>;
  const provider = anchor.getProvider();

  const distributor = anchor.web3.Keypair.generate();
  const recipient = anchor.web3.Keypair.generate();
  const invalidDestination = anchor.web3.Keypair.generate();
  const conn = provider.connection;
  before(async () => {
    await airdropSol(conn, recipient.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);
    await airdropSol(conn, distributor.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);
    await airdropSol(conn, invalidDestination.publicKey, 1 * anchor.web3.LAMPORTS_PER_SOL);
  })
  it("sucessfully claim airdrop", async () => {
    const claimAmount = 1000_000;
    const edInstruction = createEd25519Instruction(
      distributor,
      recipient.publicKey,
      claimAmount);
    // Add your test here.
    console.log("distributor: ", distributor.publicKey.toBase58());
    const claimInstruction = await program.methods.claim().accounts({
      expectedDistributor: distributor.publicKey,
      recipient: recipient.publicKey,
      instructionsysvar: anchor.web3.SYSVAR_INSTRUCTIONS_PUBKEY,

    }).instruction();

    const tx = new Transaction().add(
      edInstruction,
      claimInstruction
    );
    const signature = await provider.sendAndConfirm(tx, [recipient]);
    confirmAndPrintTxDetails(conn,signature,"confirmed","Claim");
    //expect(result).to.not.be.empty;
  });
});
