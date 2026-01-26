import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";

describe("vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.vault as Program<Vault>;
  const program_id = new anchor.web3.PublicKey("22222222222222222222222222222222222222222222");
  //let alice = anchor.web3.Keypair.generate();
  let alice = anchor.getProvider().wallet?.payer;
  const connection = anchor.getProvider().connection;
  console.log("program_id",program.programId.toBase58());


  it("deposit !", async () => {
    
    const [pda,bump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("vault"),
        alice.publicKey.toBuffer(),
      ],
      program.programId
    );  
    const sig = await program.methods.deposit(new anchor.BN(1000_000)).accounts(
      {
        signer: alice.publicKey,
        vault: pda,
        systemProgram: anchor.web3.SystemProgram.programId,
      }
    ).signers([alice]).rpc();
    
  });

  it("withdraw !", async () => {
    await connection.requestAirdrop(alice.publicKey, 1e9);
    const [pda,bump] = await anchor.web3.PublicKey.findProgramAddress(
      [
        Buffer.from("vault"),
        alice.publicKey.toBuffer(),
      ],
      program.programId
    );  
    
    // Add your test here.
    const tx = await program.methods.withdraw().accounts({
      signer: alice.publicKey,
      vault: pda,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).signers([alice]).rpc();
    console.log("Your transaction signature", tx);
  });
});
