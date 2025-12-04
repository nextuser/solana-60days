import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day11 } from "../target/types/day11";
import {getTransactionAnalysis} from './util'

describe("day11", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const connection  = anchor.getProvider().connection;
  const program = anchor.workspace.day11 as Program<Day11>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });

  it("week day",async()=>{ 
    const tx = await program.methods.getDayOfTheWeek().rpc();
    console.log("Your transaction signature", tx);
  })

  it("blockhash",async()=>{
    const tx = await program.methods.getBlockhash().accounts({"instructionSysvar": anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY}).rpc();
    console.log("Your transaction signature", tx);
    console.log("blockhash address", anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY.toBase58());
    console.log("solana account ",anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY.toBase58());
    const detail = await getTransactionAnalysis(connection,tx);
    console.log("tranaciton detail :",detail);
  })


    it("sys vars",async()=>{
        try {
            console.log('stake history account :', anchor.web3.SYSVAR_STAKE_HISTORY_PUBKEY);
            const tx = await program.methods.getSysVars().accounts(
                {
                    "blockHashVar": anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY,
                    "stakeHistoryVar": anchor.web3.SYSVAR_STAKE_HISTORY_PUBKEY
                }).rpc();
            console.log("Your transaction signature", tx);
        }catch(e){
            console.log("getSysVars error:",e);
        }
    // console.log("blockhash address", anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY.toBase58());
    // console.log("solana account ",anchor.web3.SYSVAR_RECENT_BLOCKHASHES_PUBKEY.toBase58());
  })


});
