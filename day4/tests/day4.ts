import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day4 } from "../target/types/day4";
import { assert } from "chai";

describe("day4", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day4 as Program<Day4>;


  it("initialize",async ()=>{
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("initialize signature", tx);
  })

  it("day4 limit range small!", async () => {
    // 转换snake_case => camelCase
    try{
      const tx = await program.methods.limitRange(new anchor.BN(5)).rpc();
      console.log("Your transaction signature", tx);
    } catch(e) {
      assert.equal(e.error.errorMessage, "a is too small");
      assert.equal(e.error.errorCode.code, "AisTooSmall");
    }
  });
    it("day4 limit range large!", async () => {
    // Add your test here.
    try{
      const tx = await program.methods.limitRange(new anchor.BN(105)).rpc();
    } catch (e) {
      assert.isTrue(e instanceof anchor.AnchorError);
      assert.equal(e.error.errorMessage, "a is big");
      console.log("limit range too big  is :",e);
    }
   
  });
});
