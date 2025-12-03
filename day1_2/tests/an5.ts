import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import type { An5 } from "../target/t/types/an5.ts";
//import { Program, BN } from "@coral-xyz/anchor"; 
import pkg from "@coral-xyz/anchor";
const { BN } = pkg;

describe("an5", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.an5 as Program<An5>;
  

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize(new BN(123), new BN(456),"tail information").rpc();
    console.log("Your transaction signature", tx);
  });

    it("test array!", async () => {
    // Add your test here.
    const tx = await program.methods.array(["abc","def","hhh"]).rpc();
    console.log("Your transaction signature", tx);
  });
      it("test add!", async () => {
    // Add your test here.
    const tx = await program.methods.add(new BN(100),new BN(50)).rpc();
    console.log("Your transaction signature", tx);
  });

  // it("test add overflow", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.add(100,200).rpc();
  //   console.log("Your transaction signature", tx);
  // });

  
  it("test cbrt", async () => {
    // Add your test here.
    const tx = await program.methods.cbrt(27.0).rpc();
    console.log("Your transaction signature", tx);
  });

    it("test overflow", async () => {
    // lib.rs snake_case => camel case
    const tx = await program.methods.testOverflow().rpc();
    console.log("Your transaction signature", tx);
  });


  //  it("test overflow!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.test_overflow().rpc();
  //   console.log("Your transaction signature", tx);
  // });

  // it("m3!", async () => {
  //   // Add your test here.
  //   const tx = await program.methods.add().rpc();
  //   console.log("Your transaction signature", tx);
  // });
});
