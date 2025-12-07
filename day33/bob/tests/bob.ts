import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Bob } from "../target/types/bob";
import {Alice } from "../target/types/alice"
import {getTransactionDetials,airdropSol,generateKeypairWithSol, confirmAndPrintTxDetails} from "./util"
describe("bob", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.bob as Program<Bob>;
  const provider = anchor.getProvider();
  const conn = provider.connection;

  it("bob Is initialized!", async () => {
    const dataKey = anchor.web3.Keypair.generate();
    /**
     * pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + size_of::<BobData>())]
    pub bob_data_account: Account<'info, BobData>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
     */
    const tx = await program.methods.initialize().accounts({
      bobDataAccount: dataKey.publicKey,
      user: provider.wallet.publicKey,
      //systemProgram:  anchor.web3.SystemProgram.programId
    }).signers([dataKey,anchor.getProvider().wallet?.payer]).rpc();

    await confirmAndPrintTxDetails(conn,tx);
    console.log("Your transaction signature", tx);
  });


  const program_alice = anchor.workspace.alice as Program<Alice>;

});

